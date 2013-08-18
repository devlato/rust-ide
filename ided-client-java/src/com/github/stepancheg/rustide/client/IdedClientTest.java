package com.github.stepancheg.rustide.client;

import com.github.stepancheg.rustide.client.proto.IdedProtos;
import org.junit.After;
import org.junit.Assert;
import org.junit.Before;
import org.junit.Test;

/**
 * @author Stepan Koltsov
 */
public class IdedClientTest {

    private IdedClient client;

    @Before
    public void up() {
        client = new IdedClient();
    }

    @After
    public void down() {
        if (client != null) {
            client.close();
        }
    }

    @Test
    public void ping() {
        client.ping();
    }

    @Test
    public void unknownCommand() {
        try {
            client.unknownCommand();
            Assert.fail();
        } catch (RuntimeException e) {
            // expected
        }
    }

    private IdedProtos.Response.Analyze analyze(String content) {
        IdedProtos.Request.Analyze.Builder request = IdedProtos.Request.Analyze.newBuilder();
        IdedProtos.File.Builder file = request.getFileBuilder();
        file.setName("foo.rs");
        file.setContent(content);
        return client.analyze(request.build());
    }

    @Test
    public void analyzeSimple() {
        IdedProtos.Response.Analyze response = analyze("fn main() {\nlet x = 1;}\n");
        Ast ast = Ast.fromLazy(response.getAst());
    }

    @Test
    public void analyzeLexerError() {
        IdedProtos.Response.Analyze response = analyze("\"");
        Assert.assertTrue(response.getErrorsCount() > 0);
    }

    @Test
    public void analyzeSyntaxError() {
        IdedProtos.Response.Analyze response = analyze("fn fn");
        Assert.assertTrue(response.getErrorsCount() > 0);
    }

    @Test
    public void analyzeSemanticError() {
        IdedProtos.Response.Analyze response = analyze("fn foo() -> int {}\n");
        Assert.assertTrue(response.getErrorsCount() > 0);
        boolean found = false;
        for (IdedProtos.Error error : response.getErrorsList()) {
            if (error.getMsg().contains("main function not found")) {
                found = true;
            }
        }
        Assert.assertTrue(found);
    }

}
