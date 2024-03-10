// Copyright 2024 LangVM Project
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0
// that can be found in the LICENSE file and https://mozilla.org/MPL/2.0/.

use std::collections::HashMap;

use crate::cmp_enum_tag;
use crate::peg::AST::{Ident, Node, Pattern, Rule};
use crate::peg::Token::{KeywordLookup, Token, TokenKind};
use crate::scanner::BasicToken::BasicTokenKind;
use crate::scanner::Position::Position;
use crate::scanner::PosRange::PosRange;
use crate::scanner::Scanner::{NewBufferScanner, Scanner, ScannerError};

pub struct PEGParser {
    pub Scanner: Scanner,

    pub ReachedEOF: bool,
    pub Token: Token,

    pub KeywordLookup: HashMap<String, crate::parser::Token::TokenKind>,

    pub CompleteSemicolon: bool,
}

pub enum ParserError {
    ScannerError(ScannerError),
    UnexpectedNodeError(UnexpectedNodeError),
}

pub struct UnexpectedNodeError {
    pub Want: Node,
    pub Have: Node,
}

pub fn NewPEGParser(buffer: Vec<char>) -> PEGParser {
    PEGParser {
        Scanner: Scanner {
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

impl PEGParser {
    pub fn GetPos(&self) -> Position { self.Scanner.GetPos() }

    pub fn Scan(&mut self) -> Result<(), ParserError> {
        let bt = match self.Scanner.Scan() {
            Ok(token) => { token }
            Err(err) => { return Err(ParserError::ScannerError(err)); }
        };

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

    pub fn MatchTerm(&mut self, term: TokenKind) -> Result<(), ParserError> {
        cmp_enum_tag! {
            &self.Token.Kind,
            &term => { Err(ParserError::UnexpectedNodeError(UnexpectedNodeError { Want: Node::TokenKind(term), Have: Node::Token(self.Token.clone()) })) };
            _ => {
                self.Scan()?;
                Ok(())
            }
        }
    }

    pub fn ExpectIdent(&mut self) -> Result<Ident, ParserError> {
        let token = self.Token.clone();

        match token.Kind {
            TokenKind::Ident => {
                self.Scan()?;
                Ok(Ident { Pos: token.Pos, Token: token.clone() })
            }
            _ => {
                Err(ParserError::UnexpectedNodeError(UnexpectedNodeError { Want: Node::TokenKind(TokenKind::Ident), Have: Node::Token(self.Token.clone()) }))
            }
        }
    }

    pub fn ExpectPattern(&mut self) -> Result<Pattern, ParserError> {
        let begin = self.GetPos();

        let mut elements: Vec<Box<Node>> = vec![];

        loop {
            match self.Token {
                _ => {}
            }
        }

        Ok(Pattern {
            Pos: PosRange { Begin: begin, End: self.GetPos() },
            Elements: elements,
        })
    }

    pub fn ExpectRule(&mut self) -> Result<Rule, ParserError> {
        let begin = self.GetPos();

        let name = self.ExpectIdent()?;

        self.MatchTerm(TokenKind::COLON)?;

        let mut patterns: Vec<Pattern> = vec![];

        loop {
            patterns.push(self.ExpectPattern()?);
            match self.Token.Kind {
                TokenKind::COLON => { self.Scan()? }
                _ => { break; }
            }
        }

        Ok(Rule {
            Pos: PosRange { Begin: begin, End: self.GetPos() },
            Name: name,
            Patterns: patterns,
        })
    }
}
