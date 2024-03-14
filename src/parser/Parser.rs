// Copyright 2024 LangVM Project
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0
// that can be found in the LICENSE file and https://mozilla.org/MPL/2.0/.

use crate::parser::AST::{Expr, Field, FieldList, FuncDecl, FuncType, Ident, ImportDecl, Node, Stmt, StmtBlock, StructType, TraitType, Type};
use crate::parser::Diagnosis::{SyntaxError, UnexpectedNodeError};
use crate::parser::Scanner::{NewScanner, Scanner};
use crate::parser::Token::{Token, TokenKind};
use crate::scanner::BasicScanner::BasicScannerError;
use crate::scanner::Position::Position;
use crate::scanner::PosRange::PosRange;
use crate::tag_matches;

pub enum ParserError {
    ScannerError(BasicScannerError),
}

pub struct Parser {
    pub Scanner: Scanner,

    pub SyntaxErrors: Vec<SyntaxError>,
}

pub fn NewParser(buffer: Vec<char>) -> Parser {
    Parser {
        Scanner: NewScanner(buffer),
        SyntaxErrors: vec![],
    }
}

macro_rules! begin_end {
    ($self: expr, $begin: expr) => {
        PosRange{ Begin: $begin, End: $self.GetPos() }
    };
}

macro_rules! parse_list {
    ($self: expr, $unit: ty, $parser: ident, $delimiter: expr, $term: expr) => { if true {
        let mut list: Vec<$unit> = vec![];
        loop {
            list.push($self.$parser()?);
            match &$self.Token.Kind {
                it if tag_matches!(it, &$delimiter) => {
                    $self.Scan()?;
                    match &$self.Token.Kind {
                        it if tag_matches!(it, &$term) => { break; }
                        _ => {}
                    }
                }
                it if tag_matches!(it, &$term) => { break; }
                _ => {}
            }
        }
        list
    } else { panic!() }};
}

impl Parser {
    pub fn GetPos(mut self) -> Position { self.Scanner.GetPos() }

    pub fn Scan(&mut self) -> Result<Token, ParserError> {}

    pub fn Report(&mut self, e: SyntaxError) {
        self.SyntaxErrors.push(e);
    }

    pub fn ReportAndRecover(&mut self, e: SyntaxError) -> Result<(), ParserError> {
        self.SyntaxErrors.push(e);
        while match self.Token.Kind {
            TokenKind::SEMICOLON | TokenKind::RBRACE => { false }
            _ => { true }
        } { self.Scan()?; }
        Ok(())
    }

    pub fn MatchTerm(&mut self, term: TokenKind) -> Result<Token, ParserError> {
        let token = self.Token.clone();
        self.Scan()?;
        if tag_matches!(&token.Kind, &term) {
            self.Report(SyntaxError::UnexpectedNode(UnexpectedNodeError { Want: Node::TokenKind(term), Have: Node::TokenKind(token.Kind) }));
        }
        Ok(token)
    }

    pub fn ExpectIdent(&mut self) -> Result<Ident, ParserError> {
        let token = self.Token.clone();

        match token.Kind {
            TokenKind::Ident => {
                self.Scan()?;
                Ok(Ident { Pos: token.Pos, Token: token.clone() })
            }
            _ => {
                self.ReportAndRecover(SyntaxError::UnexpectedNode(UnexpectedNodeError { Want: Node::TokenKind(TokenKind::Ident), Have: Node::Token(self.Token.clone()) }))?;
                Ok(Ident {})
            }
        }
    }

    pub fn ExpectField(&mut self) -> Result<Field, ParserError> {
        let begin = self.GetPos();

        Ok(Field {
            Name: self.ExpectIdent()?,
            Type: self.ExpectType()?,
            Pos: begin_end!(self, begin),
        })
    }

    pub fn ExpectFieldList(&mut self, delimiter: TokenKind, term: TokenKind) -> Result<FieldList, ParserError> {
        let begin = self.GetPos();

        Ok(FieldList {
            FieldList: parse_list!(self, Field, ExpectField, delimiter, term),
            Pos: begin_end!(self, begin),
        })
    }

    pub fn ExpectType(&mut self) -> Result<Type, ParserError> {
        match self.Token.Kind {
            TokenKind::STRUCT => { Ok(Type::StructType(Box::new(self.ExpectStructType()?))) }
            TokenKind::TRAIT => { Ok(Type::TraitType(Box::new(self.ExpectTraitType()?))) }
            _ => {
                Err(ParserError::UnexpectedNodeError(UnexpectedNodeError { Want: todo!(), Have: todo!() }))
            }
        }
    }

