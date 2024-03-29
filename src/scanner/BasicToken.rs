// Copyright 2024 LangVM Project
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0
// that can be found in the LICENSE file and https://mozilla.org/MPL/2.0/.

use std::fmt::Display;

use crate::scanner::PosRange::PosRange;

#[derive(Clone)]
pub enum BasicTokenKind {
    Ident,
    Operator,

    Int(IntFormat),
    Float,
    String,
    Char,

    Delimiter,

    Comment,
}

#[derive(Clone)]
pub enum IntFormat {
    BIN = 2,
    OCT = 8,
    DEC = 10,
    HEX = 16,
}

pub struct BasicToken {
    pub Pos: PosRange,
    pub Kind: BasicTokenKind,
    pub Literal: Vec<char>,
}

impl BasicToken {
    pub fn clone(&self) -> BasicToken {
        BasicToken {
            Pos: self.Pos.clone(),
            Kind: self.Kind.clone(),
            Literal: self.Literal.clone(),
        }
    }
}
