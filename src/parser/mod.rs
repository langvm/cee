// Copyright 2024 LangVM Project
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0
// that can be found in the LICENSE file and https://mozilla.org/MPL/2.0/.

pub mod Parser;
pub mod Token;
pub mod AST;
pub mod Diagnosis;
mod TestParser;

#[macro_export]
macro_rules! def_tokens {
    ($table_name:ident: $typ_name:ident=> { $($name:ident $literal:expr), * }) => {
        #[derive(Clone)]
        pub enum $typ_name {
            None,
            Ident,
            Operator,
            Int(crate::scanner::BasicToken::IntFormat),
            Float,
            String, 
            Char,
            $($name,)* 
        }
        
        impl Default for $typ_name {
            fn default() -> Self { Self::None }
        }

        pub static $table_name: std::collections::HashMap<String, $typ_name> = std::collections::HashMap::from([
            $(
            ($literal.to_string(), $typ_name::$name),
            )*
        ]);
    };
}
