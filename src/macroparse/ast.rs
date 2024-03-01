// Copyright 2024 LangVM Project
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0
// that can be found in the LICENSE file and https://mozilla.org/MPL/2.0/.

#[macro_export]
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

#[macro_export]
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
