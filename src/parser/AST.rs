// Copyright 2024 LangVM Project
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0
// that can be found in the LICENSE file and https://mozilla.org/MPL/2.0/.

use std::fmt::{Display, Formatter};
use crate::parser::Token::{Token, TokenKind};
use crate::scanner::PosRange::PosRange;

pub enum Node {
    None,
    Token(TokenKind),
    Ident,
    Expr(Expr),
    Type(Type),
}

impl Display for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

pub struct Ident {
    pub Pos: PosRange,
    pub Token: Token,
}

pub struct Field {
    pub Pos: PosRange,
    pub Name: Ident,
    pub Type: Type,
}

pub struct FieldList {
    pub Pos: PosRange,
    pub FieldList: Vec<Field>,
}

pub enum Type {
    None,
    Func(Box<FuncType>),
    Struct(Box<StructType>),
    Trait(Box<TraitType>),
}

pub struct FuncType {
    pub Pos: PosRange,
}

pub struct StructType {
    pub Pos: PosRange,
    pub Name: Ident,
    pub FieldList: FieldList,
}

pub struct TraitType {
    pub Pos: PosRange,
    pub Name: Ident,
}

pub enum Expr {
    None,
    Call(Box<CallExpr>),
    Unwrap(Box<UnwrapExpr>),
}

pub struct ExprList{
    pub Pos: PosRange,
    pub ExprList: Vec<Expr>,
}

pub struct CallExpr {
    pub Pos: PosRange,
    pub Callee: Expr,
    pub Params: ExprList,
}

pub struct UnwrapExpr {
    pub Pos: PosRange,
    pub Expr: Expr,
}

pub enum Decl {
    None,
    Func(Box<FuncDecl>),
}

pub struct ImportDecl {
    pub Pos: PosRange,
    pub Alias: Ident,
    pub Canonical: Token,
}

pub struct FuncDecl {
    pub Pos: PosRange,
    pub Name: Ident,
    pub Type: FuncType,
}

pub enum Stmt {
    None,
    StmtBlock(StmtBlock),
    Expr(Expr),
}

pub struct StmtList {
    pub Pos: PosRange,
    pub StmtList: Vec<Stmt>,
}

pub struct StmtBlock {
    pub Pos: PosRange,
    pub StmtList: StmtList,
    pub Expr: Expr,
}
