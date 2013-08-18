package com.github.stepancheg.rustide.client.perftest;

import com.github.stepancheg.rustide.client.IdedClient;

/**
 * @author Stepan Koltsov
 */
public class PingPerftest {
    public static void main(String[] args) {
        final IdedClient client = new IdedClient();
        Perftest.run(new Runnable() {
            @Override
            public void run() {
                client.ping();
            }
        });
    }
}
