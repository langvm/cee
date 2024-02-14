// Copyright 2024 LangVM Project
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0
// that can be found in the LICENSE file and https://mozilla.org/MPL/2.0/.

use crate::parser::Token::Token;
use crate::scanner::Scanner::Scanner;

pub enum Error {}

impl Error {
	pub fn Error(self) -> String { format!("") }
}

pub struct UnexpectedNodeError {}

impl UnexpectedNodeError {
	pub fn Error(self) -> String { format!("") }
}

pub struct Parser {
	pub Scanner: Scanner,
	pub ReachedEOF: bool,
	pub Token: Token,
}

impl Parser {
	pub fn Scan(&mut self)->Result<(), Error> {
		Ok(())
	}
}
