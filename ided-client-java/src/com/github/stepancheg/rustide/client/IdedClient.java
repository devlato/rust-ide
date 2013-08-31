package com.github.stepancheg.rustide.client;

import com.github.stepancheg.rustide.client.proto.IdedProtos;
import com.github.stepancheg.rustide.client.util.ProtobufUnion;
import com.google.protobuf.Message;
import org.jetbrains.annotations.Nullable;

import java.io.*;
import java.nio.charset.Charset;
import java.util.concurrent.locks.Condition;
import java.util.concurrent.locks.Lock;
import java.util.concurrent.locks.ReentrantLock;

/**
 * @author Stepan Koltsov
 */
public class IdedClient {

    private final Process process;

    private final Thread stdoutReader;
    private final Thread stderrReader;

    private final Lock lock = new ReentrantLock();
    private final Condition condition = lock.newCondition();

    @Nullable
    private Throwable error;
    @Nullable
    private IdedProtos.Response response;

    private class StdoutReader implements Runnable {
        private final InputStream inputStream;

        private StdoutReader(InputStream inputStream) {
            this.inputStream = inputStream;
        }

        @Override
        public void run() {
            try {
                for (;;) {
                    IdedProtos.Response response = IdedProtos.Response.parseDelimitedFrom(inputStream);
                    lock.lock();
                    try {
                        if (response == null) {
                            throw new IllegalStateException("unexpected EOF on stdout");
                        }

                        if (IdedClient.this.response != null) {
                            throw new IllegalStateException("still have previous response");
                        }

                        IdedClient.this.response = response;

                        condition.signalAll();
                    } finally {
                        lock.unlock();
                    }
                }
            } catch (Throwable e) {
                makeError(e);
            }
        }
    }

    private void makeError(Throwable e) {
        lock.lock();
        if (error == null) {
            error = e;
        }
        try {
            condition.signalAll();
        } finally {
            lock.unlock();
        }
    }

    private class StderrReader implements Runnable {
        private final InputStream inputStream;
        private final BufferedReader reader;

        private StderrReader(InputStream inputStream) {
            this.inputStream = inputStream;
            this.reader = new BufferedReader(new InputStreamReader(inputStream, Charset.forName("utf-8")));
        }

        @Override
        public void run() {
            try {
                String line = reader.readLine();
                if (line != null) {
                    // hack: task are executed in try, but inside try they write to stderr
                    // need a way to disable printing
                    if (!line.matches(".*task.*failed at.*libsyntax/diagnostic.rs:.*")) {
                        makeError(new RuntimeException("got error from ided: " + line));
                    }
                }
            } catch (Throwable e) {
                makeError(e);
            }
        }
    }

    public IdedClient() {
        boolean ok = false;
        try {
            ProcessBuilder builder = new ProcessBuilder();
            Paths paths = new Paths();
            builder.environment().put("RUST_HOME", paths.getRustHome().getPath());
            builder.command(paths.getIded().getPath());
            process = builder.start();

            stdoutReader = new Thread(new StdoutReader(process.getInputStream()));
            stderrReader = new Thread(new StderrReader(process.getErrorStream()));

            stdoutReader.start();
            stderrReader.start();

            ok = true;
        } catch (IOException e) {
            throw new RuntimeException(e);
        } finally {
            if (!ok) {
                close();
            }
        }
    }

    private IdedProtos.Response processRequest(IdedProtos.Request request) {
        try {
            request.writeDelimitedTo(process.getOutputStream());
            process.getOutputStream().flush();

            lock.lock();
            try {
                for (;;) {
                    if (error != null) {
                        throw new RuntimeException(error);
                    }

                    if (response != null) {
                        IdedProtos.Response response = this.response;
                        this.response = null;
                        return response;
                    }

                    condition.await();
                }
            } finally {
                lock.unlock();
            }

        } catch (InterruptedException e) {
            throw new RuntimeException(e);
        } catch (IOException e) {
            throw new RuntimeException(e);
        }
    }

    private <T extends Message> T request(IdedProtos.Request request, Class<T> responseType) {
        return ProtobufUnion.unionMember(processRequest(request), responseType);
    }

    public IdedProtos.Response.Ping ping() {
        IdedProtos.Request.Builder builder = IdedProtos.Request.newBuilder();
        builder.getPingBuilder();
        return request(builder.build(), IdedProtos.Response.Ping.class);
    }

    public void unknownCommand() {
        IdedProtos.Request.Builder builder = IdedProtos.Request.newBuilder();
        builder.getUnknownCommand();
        processRequest(builder.build());
    }

    public IdedProtos.Response.Analyze analyze(IdedProtos.Request.Analyze analyze) {
        IdedProtos.Request.Builder builder = IdedProtos.Request.newBuilder();
        builder.setAnalyze(analyze);
        return request(builder.build(), IdedProtos.Response.Analyze.class);
    }

    public void close() {
        if (process != null) {
            process.destroy();
        }
    }
}
