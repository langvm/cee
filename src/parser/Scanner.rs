// Copyright 2024 LangVM Project
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0
// that can be found in the LICENSE file and https://mozilla.org/MPL/2.0/.

use std::collections::HashMap;

use crate::parser::Parser::ParserError;
use crate::parser::Token::{KeywordLookup, Token, TokenKind};
use crate::scanner::BasicScanner::{BasicScanner, NewBufferScanner};
use crate::scanner::BasicToken::BasicTokenKind;
use crate::scanner::Position::Position;
use crate::scanner::PosRange::PosRange;

pub struct Scanner {
    pub Scanner: BasicScanner,

    pub KeywordLookup: HashMap<String, TokenKind>,

    pub ReachedEOF: bool,
    pub Token: Token,

    pub CompleteSemicolon: bool,
}

pub fn NewScanner(buffer: Vec<char>) -> Scanner {
    Scanner {
        Scanner: BasicScanner {
            BufferScanner: NewBufferScanner(buffer),
            Delimiters: vec!['(', ')', '[', ']', '{', '}', ',', ';', '/'],
            Whitespaces: vec![' ', '\t', '\r'],
        },
        KeywordLookup: KeywordLookup(),
        ReachedEOF: false,
        Token: Token {
            Pos: PosRange { Begin: Position { Offset: 0, Line: 0, Column: 0 }, End: Position { Offset: 0, Line: 0, Column: 0 } },
            Kind: TokenKind::None,
            Literal: vec![],
        },

        CompleteSemicolon: false,
    }
}

impl Scanner {
    pub fn GetPos(&self) -> Position { self.Scanner.GetPos() }

    pub fn Scan(&mut self) -> Result<(), ParserError> {
        let bt = match self.Scanner.Scan() {
            Ok(token) => { token }
            Err(err) => { return Err(ParserError::ScannerError(err)); }
        };

        match self.Token.Kind {
            TokenKind::NEWLINE => {
                if self.CompleteSemicolon {
                    self.CompleteSemicolon = false;
                    self.Token = Token {
                        Pos: bt.Pos,
                        Kind: TokenKind::SEMICOLON,
                        Literal: vec![';'],
                    };
                    return Ok(());
                }
            }
            _ => {}
        }

        self.CompleteSemicolon = false;

        macro_rules! lookup {
            ($e: ident) => {{
                let s = bt.Literal.iter().collect::<String>();
                if self.KeywordLookup.contains_key(&s) {
                    self.KeywordLookup.get(&s).unwrap().clone()
                } else {
                    TokenKind::$e
                }
            }};
        }

        let kind = match bt.Kind {
            BasicTokenKind::Ident | BasicTokenKind::Operator | BasicTokenKind::Delimiter => { lookup!(Operator) }
            BasicTokenKind::Int(format) => { TokenKind::Int(format) }
            BasicTokenKind::Float => { TokenKind::Float } // TODO
            BasicTokenKind::String => { TokenKind::String }
            BasicTokenKind::Char => { TokenKind::Char }
            BasicTokenKind::Comment => { return self.Scan(); }
            _ => { panic!("impossible") }
        };

        self.Token = Token {
            Pos: bt.Pos,
            Kind: kind,
            Literal: bt.Literal,
        };

        Ok(())
    }
}
