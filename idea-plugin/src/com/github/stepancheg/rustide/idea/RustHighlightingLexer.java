package com.github.stepancheg.rustide.idea;

import com.intellij.lexer.LayeredLexer;
import com.intellij.lexer.Lexer;

/**
 * @author Stepan Koltsov
 */
public class RustHighlightingLexer extends LayeredLexer {
    public RustHighlightingLexer() {
        super(new RustLexer());
    }
}
