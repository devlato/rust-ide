package com.github.stepancheg.rustide.client.perftest;

import com.github.stepancheg.rustide.client.IdedClient;
import com.github.stepancheg.rustide.client.proto.IdedProtos;
import org.junit.Assert;

/**
 * @author Stepan Koltsov
 */
public class AnalyzeTrivialPerftest {

    public static void main(String[] args) {
        final IdedClient client = new IdedClient();

        StringBuilder programBuilder = new StringBuilder();
        for (int i = 0; i < 0; ++i) {
            programBuilder.append("fn foo" + i + "() {}\n");
        }
        programBuilder.append("fn main() {}\n");
        final String program = programBuilder.toString();

        Perftest.run(new Runnable() {
            @Override
            public void run() {
                IdedProtos.Request.Analyze.Builder req = IdedProtos.Request.Analyze.newBuilder();
                req.getFileBuilder().setName("main.rs");
                req.getFileBuilder().setContent(program);
                IdedProtos.Response.Analyze resp = client.analyze(req.build());
                Assert.assertTrue(resp.getErrorsCount() == 0);
                Assert.assertTrue(resp.getAst().getMarkersCount() > 0);
            }
        });
    }

}
