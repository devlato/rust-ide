package com.github.stepancheg.rustide.client;

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

}
