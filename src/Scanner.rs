// Copyright 2024 LangVM Project
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0
// that can be found in the LICENSE file and https://mozilla.org/MPL/2.0/.

use std::char::from_u32;

use crate::{atoi, string_vec};
use crate::BufferScanner::{BufferScanner, EOFError};
use crate::Position::Position;
use crate::PosRange::PosRange;
use crate::Scanner_errors::{Error, FormatError};
use crate::Token::Token;

pub struct Scanner {
	pub BufferScanner: BufferScanner,

	pub Delimiters: Vec<char>,
	pub Whitespaces: Vec<char>,
}

macro_rules! from_to {
    ($begin: expr, $vec: expr) => {
		$vec.BufferScanner.Buffer[$begin.Offset..$vec.GetPos().Offset].to_owned()
	};
}

impl Scanner {
	pub fn GetChar(&self) -> Result<char, Error> {
		match self.GetChar() {
			Ok(ch) => { Ok(ch) }
			Err(err) => { Err(Error::EOF(EOFError { Pos: self.GetPos() })) }
		}
	}

	pub fn Move(&mut self) -> Result<char, Error> {
		match self.Move() {
			Ok(ch) => { Ok(ch) }
			Err(err) => { Err(Error::EOF(EOFError { Pos: self.GetPos() })) }
		}
	}

	pub fn GotoNextLine(&mut self) -> Result<(), Error> {
		match self.BufferScanner.GotoNextLine() {
			Ok(_) => { Ok(()) }
			Err(err) => { Err(Error::EOF(err)) }
		}
	}

	pub fn GetPos(&self) -> Position { self.BufferScanner.Pos.clone() }

	pub fn SkipWhitespaces(&mut self) -> Result<(), Error> {
		loop {
			let ch = self.Move()?;
			for white in &self.Whitespaces {
				if ch != *white {
					return Ok(());
				}
			}
		}
	}

	pub fn ScanLineComment(&mut self) -> Result<Vec<char>, Error> {
		let begin = self.GetPos();

		self.GotoNextLine()?;

		Ok(from_to!(begin, self.GetPos(), self))
	}

	pub fn ScanQuotedComment(&mut self) -> Result<Vec<char>, Error> {
		let begin = self.GetPos();

		loop {
			if self.Move()? == '*' {
				if self.Move()? == '/' {
					break;
				}
			}
		}

		Ok(from_to!(begin, self.GetPos(), self))
	}

	pub fn ScanComment(&mut self) -> Result<Vec<char>, Error> {
		let begin = self.GetPos();

		return match self.Move()? {
			'/' => { self.ScanLineComment() }
			'*' => { self.ScanQuotedComment() }
			_ => {
				return Err(Error::Format(FormatError {
					PosRange: PosRange { Begin: begin, End: self.GetPos() },
				}));
			}
		};
	}

	pub fn ScanIdent(&mut self) -> Result<Vec<char>, Error> {
		let mut seq: Vec<char> = vec![];

		loop {
			let ch = self.GetChar()?;
			if ch.is_ascii_alphabetic() || ch.is_numeric() || ch == '_' {
				seq.push(ch);
				self.Move()?;
			} else {
				break;
			}
		}

		Ok(seq)
	}

	pub fn ScanHex(&mut self) -> Result<Vec<char>, Error> {
		let mut seq: Vec<char> = vec![];
		loop {
			let ch = self.Move()?;
			if '0' <= ch && ch <= '9' || 'a' <= ch && ch <= 'f' {
				seq.push(ch);
			} else {
				break;
			}
		}

		Ok(seq)
	}

	pub fn ScanDec(&mut self) -> Result<Vec<char>, Error> {
		let mut seq: Vec<char> = vec![];
		loop {
			let ch = self.Move()?;
			if '0' <= ch && ch <= '9' {
				seq.push(ch);
			} else {
				break;
			}
		}

		Ok(seq)
	}

	pub fn ScanOct(&mut self) -> Result<Vec<char>, Error> {
		let mut seq: Vec<char> = vec![];
		loop {
			let ch = self.Move()?;
			if '0' <= ch && ch <= '7' {
				seq.push(ch);
			} else {
				break;
			}
		}

		Ok(seq)
	}

