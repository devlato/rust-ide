package com.github.stepancheg.rustide.idea.psi;

import com.github.stepancheg.rustide.idea.RustFileType;
import com.github.stepancheg.rustide.idea.RustLanguage;
import com.intellij.extapi.psi.PsiFileBase;
import com.intellij.lang.Language;
import com.intellij.openapi.fileTypes.FileType;
import com.intellij.psi.FileViewProvider;
import org.jetbrains.annotations.NotNull;

/**
 * @author Stepan Koltsov
 */
public class RustFile extends PsiFileBase {
    public RustFile(@NotNull FileViewProvider viewProvider) {
        super(viewProvider, RustLanguage.INSTANCE);
    }

    @NotNull
    @Override
    public FileType getFileType() {
        return RustFileType.INSTANCE;
    }
}
