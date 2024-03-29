// Copyright 2024 LangVM Project
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0
// that can be found in the LICENSE file and https://mozilla.org/MPL/2.0/.

use crate::parser::Parser::{Parser, ParserError};

#[test]
fn TestParserSemicolonComplete() {
    fn test() -> Result<(), ParserError> {
        let mut p = Parser::new(String::from("Abc").chars().collect());

        while !&p.ReachedEOF {
            p.Scan()?;
            println!("{}", String::from_iter(&p.Token.Literal));
        }

        Ok(())
    }

    match test() {
        Ok(_) => {}
        Err(e) => {
            println!("{}", e)
        }
    }
}
