package com.github.stepancheg.rustide.idea;

import com.intellij.psi.TokenType;
import com.intellij.psi.tree.IElementType;
import com.intellij.psi.tree.TokenSet;

/**
 * @author Stepan Koltsov
 */
public class RustTokenTypes {
  static IElementType WHITE_SPACE = TokenType.WHITE_SPACE;
  static IElementType BAD_CHARACTER = TokenType.BAD_CHARACTER;
    static IElementType NUMBER = new RustElementType("number");

  static IElementType END_OF_LINE_COMMENT = new RustElementType("END_OF_LINE_COMMENT");
  static IElementType KEY_CHARACTERS = new RustElementType("KEY_CHARACTERS");
  static IElementType VALUE_CHARACTERS = new RustElementType("VALUE_CHARACTERS");
  static IElementType KEY_VALUE_SEPARATOR = new RustElementType("KEY_VALUE_SEPARATOR");

  static TokenSet COMMENTS = TokenSet.create(END_OF_LINE_COMMENT);
  static TokenSet WHITESPACES = TokenSet.create(WHITE_SPACE);
}
