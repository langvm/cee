// Copyright 2024 LangVM Project
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0
// that can be found in the LICENSE file and https://mozilla.org/MPL/2.0/.

use crate::scanner::BufferScanner::BufferScanner;
use crate::scanner::Position::Position;
use crate::scanner::Scanner::Scanner;

/*

package main
var i = len("String for testing."+"")
i++
if i != 127 | 0b01 && i == '1' {
	println("String for testing.\nChinese letter: \u554a")
}
*/

#[test]
fn TestScanner() {
    let src = r#"package main	var i = len("String for testing."+"")	i++	if i != 127 | 0b01 && i == '1' {		println("String for testing.\nChinese letter: \u554a")	}"#;

    let mut s = Scanner {
        BufferScanner: BufferScanner {
            Pos: Position { Offset: 0, Line: 0, Column: 0 },
            Buffer: src.chars().collect(),
        },
        Delimiters: vec![',', '(', ')', '[', ']', '{', '}'],
        Whitespaces: vec![' ', '\r', '\t'],
    };
    loop {
        match s.Scan() {
            Ok(tok) => {
                println!("{}\n{}", tok.Pos.to_string(), tok.Literal.iter().collect::<String>())
            }
            Err(err) => {
                println!("{}", err.Error());
                break;
            }
        };
    }
}
