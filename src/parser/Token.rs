// Copyright 2024 LangVM Project
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0
// that can be found in the LICENSE file and https://mozilla.org/MPL/2.0/.

use crate::scanner::BasicToken::IntFormat;
use crate::scanner::PosRange::PosRange;

#[derive(Clone)]
pub enum TokenKind {
    Ident,
    Operator,

    Int(IntFormat),
    Float,
    String,
    Char,

    Delimiter,

    Comment,
}

pub struct Token {
    pub Pos: PosRange,
    pub Kind: TokenKind,
    pub Literal: Vec<char>,
}

impl Token {
    pub fn clone(&self) -> Token {
        Token {
            Pos: self.Pos.clone(),
            Kind: self.Kind.clone(),
            Literal: self.Literal.clone(),
        }
    }
}
