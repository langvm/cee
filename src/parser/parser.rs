// Copyright 2024 LangVM Project
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0
// that can be found in the LICENSE file and https://mozilla.org/MPL/2.0/.

use std::collections::HashMap;

use err_rs::*;

use crate::ast::*;
use crate::parser::*;
use crate::scanner::*;
use crate::tag_matches;

#[derive(Debug)]
pub enum ParserError {
    ScannerError(BasicScannerError),
}

pub struct Parser {
    pub Scanner: BasicScanner,

    pub KeywordLookup: &'static HashMap<String, TokenKind>,

    pub ReachedEOF: bool,
    pub Token: Token,

    // Insert semicolon when true
    pub CompleteSemicolon: bool,

    // Recover from error
    pub QuoteStack: Vec<TokenKind>,

    // Diagnosis
    pub SyntaxErrors: Vec<SyntaxError>,

    // Package names
    pub NamespaceIdents: HashMap<String, ImportDecl>,
}

macro_rules! begin_end {
    ($self: expr, $begin: expr) => {
        PosRange{ Begin: $begin, End: $self.GetPos() }
    };
}

macro_rules! parse_list {
    (
        $self: expr,
        $ast_node: ty,
        $parser_func: ident,
        $delimiter: expr,
        $term: expr) => { if true {
        let begin = $self.GetPos();

        let mut list: Vec<$ast_node> = vec![];
        loop {
            list.push($self.$parser_func()?);
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
        List {
            Pos: PosRange { Begin: begin, End: $self.GetPos() },
            Elements: list,
        }
    } else { panic!() }};
}

impl Parser {
    pub fn new(buffer: Vec<char>) -> Parser {
        Parser {
            Scanner: BasicScanner {
                BufferScanner: BufferScanner::new(buffer),
                Delimiters: vec!['(', ')', '[', ']', '{', '}', ',', ';', '/'],
                Whitespaces: vec![' ', '\t', '\r'],
            },
            KeywordLookup: &keywordLookup,
            ReachedEOF: false,
            Token: Token::default(),

            CompleteSemicolon: false,

            QuoteStack: vec![],

            SyntaxErrors: vec![],

            NamespaceIdents: HashMap::new(),
        }
    }

    pub fn GetPos(&self) -> Position { self.Scanner.GetPos() }

    pub fn Scan(&mut self) -> Result<(), ParserError> {
        let bt = wrap_result!(ParserError::ScannerError, self.Scanner.Scan());

        // Semicolon complete: replace newline to semicolon
        match self.Token.Kind {
            TokenKind::NEWLINE => {
                if self.CompleteSemicolon {
                    self.CompleteSemicolon = false;
                    self.Token = Token {
                        Pos: bt.Pos,
                        Kind: TokenKind::SEMICOLON,
                        Literal: vec![';'],
                    };
                    ok!(());
                }
            }
            _ => {}
        }

        self.CompleteSemicolon = false;

        let literal = bt.Literal.iter().collect::<String>();

        // Determines whether BasicToken is a keyword, operator or delimiter.
        let kind = match bt.Kind {
            BasicTokenKind::Ident => {
                if keywordLookup.contains_key(&literal) {
                    keywordLookup.get(&literal).unwrap().clone()
                } else {
                    TokenKind::Ident
                }
            }
            BasicTokenKind::Operator => {
                if keywordLookup.contains_key(&literal) {
                    keywordLookup.get(&literal).unwrap().clone()
                } else {
                    TokenKind::Operator
                }
            }
            BasicTokenKind::Delimiter => {
                let kind = keywordLookup.get(&literal).unwrap().clone(); // Panic means design error.
                match kind {
                    TokenKind::LPAREN => { self.QuoteStack.push(TokenKind::RPAREN) }
                    TokenKind::LBRACE => { self.QuoteStack.push(TokenKind::RBRACE) }
                    TokenKind::LBRACK => { self.QuoteStack.push(TokenKind::RBRACK) }
                    _ => {}
                }
                kind
            }
            BasicTokenKind::Int(format) => { TokenKind::Int(format) }
            BasicTokenKind::Float => { TokenKind::Float } // TODO
            BasicTokenKind::String => { TokenKind::String }
            BasicTokenKind::Char => { TokenKind::Char }
            BasicTokenKind::Comment => { return self.Scan(); }
        };

        self.Token = Token {
            Pos: bt.Pos,
            Kind: kind,
            Literal: bt.Literal,
        };

        Ok(())
    }

    pub fn Report(&mut self, e: SyntaxError) {
        self.SyntaxErrors.push(e);
    }

    pub fn ReportAndRecover(&mut self, e: SyntaxError) -> Result<(), ParserError> {
        self.SyntaxErrors.push(e);

        if self.QuoteStack.len() != 0 {
            while !tag_matches!(&self.Token.Kind, &self.QuoteStack.pop().unwrap()) {
                self.Scan()?;
            }
        }

        Ok(())
    }

    pub fn MatchTerm(&mut self, term: TokenKind) -> Result<Token, ParserError> {
        let token = self.Token.clone();
        self.Scan()?;
        if tag_matches!(&token.Kind, &term) {
            self.Report(SyntaxError::UnexpectedNode(UnexpectedNodeError { Want: Node::TokenKind(term), Have: Node::TokenKind(token.Kind) }));
            Ok(Token::default())
        } else {
            Ok(token)
        }
    }
}

macro_rules! match_terms {
    ($self:expr, $($term:expr), *) => {
        $(
        $self.MatchTerm($term)?;
        )*
    };
}

impl Parser {
    pub fn ExpectIdent(&mut self) -> Result<Ident, ParserError> {
        let token = self.Token.clone();

        Ok(match token.Kind {
            TokenKind::Ident => {
                self.Scan()?;
                Ident { Pos: token.Pos.clone(), Token: token.clone() }
            }
            _ => {
                self.ReportAndRecover(SyntaxError::UnexpectedNode(UnexpectedNodeError { Want: Node::TokenKind(TokenKind::Ident), Have: Node::Token(self.Token.clone()) }))?;
                Ident::default()
            }
        })
    }

    pub fn ExpectField(&mut self) -> Result<Field, ParserError> {
        let begin = self.GetPos();

        Ok(Field {
            Name: self.ExpectIdent()?,
            Type: self.ExpectType()?,
            Pos: begin_end!(self, begin),
        })
    }

    pub fn ExpectFieldList(&mut self, delimiter: TokenKind, term: TokenKind) -> Result<List<Field>, ParserError> {
        Ok(parse_list!(self, Field, ExpectField, delimiter, term))
    }

    pub fn ExpectType(&mut self) -> Result<Type, ParserError> {
        Ok(match self.Token.Kind {
            TokenKind::STRUCT => { Type::StructType(Box::new(self.ExpectStructType()?)) }
            TokenKind::TRAIT => { Type::TraitType(Box::new(self.ExpectTraitType()?)) }
            _ => {
                self.ReportAndRecover(SyntaxError::UnexpectedNode(UnexpectedNodeError { Want: todo!(), Have: todo!() }))?;
                Type::None
            }
        })
    }

    pub fn ExpectFuncType(&mut self) -> Result<FuncType, ParserError> {
        let begin = self.GetPos();

        self.MatchTerm(TokenKind::LPAREN)?;

        let params = self.ExpectFieldList(TokenKind::COMMA, TokenKind::RPAREN)?;

        Ok(match self.Token.Kind {
            TokenKind::PASS => {
                FuncType {
                    Params: params,
                    Result: self.ExpectType()?,
                    Pos: begin_end!(self, begin),
                }
            }
            _ => {
                FuncType {
                    Params: params,
                    Result: Type::None,
                    Pos: begin_end!(self, begin),
                }
            }
        })
    }

    pub fn ExpectStructType(&mut self) -> Result<StructType, ParserError> {
        let begin = self.GetPos();

        match_terms!(self, TokenKind::STRUCT, TokenKind::LBRACE);

        Ok(StructType {
            Name: self.ExpectIdent()?,
            FieldList: parse_list!(self, Field, ExpectField, TokenKind::SEMICOLON, TokenKind::RBRACE),
            Pos: begin_end!(self, begin),
        })
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
        Ok(ImportDecl {
            Alias: self.ExpectIdent()?,
            Canonical: self.MatchTerm(TokenKind::String)?,
            Pos: begin_end!(self, begin),
        })
    }

    pub fn ExpectFuncDecl(&mut self) -> Result<FuncDecl, ParserError> {
        let begin = self.GetPos();

        self.MatchTerm(TokenKind::FUNC)?;

        let name = self.ExpectIdent()?;

        self.MatchTerm(TokenKind::LPAREN)?;

        let params = match self.Token.Kind {
            TokenKind::RPAREN => {
                List { Pos: begin_end!(self, self.GetPos()), Elements: vec![] }
            }
            _ => {
                parse_list!(self, Field, ExpectField, TokenKind::COMMA, TokenKind::RPAREN)
            }
        };

        let typ = match self.Token.Kind {
            TokenKind::PASS => {
                self.Scan()?;
                FuncType {
                    Params: params,
                    Result: self.ExpectType()?,
                    Pos: begin_end!(self, begin.clone()),
                }
            }
            _ => {
                FuncType {
                    Pos: begin_end!(self, begin.clone()),
                    Params: params,
                    Result: Type::None,
                }
            }
        };

        Ok(FuncDecl {
            Name: name,
            Type: typ,
            Pos: begin_end!(self, begin),
        })
    }

    pub fn ExpectExpr(&mut self) -> Result<Expr, ParserError> {
        Ok(match self.Token.Kind {
            TokenKind::Ident => { todo!() }
            TokenKind::Int(_) | TokenKind::Float | TokenKind::Char | TokenKind::String => {
                Expr::LiteralValue(Box::new(LiteralValue {
                    Pos: self.Token.Pos.clone(),
                    Token: self.Token.clone(),
                }))
            }
            _ => { todo!() }
        })
    }

    pub fn ExpectStmt(&mut self) -> Result<Stmt, ParserError> {
        let expr = self.ExpectExpr()?;

        Ok(match self.Token.Kind {
            TokenKind::MUT => {
                Stmt::MutDecl(Box::from(self.ExpectMutDecl()?))
            }
            TokenKind::VAL => {
                Stmt::MutDecl(Box::from(self.ExpectMutDecl()?))
            }
            _ => {
                Stmt::Expr(Box::from(self.ExpectExpr()?))
            }
        })
    }

    pub fn ExpectMutDecl(&mut self) -> Result<MutDecl, ParserError> {
        let begin = self.GetPos();

        self.MatchTerm(TokenKind::MUT)?;

        Ok(MutDecl {
            Name: self.ExpectIdent()?,
            Type: self.ExpectType()?,
            Pos: begin_end!(self, begin),
        })
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
