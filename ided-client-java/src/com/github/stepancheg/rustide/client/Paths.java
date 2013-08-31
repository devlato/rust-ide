package com.github.stepancheg.rustide.client;

import java.io.File;

/**
 * @author Stepan Koltsov
 */
public class Paths {
    public File getRustIdeHome() {
        return new File(".").getAbsoluteFile();
    }

    public File getRustHome() {
        return new File(getRustIdeHome(), "rust/x86_64-apple-darwin/stage2");
    }

    public File getIded() {
         return new File(getRustIdeHome(), "ided/ided");
    }
}
