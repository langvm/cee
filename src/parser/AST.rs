// Copyright 2024 LangVM Project
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0
// that can be found in the LICENSE file and https://mozilla.org/MPL/2.0/.

use std::fmt::Display;

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
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, $fmt, $(self.$e,)*)
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
        impl Display for $node {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    $node::None => { write!(f, "") }
                    $(
                    $node::$typ(e) => { write!(f, "{}", e) }
                    )*
                }
            }
        }
        )*
    };
}

#[derive(Default)]
pub struct List<T> {
    pub Pos: PosRange,
    pub Elements: Vec<T>,
}

impl<T> Display for List<T> where T: Display {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for e in &self.Elements {
            write!(f, "{}", e)?;
        }
        Ok(())
    }
}

pub enum Node {
    None,
    Token(Token),
    TokenKind(TokenKind),
    Ident(Ident),
    Expr(Expr),
    Type(Type),
}

impl Default for Node { fn default() -> Self { Node::None } }

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Node::None => { f.write_fmt(::core::format_args!("")) }
            Node::Token(e) => { f.write_fmt(::core::format_args!("{}", e)) }
            Node::TokenKind(e) => { f.write_fmt(::core::format_args!("{}", e)) }
            Node::Ident(e) => { f.write_fmt(::core::format_args!("{}", e)) }
            Node::Expr(e) => { f.write_fmt(::core::format_args!("{}", e)) }
            Node::Type(e) => { f.write_fmt(::core::format_args!("{}", e)) }
        }
    }
}

def_node! {
    Type {
        FuncType,
        StructType,
        TraitType,
    },

    Expr {
        LiteralValue,
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

def_ast! {
    Ident ("{}", Token) {
        Token: Token,
    },
    
    LiteralValue ("{}", Token) {
        Token: Token,
    },

    // Type

    FuncType ("fun ({}) {}", Params, Result) {
        Params: List<Field>,
        Result: Type,
    },

    StructType ("fun ({}) {}", Name, FieldList) {
        Name: Ident,
        FieldList: List<Field>,
    },

    TraitType ("trait {}", Name){
        Name: Ident,
    },

    // Expression

    CallExpr ("fun ({}) {}", Callee, Params){
        Callee: Expr,
        Params: List<Expr>,
    },

    UnwrapExpr("{}?", Expr) {
        Expr: Expr,
    },

    // Declaration

    Field ("fun ({}) {}", Name, Type) {
        Name: Ident,
        Type: Type,
    },

    ImportDecl("fun ({}) {}", Alias, Canonical) {
        Alias: Ident,
        Canonical: Token,
    },

    FuncDecl ("fun ({}) {}", Name, Type){
        Name: Ident,
        Type: FuncType,
    },

    // Statement

    StmtBlock("fun ({}) {}", StmtList, Expr) {
        StmtList: List<Stmt>,
        Expr: Expr,
    }
}
