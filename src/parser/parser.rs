// Copyright 2024 LangVM Project
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0
// that can be found in the LICENSE file and https://mozilla.org/MPL/2.0/.

use std::collections::HashMap;

use err_rs::*;

use crate::ast;
use crate::parser::*;
use crate::scanner::*;
use crate::tag_matches;

pub trait AstNodeParserTrait<T> {
    fn Expect(p: &mut Parser) -> Result<T, ParserError>;
}

#[derive(Debug)]
pub enum ParserError {
    ScannerError(BasicScannerError),
}

pub struct Parser {
    pub Scanner: BasicScanner,

    pub KeywordLookup: HashMap<String, ast::TokenKind>,

    pub Token: ast::Token,

    // Insert semicolon when true
    pub CompleteSemicolon: bool,

    // Recover from error
    pub QuoteStack: Vec<ast::TokenKind>,

    // Diagnosis
    pub SyntaxErrors: Vec<SyntaxError>,

    // Package names
    pub NamespaceIdents: HashMap<String, ast::ImportDecl>,
}

impl Parser {
    pub fn new(buffer: Vec<char>) -> Parser {
        Parser {
            Scanner: BasicScanner {
                BufferScanner: BufferScanner::new(buffer),
                Delimiters: vec!['(', ')', '[', ']', '{', '}', ',', ';', '/', '\n'],
                Whitespaces: vec![' ', '\t', '\r'],
            },
            KeywordLookup: ast::TokenKind::KeywordLookup(),
            Token: ast::Token::default(),

            CompleteSemicolon: false,

            QuoteStack: vec![],

            SyntaxErrors: vec![],

            NamespaceIdents: HashMap::default(),
        }
    }

    pub fn GetPos(&self) -> Position { self.Scanner.GetPos() }

    pub fn Scan(&mut self) -> Result<&ast::Token, ParserError> {
        let bt = wrap_result!(ParserError::ScannerError, self.Scanner.Scan());

        // Semicolon complete: replace newline to semicolon
        match self.Token.Kind {
            ast::TokenKind::NEWLINE => {
                if self.CompleteSemicolon {
                    self.CompleteSemicolon = false;
                    self.Token = ast::Token {
                        Pos: bt.Pos,
                        Kind: ast::TokenKind::SEMICOLON,
                        Literal: String::from(";"),
                    };
                    ok!(&self.Token);
                }
            }
            _ => {}
        }

        self.CompleteSemicolon = false;

        let literal = bt.Literal.iter().collect::<String>();

        // Determines whether BasicToken is a keyword, operator or delimiter.
        let kind = match bt.Kind {
            BasicTokenKind::Ident => {
                match self.KeywordLookup.get(&literal) {
                    None => { ast::TokenKind::Ident }
                    Some(v) => { v.to_owned() }
                }
            }
            BasicTokenKind::Operator => {
                match self.KeywordLookup.get(&literal) {
                    None => { ast::TokenKind::Operator }
                    Some(v) => { v.to_owned() }
                }
            }
            BasicTokenKind::Delimiter => {
                let kind = self.KeywordLookup.get(&literal).expect("your scanner has design error");
                match kind {
                    ast::TokenKind::LPAREN => { self.QuoteStack.push(ast::TokenKind::RPAREN) }
                    ast::TokenKind::LBRACE => { self.QuoteStack.push(ast::TokenKind::RBRACE) }
                    ast::TokenKind::LBRACK => { self.QuoteStack.push(ast::TokenKind::RBRACK) }
                    _ => {}
                }
                kind.to_owned()
            }
            BasicTokenKind::Int(format) => { ast::TokenKind::Int(format) }
            BasicTokenKind::Float => { ast::TokenKind::Float } // TODO
            BasicTokenKind::String => { ast::TokenKind::String }
            BasicTokenKind::Char => { ast::TokenKind::Char }
            BasicTokenKind::Comment => { return self.Scan(); }
        };

        self.Token = ast::Token {
            Pos: bt.Pos,
            Kind: kind,
            Literal: bt.Literal.iter().collect(),
        };

        Ok(&self.Token)
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

    pub fn MatchTerm(&mut self, term: ast::TokenKind) -> Result<&ast::Token, ParserError> {
        let tok = self.Scan()?;
        if tag_matches!(&tok.Kind, &term) {
            self.Report(SyntaxError::UnexpectedNode(UnexpectedNodeError { Want: ast::Node::TokenKind(term), Have: ast::Node::TokenKind(tok.Kind.clone()) }));
        }
        Ok(tok)
    }
}

#[macro_export]
macro_rules! def_parser {
    (
        $(
        $ast_node:ty, $p:ident => $block:block
        ), *
    ) => {
        $(
        impl crate::parser::AstNodeParserTrait<$ast_node> for $ast_node {
            fn Expect($p: &mut crate::parser::Parser) -> Result<$ast_node, ParserError> { Ok($block) }
        }
        )*
    };
}
