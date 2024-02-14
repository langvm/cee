// Copyright 2024 LangVM Project
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0
// that can be found in the LICENSE file and https://mozilla.org/MPL/2.0/.

use crate::scanner::Position::Position;

pub struct BufferScanner {
	pub Pos: Position,
	pub Buffer: Vec<char>,
}

pub struct EOFError {
	pub Pos: Position,
}

impl EOFError {
	pub fn Error(&self) -> String { format!("{}: end of file", self.Pos.to_string()) }
}

impl BufferScanner {
	pub fn GetChar(&self) -> Result<char, EOFError> {
		if self.Pos.Offset == self.Buffer.len() {
			return Err(EOFError {
				Pos: self.Pos.clone(),
			});
		}

		Ok(self.Buffer[self.Pos.Offset])
	}

	pub fn Move(&mut self) -> Result<char, EOFError> {
		let ch = self.GetChar()?;

		if ch == '\n' {
			self.Pos.Line += 1;
			self.Pos.Column = 0;
		} else {
			self.Pos.Column += 1;
		}
		self.Pos.Offset += 1;

		Ok(ch)
	}

	pub fn GotoNextLine(&mut self) -> Result<(), EOFError> {
		loop {
			match self.Move() {
				Ok(ch) => {
					if ch == '\n' {
						break;
					}
				}
				Err(err) => { return Err(err); }
			}
		}

		Ok(())
	}
}