    pub fn ExpectFuncType(&mut self) -> Result<FuncType, ParserError> {
        let begin = self.GetPos();

        self.MatchTerm(TokenKind::LPAREN)?;

        let params = self.ExpectFieldList(TokenKind::COMMA, TokenKind::RPAREN)?;

        match self.Token.Kind {
            TokenKind::PASS => {
                Ok(FuncType {
                    Params: params,
                    Result: self.ExpectType()?,
                    Pos: begin_end!(self, begin),
                })
            }
            _ => {
                Ok(FuncType {
                    Params: params,
                    Result: Type::None,
                    Pos: begin_end!(self, begin),
                })
            }
        }
    }

    pub fn ExpectStructType(&mut self) -> Result<StructType, ParserError> {
        let begin = self.GetPos();

        self.MatchTerm(TokenKind::STRUCT)?;
        self.MatchTerm(TokenKind::LBRACE)?;

        let name = self.ExpectIdent()?;

        let fieldList = self.ExpectFieldList(TokenKind::SEMICOLON, TokenKind::RBRACE)?;

        Ok(StructType { Pos: begin_end!(self, begin), Name: name, FieldList: fieldList })
    }

    pub fn ExpectTraitType(&mut self) -> Result<TraitType, ParserError> {
        let begin = self.GetPos();

        self.MatchTerm(TokenKind::TRAIT)?;

        let name = self.ExpectIdent()?;

        Ok(TraitType {
            Name: name,
            Pos: begin_end!(self, begin),
        })
    }

    pub fn ExpectImportDecl(&mut self) -> Result<ImportDecl, ParserError> {
        let begin = self.GetPos();

        self.MatchTerm(TokenKind::IMPORT)?;
        match self.Token.Kind {
            TokenKind::Ident => {
                Ok(ImportDecl {
                    Alias: Some(self.ExpectIdent()?),
                    Canonical: self.MatchTerm(TokenKind::String)?,
                    Pos: begin_end!(self, begin),
                })
            }
            TokenKind::String => {
                Ok(ImportDecl {
                    Alias: None,
                    Canonical: self.MatchTerm(TokenKind::String)?,
                    Pos: begin_end!(self, begin),
                })
            }
            _ => {
                Err(ParserError::UnexpectedNodeError(UnexpectedNodeError {
                    Want: Node::TokenKind(TokenKind::String),
                    Have: Node::Token(self.Token.clone()),
                }))
            }
        }
    }

    pub fn ExpectFuncDecl(&mut self) -> Result<FuncDecl, ParserError> {
        let begin = self.GetPos();

        self.MatchTerm(TokenKind::FUNC)?;

        let name = self.ExpectIdent()?;

        self.MatchTerm(TokenKind::LPAREN)?;

        let params = match self.Token.Kind {
            TokenKind::RPAREN => {
                FieldList { Pos: begin_end!(self, self.GetPos()), FieldList: vec![] }
            }
            _ => {
                self.ExpectFieldList(TokenKind::COMMA, TokenKind::RPAREN)?
            }
        };

        let typ = match self.Token.Kind {
            TokenKind::PASS => {
                self.Scan()?;
                FuncType {
                    Pos: begin_end!(self, begin),
                    Params: params,
                    Result: self.ExpectType()?,
                }
            }
            _ => {
                FuncType {
                    Pos: begin_end!(self, begin),
                    Params: params,
                    Result: Type::None,
                }
            }
        };

        Ok(FuncDecl {
            Name: name,
            Type: typ,
            Pos: begin_end!(self, begin.clone()),
        })
    }

    pub fn ExpectExpr(&mut self) -> Result<Expr, ParserError> {}

    pub fn ExpectStmt(&mut self) -> Result<Stmt, ParserError> {
        let expr = self.ExpectExpr()?;

        match self.Token.Kind {}

        Ok(Stmt {})
    }

    pub fn ExpectStmtBlock(&mut self) -> Result<StmtBlock, ParserError> {
        let begin = self.GetPos();

        Ok(StmtBlock {
            StmtList: parse_list!(self, Stmt, ExpectStmt, TokenKind::SEMICOLON, TokenKind::None),
            Expr: Expr::None, // TODO
            Pos: begin_end!(self, begin),
        })
    }
}
