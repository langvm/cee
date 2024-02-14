// Copyright 2024 LangVM Project
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0
// that can be found in the LICENSE file and https://mozilla.org/MPL/2.0/.

use crate::Position::Position;

pub enum TokenKind {
    IDENT,
	OPERATOR,

    INT(IntFormat),
    FLOAT,
    STRING,
    CHAR,

	DELIMITER,

    COMMENT,
}

pub enum IntFormat {
    BIN,
    OCT,
    DEC,
    HEX,
}

pub struct Token {
    pub Pos: Position,
    pub Kind: TokenKind,
    pub Literal: Vec<char>,
}
