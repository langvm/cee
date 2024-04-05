// Copyright 2024 LangVM Project
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0
// that can be found in the LICENSE file and https://mozilla.org/MPL/2.0/.

use std::fmt::{Display, Formatter};

use crate::scanner::*;

macro_rules! def_tokens {
    ($typ_name:ident => { $($name:ident $literal:expr), * }) => {
        #[derive(Clone, Debug)]
        pub enum $typ_name {
            None,
            Ident,
            Operator,
            Int(crate::scanner::IntFormat),
            Float,
            String,
            Char,
            $($name,)*
        }

        impl Default for $typ_name {
            fn default() -> Self { Self::None }
        }

        impl std::fmt::Display for $typ_name {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", match self {
                    $(
                    $typ_name::$name => { $literal }
                    )*
                    _ => { "" }
                })
            }
        }

        impl $typ_name {
            pub fn KeywordLookup() -> std::collections::HashMap<String, $typ_name> {
                std::collections::HashMap::from([
                    $(
                    ($literal.to_string(), $typ_name::$name),
                    )*
                ])
            }
        }
    };
}

def_tokens! {
    TokenKind => {
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
        MUT         "mut",
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
    pub Literal: String,
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.Pos, &self.Literal)
    }
}
