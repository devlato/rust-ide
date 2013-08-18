package com.github.stepancheg.rustide.client.perftest;

/**
 * @author Stepan Koltsov
 */
public class Perftest {

    public static void run(Runnable test) {
        Thread.currentThread().setUncaughtExceptionHandler(new Thread.UncaughtExceptionHandler() {
            @Override
            public void uncaughtException(Thread t, Throwable e) {
                e.printStackTrace();
                System.exit(1);
            }
        });

        for (;;) {
            long start = System.currentTimeMillis();
            long iterations = 0;
            int iterationsPerStep = 1;
            // TODO: race
            while (System.currentTimeMillis() - start < 1000) {
                for (int i = 0; i < iterationsPerStep; ++i) {
                    test.run();
                }
                iterations += iterationsPerStep;
                iterationsPerStep = iterationsPerStep * 2;
            }
            long d = System.currentTimeMillis() - start;
            long rp1000s = iterations * 1000000 / d;
            long durationUs = 1000000000 / rp1000s;
            System.out.println(durationUs + "us per iteration");
        }
    }

}
