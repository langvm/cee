// Copyright 2024 LangVM Project
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0
// that can be found in the LICENSE file and https://mozilla.org/MPL/2.0/.

use crate::{def_parser, tag_matches};
use crate::ast::*;
use crate::parser::*;

macro_rules! range {
    ($begin: expr, $self: expr) => {
        crate::scanner::PosRange { Begin: $begin, End: $self.GetPos() }
    };
}

macro_rules! match_terms {
    ($p:expr, $($token:expr), *) => {
        $(
        $p.MatchTerm($token)?;
        )*
    };
}

impl<T> List<T> where T: AstNodeParserTrait<T> {
    pub fn Expect(p: &mut Parser, delimiter: TokenKind, term: TokenKind) -> Result<List<T>, ParserError> {
        let begin = p.GetPos();

        let mut list: Vec<T> = vec![];
        loop {
            list.push(T::Expect(p)?);
            match &p.Token.Kind {
                it if tag_matches!(it, &delimiter) => {
                    p.Scan()?;
                    match &p.Token.Kind {
                        it if tag_matches!(it, &term) => { break; }
                        _ => {}
                    }
                }
                it if tag_matches!(it, &term) => { break; }
                _ => {}
            }
        }
        Ok(List {
            Pos: range![begin, p],
            Elements: list,
            Delimiter: delimiter,
            Term: term,
        })
    }
}

def_parser! {
    Ident, p => {
        match p.Token.Kind {
            TokenKind::Ident => {
                if p.NamespaceIdents.contains_key(&p.Token.Literal) {}
                Ident { Pos: p.Token.Pos, Token: p.Token.clone() }
            }
            _ => {
                p.ReportAndRecover(SyntaxError::UnexpectedNode(UnexpectedNodeError { Want: Default::default(), Have: Default::default() }))?;
                Ident::default()
            }
        }
    },
    
    Field, p => {
        let begin = p.GetPos();
        
        Field {
            Names: List::Expect(p, TokenKind::COMMA, TokenKind::None)?,
            Type: Type::Expect(p)?,
            Pos: range![begin, p],
        }
    },

    Type, p => {
        match p.Token.Kind {
            TokenKind::Ident => {  }
            TokenKind::STRUCT => { Type::StructType(Box::new(StructType::Expect(p)?)) }
            TokenKind::TRAIT => { Type::TraitType(Box::new(TraitType::Expect(p)?)) }
            TokenKind::FUNC => {  }
            _ => {
                p.ReportAndRecover(SyntaxError::UnexpectedNode(UnexpectedNodeError { Want: todo!(), Have: todo!() }))?;
                Type::None
            }
        }
    },

    FuncType, p => {
        let begin = p.GetPos();

        p.MatchTerm(TokenKind::LPAREN)?;

        let params = List::Expect(p,TokenKind::COMMA, TokenKind::RPAREN)?;

        match p.Token.Kind {
            TokenKind::PASS => {
                FuncType {
                    Params: params,
                    Result: Type::Expect(p)?,
                    Pos: range![begin, p],
                }
            }
            _ => {
                FuncType {
                    Params: params,
                    Result: Type::None,
                    Pos: range![begin, p],
                }
            }
        }
    },

    StructType, p => {
        let begin = p.GetPos();

        match_terms![p, TokenKind::STRUCT, TokenKind::LBRACE];

        StructType {
            Name: Ident::Expect(p)?,
            FieldList: List::Expect(p, TokenKind::SEMICOLON, TokenKind::RBRACE)?,
            Pos: range![begin, p],
        }
    },

    TraitType, p => {
        let begin = p.GetPos();

        p.MatchTerm(TokenKind::TRAIT)?;

        let name = Ident::Expect(p)?;

        TraitType {
            Name: name,
            Pos: range![begin, p],
        }
    },
    
    ImportDecl, p => {
        let begin = p.GetPos();

        p.MatchTerm(TokenKind::IMPORT)?;
        ImportDecl {
            Alias: Ident::Expect(p)?,
            Canonical: p.MatchTerm(TokenKind::String)?.clone(),
            Pos: range![begin, p],
        }
    },
    
    FuncDecl, p => {
        let begin = p.GetPos();

        p.MatchTerm(TokenKind::FUNC)?;

        let name = Ident::Expect(p)?;

        p.MatchTerm(TokenKind::LPAREN)?;

        let params = match p.Token.Kind {
            TokenKind::RPAREN => {
                List { Pos: range![begin, p], Elements: vec![], Delimiter: TokenKind::None, Term: TokenKind::RPAREN }
            }
            _ => {
                List::Expect(p, TokenKind::COMMA, TokenKind::RPAREN)?
            }
        };

        let typ = match p.Token.Kind {
            TokenKind::PASS => {
                p.Scan()?;
                FuncType {
                    Params: params,
                    Result: Type::Expect(p)?,
                    Pos: range![begin, p],
                }
            }
            _ => {
                FuncType {
                    Params: params,
                    Result: Type::None,
                    Pos: range![begin, p],
                }
            }
        };

        FuncDecl {
            Name: Optional::Some(name),
            Type: typ,
            Stmt: Optional::None,
            Pos: range![begin, p],
        }
    },
    
    Expr, p => {
        match p.Token.Kind {
            TokenKind::Ident => { todo!() }
            TokenKind::Int(_) | TokenKind::Float | TokenKind::Char | TokenKind::String => {
                Expr::LiteralValue(Box::new(LiteralValue {
                    Pos: p.Token.Pos,
                    Token: p.Token.clone(),
                }))
            }
            _ => { todo!() }
        }
    },
    
    Stmt, p => {
        let expr = Expr::Expect(p)?;

        match p.Token.Kind {
            TokenKind::MUT => {
                Stmt::MutDecl(Box::from(MutDecl::Expect(p)?))
            }
            TokenKind::VAL => {
                Stmt::MutDecl(Box::from(MutDecl::Expect(p)?))
            }
            _ => {
                Stmt::Expr(Box::from(Expr::Expect(p)?))
            }
        }
    },
    
    MutDecl, p => {
        let begin = p.GetPos();

        p.MatchTerm(TokenKind::MUT)?;

        MutDecl {
            Name: Ident::Expect(p)?,
            Type: Type::Expect(p)?,
            Pos: range![begin, p],
        }
    },
    
    StmtBlock, p => {
        let begin = p.GetPos();

        StmtBlock {
            StmtList: List::Expect(p, TokenKind::SEMICOLON, TokenKind::None)?,
            Type: Type::None, // TODO
            Pos: range![begin, p],
        }
    }
}
