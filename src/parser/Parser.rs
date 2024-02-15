// Copyright 2024 LangVM Project
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0
// that can be found in the LICENSE file and https://mozilla.org/MPL/2.0/.

use std::collections::HashMap;
use std::fmt;
use std::fmt::Formatter;

use crate::parser::AST::{Field, FieldList, FuncDecl, FuncType, Ident, Node, Stmt, StmtBlock, StructType, TraitType, Type};
use crate::parser::Token::{KeywordLookup, Token, TokenKind};
use crate::parser::Token::TokenKind::{RPAREN, SEMICOLON};
use crate::scanner::BasicToken::BasicTokenKind;
use crate::scanner::Position::Position;
use crate::scanner::PosRange::PosRange;
use crate::scanner::Scanner::{NewBufferScanner, Scanner, ScannerError};

pub enum ParserError {
    ScannerError(ScannerError),
    UnexpectedNodeError(UnexpectedNodeError),
}

pub struct UnexpectedNodeError {
    pub Want: Node,
    pub Have: Node,
}

impl fmt::Display for UnexpectedNodeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result { write!(f, "unexpected node: {} but have {}", self.Have, self.Want) }
}

pub struct Parser {
    pub Scanner: Scanner,

    pub KeywordLookup: HashMap<String, TokenKind>,

    pub ReachedEOF: bool,
    pub Token: Token,
}

pub fn NewParser(buffer: Vec<char>) -> Parser {
    Parser {
        Scanner: Scanner {
            BufferScanner: NewBufferScanner(buffer),
            Delimiters: vec!['(', ')', '[', ']', '{', '}', ',', ';', '/'],
            Whitespaces: vec![' ', '\t', '\r'],
        },
        ReachedEOF: false,
        Token: Token {
            Pos: PosRange { Begin: Position { Offset: 0, Line: 0, Column: 0 }, End: Position { Offset: 0, Line: 0, Column: 0 } },
            Kind: TokenKind::None,
            Literal: vec![],
        },
        KeywordLookup: KeywordLookup(),
    }
}

macro_rules! begin_end {
    ($e: expr, $s: expr) => {
        PosRange{ Begin: $e, End: $s.GetPos().clone() }
    };
}

impl Parser {
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

    pub fn MatchTerm(&mut self, term: TokenKind) -> Result<(), ParserError> {
        let token = self.Token.clone();

        match token.Kind {
            term => { Err(ParserError::UnexpectedNodeError(UnexpectedNodeError { Want: Node::Token(term), Have: Node::Token(token.Kind) })) }
            _ => {
                self.Scan()?;
                Ok(())
            }
        }
    }

    pub fn ExpectIdent(&mut self) -> Result<Ident, ParserError> {
        let token = self.Token.clone();

        match token.Kind {
            Ident => {
                self.Scan()?;
                Ok(Ident { Pos: token.Pos, Token: token.clone() })
            }
            _ => {
                Err(ParserError::UnexpectedNodeError(UnexpectedNodeError { Want: Node::Ident, Have: Node::Token(token.Kind) }))
            }
        }
    }

    pub fn ExpectField(&mut self) -> Result<Field, ParserError> {
        let begin = self.GetPos();

        let name = self.ExpectIdent()?;

        let typ = self.ExpectType()?;

        self.MatchTerm(TokenKind::SEMICOLON)?;

        Ok(Field { Pos: begin_end!(begin, self), Name: name, Type: typ })
    }

    pub fn ExpectFieldList(&mut self, delimiter: TokenKind, term: TokenKind) -> Result<FieldList, ParserError> {
        let begin = self.GetPos();

        let mut fieldList: Vec<Field> = vec![];

        loop {
            match self.Token.Kind { // TODO
                term => { break; }
                _ => {
                    fieldList.push(self.ExpectField()?);
                    match self.Token.Kind {
                        delimiter => { self.Scan()?; }
                        term => {
                            break;
                        }
                        _ => {
                            return Err(ParserError::UnexpectedNodeError(UnexpectedNodeError {
                                Want: Node::Token(term),
                                Have: Node::Token(self.Token.Kind),
                            }));
                        }
                    }
                }
            }
        }

        self.Scan()?;

        Ok(FieldList { Pos: begin_end!(begin, self), FieldList: fieldList })
    }

    pub fn ExpectType(&mut self) -> Result<Type, ParserError> {
        match self.Token.Kind {
            TokenKind::STRUCT => { Ok(Type::Struct(Box::new(self.ExpectStructType()?))) }
            TokenKind::TRAIT => { Ok(Type::Trait(Box::new(self.ExpectTraitType()?))) }
            _ => {
                Err(ParserError::UnexpectedNodeError(UnexpectedNodeError { Want: todo!(), Have: todo!() }))
            }
        }
    }

    pub fn ExpectFuncType(&mut self) -> Result<FuncType, ParserError> {
        let begin = self.GetPos();

        self.MatchTerm(TokenKind::LPAREN)?;

        let params = self.ExpectFieldList(TokenKind::COMMA, TokenKind::RPAREN)?;

        let results = match self.Token.Kind {
            TokenKind::LPAREN => {
                let results = self.ExpectFieldList(TokenKind::COMMA, TokenKind::RPAREN)?;
                self.Scan()?;
                results
            }
            _ => {  self.ExpectFieldList(TokenKind::None, TokenKind::None)? }
        };

        Ok(FuncType{ Pos: begin_end!(begin, self) })
    }

    pub fn ExpectStructType(&mut self) -> Result<StructType, ParserError> {
        let begin = self.GetPos();

        self.MatchTerm(TokenKind::STRUCT)?;
        self.MatchTerm(TokenKind::LBRACE)?;

        let name = self.ExpectIdent()?;

        let fieldList = self.ExpectFieldList(TokenKind::SEMICOLON, TokenKind::RBRACE)?;

        Ok(StructType { Pos: begin_end!(begin, self), Name: name, FieldList: fieldList })
    }

    pub fn ExpectTraitType(&mut self) -> Result<TraitType, ParserError> {
        let begin = self.GetPos();

        self.MatchTerm(TokenKind::TRAIT)?;

        let name = self.ExpectIdent()?;

        Ok(TraitType {
            Pos: begin_end!(begin, self),
            Name: name,
        })
    }

    pub fn ExpectFuncDecl(&mut self) -> Result<FuncDecl, ParserError> {
        let begin = self.GetPos();

        let name = self.ExpectIdent()?;

        let typ = self.ExpetFuncType()?;

        let block = self.ExpectStmtBlock()?;

        Ok(FuncDecl {
            Pos: begin_end!(begin, self),
            Name: name,
            Type:,
        })
    }

    pub fn ExpectStmt(&mut self) -> Result<Stmt, ParserError> {
        Ok()
    }

    pub fn ExpectStmtBlock(&mut self) -> Result<StmtBlock, ParserError> {
        Ok(StmtBlock {})
    }
}