	pub fn ScanBin(&mut self) -> Result<Vec<char>, Error> {
		let mut seq: Vec<char> = vec![];
		loop {
			let ch = self.Move()?;
			if ch == '0' || ch == '1' {
				seq.push(ch);
			} else {
				break;
			}
		}

		Ok(seq)
	}

	pub fn ScanDigit(&mut self) -> Result<Vec<char>, Error> {
		let begin = self.GetPos();

		let ch = self.Move()?;

		let seq = match ch {
			'0' => {
				self.Move()?;

				match self.Move()? {
					'x' => { self.ScanHex() }
					'o' => { self.ScanOct() }
					'b' => { self.ScanBin() }
					_ => {
						return Err(Error::Format(FormatError {
							PosRange: PosRange {
								Begin: begin,
								End: self.GetPos(),
							},
						}));
					}
				}
			}
			_ => { self.ScanDec() }
		}?;

		if seq.len() == 0 {
			return Err(Error::Format(FormatError {
				PosRange: PosRange { Begin: begin, End: self.GetPos() },
			}));
		}

		Ok(seq)
	}

	pub fn ScanUnicodeHex(&mut self, runesN: u8) -> Result<char, Error> {
		let begin = self.GetPos();

		let mut seq: Vec<char> = vec![];
		for i in 0..runesN {
			seq.push(self.Move()?);
		}

		let ch = match atoi!(u32, string_vec!(seq)) {
			Ok(ch) => {
				match from_u32(ch) {
					None => {
						return Err(Error::Format(FormatError {
							PosRange: PosRange { Begin: begin, End: self.GetPos() }
						}));
					}
					Some(ch) => { ch }
				}
			}
			Err(err) => {
				return Err(Error::Format(FormatError {
					PosRange: PosRange { Begin: begin, End: self.GetPos() },
				}));
			}
		};

		Ok(ch)
	}

	pub fn ScanEscapeChar(&mut self, quote: char) -> Result<char, Error> {
		let begin = self.GetPos();

		let ch = self.GetChar()?;

		Ok(match ch {
			'n' => { '\n' }
			't' => { '\t' }
			'r' => { '\r' }
			'\\' => { '\\' }
			'x' => { // 1 byte
				self.ScanUnicodeHex(2)?
			}
			'u' => { // 2 byte
				self.ScanUnicodeHex(4)?
			}
			'U' => { // 4 byte
				self.ScanUnicodeHex(8)?
			}
			_ if ch == quote => { quote }
			_ => {
				return Err(Error::Format(FormatError { PosRange: PosRange { Begin: begin, End: self.GetPos() } }));
			}
		})
	}

	pub fn ScanString(&mut self, quote: char) -> Result<Vec<char>, Error> {
		let begin = self.GetPos();

		self.Move()?;

		loop {
			let ch = self.Move()?;
			match ch {
				'\\' => {
					let esc = self.ScanEscapeChar(quote)?;
				}
				_ if ch == quote => {
					break;
				}
				_ => {}
			}
		}

		Ok(from_to!(begin, self))
	}

	pub fn ScanOperator(&mut self) -> Result<Vec<char>, Error> {
		let begin = self.GetPos();

		loop {
			let ch = self.GetChar()?;
			if !ch.is_ascii_punctuation() {
				break;
			}
		}

		Ok(from_to!(begin, self))
	}

	pub fn Scan(&mut self) -> Result<Token, Error> {
		self.SkipWhitespaces()?;

		let ch = self.GetChar()?;

		let seq =
			if ch.is_alphabetic() {
				self.ScanIdent()
			} else if ch.is_numeric() {
				self.ScanDigit()
			} else {
				match ch {
					'/' => { self.ScanComment() }
					_ => { return Err(Error::EOF(EOFError { Pos: self.GetPos() })); }
					// TODO
				}
				// TODO
			}?;

		Ok(Token { // TODO
			Pos: self.GetPos(),
			Kind: 0,
			Literal: "".to_string(),
		})
	}
}
