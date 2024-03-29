// Copyright 2024 LangVM Project
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0
// that can be found in the LICENSE file and https://mozilla.org/MPL/2.0/.

#![allow(nonstandard_style)]

pub mod parser;
pub mod scanner;
pub mod types;

#[macro_export]
macro_rules! tag_matches {
    ($e:expr, $p:expr) => {
        std::mem::discriminant($e) == std::mem::discriminant($p)
    };
}

#[macro_export]
macro_rules! no_display {
    ($typ:ty) => {
        impl std::fmt::Display for $typ {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result { Ok(()) }
        }
    };
}
