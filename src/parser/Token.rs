// Copyright 2024 LangVM Project
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0
// that can be found in the LICENSE file and https://mozilla.org/MPL/2.0/.

use std::fmt::{Display, Formatter};

use crate::def_tokens;
use crate::scanner::PosRange::PosRange;

def_tokens! {
    keywordLookup: TokenKind => {
        PASS        "<-",

        BREAK       "break",
        CONTINUE    "continue",
        ELSE        "else",
        FOR         "for",
        FUNC        "func",
        IF          "if",
        IMPORT      "import",
        TRAIT       "trait",
        RETURN      "return",
        MATCH       "match",
        STRUCT      "struct",
        VAR         "mut",
        VAL         "let",

        LPAREN      "(",
        LBRACK      "[",
        LBRACE      "{",
        RPAREN      ")",
        RBRACK      "]",
        RBRACE      "}",
        
        COLON       ":",
        SEMICOLON   ";",
        COMMA       ",",
        DOT         ".",
        
        NEWLINE     "\n"
    }
}

#[derive(Clone, Default)]
pub struct Token {
    pub Pos: PosRange,
    pub Kind: TokenKind,
    pub Literal: Vec<char>,
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.Pos, String::from_iter(&self.Literal))
    }
}
