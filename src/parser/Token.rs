// Copyright 2024 LangVM Project
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0
// that can be found in the LICENSE file and https://mozilla.org/MPL/2.0/.

use std::collections::HashMap;

use crate::scanner::BasicToken::IntFormat;
use crate::scanner::Position::Position;
use crate::scanner::PosRange::PosRange;

#[derive(Clone)]
pub enum TokenKind {
    None,

    Ident,
    Operator,

    Int(IntFormat),
    Float,
    String,
    Char,

    Delimiter,

    Comment,

    // Keywords

    BREAK,
    CASE,
    CHAN,
    CONST,
    CONTINUE,

    DEFAULT,
    DEFER,
    ELSE,
    FOR,

    FUNC,
    GO,
    GOTO,
    IF,
    IMPORT,

    TRAIT,
    MAP,
    PACKAGE,
    RANGE,
    RETURN,

    MATCH,
    STRUCT,
    TYPE,
    VAR,
    VAL,

    LPAREN,
    LBRACK,
    LBRACE,

    RPAREN,
    RBRACK,
    RBRACE,

    COMMA,
    SEMICOLON,
    COLON,
    NEWLINE,
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

pub fn KeywordLookup() -> HashMap<String, TokenKind> {
    HashMap::from([
        ("break".to_string(), TokenKind::BREAK),
        ("const".to_string(), TokenKind::CONST),
        ("continue".to_string(), TokenKind::CONTINUE),
        ("else".to_string(), TokenKind::ELSE),
        ("for".to_string(), TokenKind::FOR),
        ("fun".to_string(), TokenKind::FUNC),
        ("go".to_string(), TokenKind::GO),
        ("if".to_string(), TokenKind::IF),
        ("import".to_string(), TokenKind::IMPORT),
        ("trait".to_string(), TokenKind::TRAIT),
        ("return".to_string(), TokenKind::RETURN),
        ("match".to_string(), TokenKind::MATCH),
        ("struct".to_string(), TokenKind::STRUCT),
        ("typedef".to_string(), TokenKind::TYPE),
        ("mut".to_string(), TokenKind::VAR),
        ("let".to_string(), TokenKind::VAL),
        ("(".to_string(), TokenKind::LPAREN),
        ("[".to_string(), TokenKind::LBRACK),
        ("{".to_string(), TokenKind::LBRACE),
        (")".to_string(), TokenKind::RPAREN),
        ("]".to_string(), TokenKind::RBRACK),
        ("}".to_string(), TokenKind::RBRACE),
        (",".to_string(), TokenKind::COMMA),
        (";".to_string(), TokenKind::SEMICOLON),
        (":".to_string(), TokenKind::COLON),
        ("\n".to_string(), TokenKind::NEWLINE),
    ])
}