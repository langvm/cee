// Copyright 2024 LangVM Project
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0
// that can be found in the LICENSE file and https://mozilla.org/MPL/2.0/.

use crate::peg::Token::{Token, TokenKind};
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

pub enum Node {
    None,
    Token(Token),
    TokenKind(TokenKind),
    Ident(Ident),
    Rule(Rule),
}

def_ast! {
    Ident  {
        Token: Token,
    },
    
    Pattern {
        Elements: Vec<Box<Node>>,
    },
    
    Rule {
        Name: Ident,
        Patterns: Vec<Pattern>,
    }
}
