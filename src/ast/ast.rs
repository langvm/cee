// Copyright 2024 LangVM Project
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0
// that can be found in the LICENSE file and https://mozilla.org/MPL/2.0/.

use std::fmt::{Display, Formatter, Pointer};

use crate::parser::{Token, TokenKind};
use crate::scanner::*;

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
            Node::TokenKind(e) => { f.write_fmt(::core::format_args!("{:?}", e)) }
            Node::Ident(e) => { f.write_fmt(::core::format_args!("{}", e)) }
            Node::Expr(e) => { f.write_fmt(::core::format_args!("{}", e)) }
            Node::Type(e) => { f.write_fmt(::core::format_args!("{}", e)) }
        }
    }
}

pub enum Optional<T> {
    Some(T),
    None,
}

impl<T> Display for Optional<T> where T: Display {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Optional::Some(v) => { write!(f, "{}", v) }
            Optional::None => { Ok(()) }
        }
    }
}

def_ast! {
    Ident ("{}", Token) {
        Token: Token,
    }
}

def_node! {
    Expr {
        LiteralValue,
        CallExpr,
        UnwrapExpr,
    }
}

def_ast! {
    LiteralValue ("{}", Token) {
        Token: Token,
    },

    CallExpr ("fun ({}) {}", Callee, Params) {
        Callee: Expr,
        Params: List<Expr>,
    },

    UnwrapExpr("{}?", Expr) {
        Expr: Expr,
    }
}

def_node! {
    Type {
        FuncType,
        StructType,
        TraitType,
    }
}

def_ast! {
    FuncType ("fun ({}) {}", Params, Result) {
        Params: List<Field>,
        Result: Type,
    },

    StructType ("fun ({}) {}", Name, FieldList) {
        Name: Ident,
        FieldList: List<Field>,
    },

    TraitType ("trait {}", Name) {
        Name: Ident,
    }
}

def_node! {
    Stmt {
        MutDecl,
        FuncDecl,
        StmtBlock,
        Expr,
    }
}

def_ast! {
    Field ("{}: {}", Name, Type) {
        Name: Ident,
        Type: Type,
    },

    ImportDecl("import ({}) {}", Alias, Canonical) {
        Alias: Ident,
        Canonical: Token,
    },

    FuncDecl ("fun ({}) {}", Name, Type) {
        Name: Ident,
        Type: FuncType,
    },

    MutDecl ("mut {}: {}", Name, Type) {
        Name: Ident,
        Type: Type,
    }
}

def_ast! {
    StmtBlock("fun ({}) {}", StmtList, Expr) {
        StmtList: List<Stmt>,
        Expr: Expr,
    }
}
