package com.github.stepancheg.rustide.idea;

import com.intellij.psi.tree.IElementType;
import org.jetbrains.annotations.NonNls;

/**
 * @author Stepan Koltsov
 */
public class RustElementType extends IElementType {
  public RustElementType(@NonNls String debugName) {
    super(debugName, RustLanguage.INSTANCE);
  }

  public String toString() {
    return "Properties:" + super.toString();
  }}
