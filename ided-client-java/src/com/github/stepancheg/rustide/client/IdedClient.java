package com.github.stepancheg.rustide.client;

import com.github.stepancheg.rustide.client.proto.IdedProtos;
import com.google.protobuf.CodedInputStream;
import org.jetbrains.annotations.Nullable;

import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStream;
import java.io.InputStreamReader;
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
                    makeError(new RuntimeException("got error from ided: " + line));
                }
            } catch (Throwable e) {
                makeError(e);
            }
        }
    }

    public IdedClient() {
        boolean ok = false;
        try {
            ProcessBuilder processBuilder = new ProcessBuilder();
            processBuilder.command("../ided/ided");
            process = processBuilder.start();

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
    
    private IdedProtos.Response request(IdedProtos.Request request) {
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

    public IdedProtos.Response.Ping ping() {
        IdedProtos.Request.Builder builder = IdedProtos.Request.newBuilder();
        builder.getPingBuilder();
        IdedProtos.Response response = request(builder.build());
        if (!response.hasPing()) {
            throw new IllegalStateException("must have ping");
        }
        return response.getPing();
    }

    public void unknownCommand() {
        IdedProtos.Request.Builder builder = IdedProtos.Request.newBuilder();
        builder.getUnknownCommand();
        request(builder.build());
    }

    public void close() {
        process.destroy();
    }
}
