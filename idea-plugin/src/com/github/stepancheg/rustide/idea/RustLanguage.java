package com.github.stepancheg.rustide.idea;

import com.intellij.lang.Language;
import com.intellij.openapi.fileTypes.SingleLazyInstanceSyntaxHighlighterFactory;
import com.intellij.openapi.fileTypes.SyntaxHighlighter;
import com.intellij.openapi.fileTypes.SyntaxHighlighterFactory;
import org.jetbrains.annotations.NotNull;

/**
 * @author Stepan Koltsov
 */
public class RustLanguage extends Language {
    public static final RustLanguage INSTANCE = new RustLanguage();

    public RustLanguage() {
        super("Rust");
    }
}
