package com.github.stepancheg.rustide.idea;

import com.intellij.lang.Language;
import com.intellij.openapi.fileTypes.LanguageFileType;
import org.jetbrains.annotations.NotNull;
import org.jetbrains.annotations.Nullable;

import javax.swing.*;

/**
 * @author Stepan Koltsov
 */
public class RustFileType extends LanguageFileType {
    public static final RustFileType INSTANCE = new RustFileType();
    public static final String DEFAULT_EXTENSION = "rs";

    public RustFileType() {
        super(RustLanguage.INSTANCE);
    }

    @NotNull
    @Override
    public String getName() {
        return "Rust";
    }

    @NotNull
    @Override
    public String getDescription() {
        return "Rust";
    }

    @NotNull
    @Override
    public String getDefaultExtension() {
        return DEFAULT_EXTENSION;
    }

    @Nullable
    @Override
    public Icon getIcon() {
        return RustIcon.ICON;
    }
}
