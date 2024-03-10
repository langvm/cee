// Copyright 2024 LangVM Project
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0
// that can be found in the LICENSE file and https://mozilla.org/MPL/2.0/.

use std::fmt::{Display, Formatter};

use crate::parser::Token::{Token, TokenKind};
use crate::scanner::PosRange::PosRange;

macro_rules! def_ast {
    (
        $(
            $ast:ident {
                $($name:ident: $typ:ty), *,
            }
        ), *
    ) => {
        $(
            pub struct $ast {
                pub Pos: PosRange,
                $(
                    pub $name: $typ,    
                )*
            }
        )*
    };
}

macro_rules! def_node {
    (
        $(
            $node:ident {
                $($typ:ident), *,
            }
        ), *
    ) => {
        $(
            pub enum $node {
                None,
                $(
                    $typ(Box<$typ>),
                )*
            }
        )*
    };
}

pub enum Node {
    None,
    Token(Token),
    TokenKind(TokenKind),
    Ident(Ident),
    Expr(Expr),
    Type(Type),
}

def_node! {
    Type {
        FuncType,
        StructType,
        TraitType,
    },

    Expr {
        CallExpr,
        UnwrapExpr,
    },

    Decl {
        FuncDecl,
    },

    Stmt {
        StmtBlock,
        Expr,
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

def_ast! {
    Ident  {
        Token: Token,
    },

    // Type

    FuncType {
        Params: FieldList,
        Result: Type,
    },

    StructType {
        Name: Ident,
        FieldList: FieldList,
    },

    TraitType {
        Name: Ident,
    },

    // Expression

    ExprList {
        ExprList: Vec<Expr>,
    },

    CallExpr {
        Callee: Expr,
        Params: ExprList,
    },

    UnwrapExpr {
        Expr: Expr,
    },

    // Declaration

    Field  {
        Name: Ident,
        Type: Type,
    },

    FieldList  {
        FieldList: Vec<Field>,
    },

    ImportDecl {
        Alias: Option<Ident>,
        Canonical: Token,
    },

    FuncDecl {
        Name: Ident,
        Type: FuncType,
    },

    // Statement

    StmtBlock {
        StmtList: Vec<Stmt>,
        Expr: Expr,
    }
}

macro_rules! def_rule {
    () => {};
}
