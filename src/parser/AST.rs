// Copyright 2024 LangVM Project
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0
// that can be found in the LICENSE file and https://mozilla.org/MPL/2.0/.

use std::fmt::{Display, Formatter};

use crate::parser::Token::{Token, TokenKind};
use crate::scanner::PosRange::PosRange;

macro_rules! def_ast {
    (
        $(
        $ast:ident ($fmt:expr, $($e:ident), *) {
            $($name:ident: $typ:ty), *,
        }
        ), *
    ) => {
        $(
        #[derive(Default)]
        pub struct $ast { 
            pub Pos: PosRange,
            $(
            pub $name: $typ,    
            )*
        }
        impl Display for $ast {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                write!(f, $fmt, $(self.$e)*)
            }
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
        impl Default for $node {
            fn default() -> Self {
                $node::None
            }
        }
        )*
    };
}

pub struct List<T> {
    Lists: Vec<T>,
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
        CallExpr,
        UnwrapExpr,
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

def_ast! {
    Ident ("{}", Token) {
        Token: Token,
    },

    // Type

    FuncType ("fun ({}) {}", Params, Result) {
        Params: FieldList,
        Result: Type,
    },

    StructType ("fun ({}) {}", Name, FieldList) {
        Name: Ident,
        FieldList: FieldList,
    },

    TraitType ("trait {}", Name){
        Name: Ident,
    },

    // Expression

    ExprList ("fun ({})", ExprList){
        ExprList: Vec<Expr>,
    },

    CallExpr ("fun ({}) {}", Params, Result){
        Callee: Expr,
        Params: ExprList,
    },

    UnwrapExpr("fun ({}) {}", Params, Result) {
        Expr: Expr,
    },

    // Declaration

    Field ("fun ({}) {}", Params, Result) {
        Name: Ident,
        Type: Type,
    },

    FieldList ("fun ({}) {}", Params, Result) {
        FieldList: Vec<Field>,
    },

    ImportDecl("fun ({}) {}", Params, Result) {
        Alias: Option<Ident>,
        Canonical: Token,
    },

    FuncDecl ("fun ({}) {}", Params, Result){
        Name: Ident,
        Type: FuncType,
    },

    // Statement

    StmtBlock("fun ({}) {}", Params, Result) {
        StmtList: Vec<Stmt>,
        Expr: Expr,
    }
}

macro_rules! def_rule {
    () => {};
}
