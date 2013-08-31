package com.github.stepancheg.rustide.idea;

import com.intellij.lexer.FlexLexer;
import com.intellij.psi.tree.IElementType;

%%

%class _RustLexer
%implements FlexLexer
%unicode
%function advance
%type IElementType
%eof{ return;
%eof}

//CRLF= \n | \r | \r\n
//WHITE_SPACE_CHAR=[\ \n\r\t\f]
//VALUE_CHARACTER=[^\n\r\f\\] | "\\"{CRLF} | "\\".
//END_OF_LINE_COMMENT=("#"|"!")[^\r\n]*
//KEY_SEPARATOR=[\ \t]*[:=][\ \t]* | [\ \t]+
//KEY_CHARACTER=[^:=\ \n\r\t\f\\] | "\\"{CRLF} | "\\".

DIGIT = [0-9]

%state IN_VALUE
%state IN_KEY_VALUE_SEPARATOR

%%

//<YYINITIAL> {END_OF_LINE_COMMENT} { yybegin(YYINITIAL); return RustTokenTypes.END_OF_LINE_COMMENT; }

//<YYINITIAL> {KEY_CHARACTER}+ { yybegin(IN_KEY_VALUE_SEPARATOR); return RustTokenTypes.KEY_CHARACTERS; }
//<IN_KEY_VALUE_SEPARATOR> {KEY_SEPARATOR} { yybegin(IN_VALUE); return RustTokenTypes.KEY_VALUE_SEPARATOR; }
//<IN_VALUE> {VALUE_CHARACTER}+ { yybegin(YYINITIAL); return RustTokenTypes.VALUE_CHARACTERS; }

//<IN_KEY_VALUE_SEPARATOR> {CRLF}{WHITE_SPACE_CHAR}* { yybegin(YYINITIAL); return RustTokenTypes.WHITE_SPACE; }
//<IN_VALUE> {CRLF}{WHITE_SPACE_CHAR}* { yybegin(YYINITIAL); return RustTokenTypes.WHITE_SPACE; }
//{WHITE_SPACE_CHAR}+ { return RustTokenTypes.WHITE_SPACE; }


{DIGIT}+ { return RustTokenTypes.NUMBER; }

\n { return RustTokenTypes.BAD_CHARACTER; }
. { return RustTokenTypes.BAD_CHARACTER; }

