package com.github.stepancheg.rustide.idea;

import com.intellij.openapi.fileTypes.FileTypeConsumer;
import com.intellij.openapi.fileTypes.FileTypeFactory;
import org.jetbrains.annotations.NotNull;

/**
 * @author Stepan Koltsov
 */
public class RustFileTypeFactory extends FileTypeFactory {
    @Override
    public void createFileTypes(@NotNull FileTypeConsumer consumer) {
        consumer.consume(RustFileType.INSTANCE, RustFileType.DEFAULT_EXTENSION);
    }
}
