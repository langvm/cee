// Copyright 2024 LangVM Project
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0
// that can be found in the LICENSE file and https://mozilla.org/MPL/2.0/.

use crate::parser::{Parser, ParserError};

#[test]
fn TestParser_ExpectFuncDecl() {
    fn test() -> Result<(), ParserError> {
        let mut p = Parser::new(String::from(r#"
        fun Function() {
            return
        }
        "#).chars().collect());

        loop {
            p.Scan()?;
            println!("{}", p.Token);
        }
    }

    match test() {
        Ok(_) => {}
        Err(e) => {
            println!("{:?}", e)
        }
    }
}
