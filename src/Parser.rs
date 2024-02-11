// Copyright 2024 LangVM Project
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0
// that can be found in the LICENSE file and https://mozilla.org/MPL/2.0/.

use crate::ErrParser::ParserError;
use crate::Scanner::Scanner;
use crate::Token::Token;

pub struct Parser {
	pub Scanner: Scanner,
	pub ReachedEOF: bool,
	pub Token: Token,
}

impl Parser {
	pub fn Scan(&mut self)->Result<(), ParserError> {
		Ok(())
	}
}
