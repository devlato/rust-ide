package com.github.stepancheg.rustide.idea;

import com.intellij.lexer.FlexAdapter;
import com.intellij.lexer.FlexLexer;
import com.intellij.lexer.Lexer;

import java.io.Reader;

/**
 * @author Stepan Koltsov
 */
public class RustLexer extends FlexAdapter {
    public RustLexer() {
        super(new _RustLexer((Reader) null));
    }
}
