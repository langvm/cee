// Copyright 2024 LangVM Project
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0
// that can be found in the LICENSE file and https://mozilla.org/MPL/2.0/.

pub struct Position {
	pub Offset: usize,
	pub Line: usize,
	pub Column: usize,
}

impl Position {
	pub fn clone(&self) -> Position {
		return Position {
			Offset: self.Offset,
			Line: self.Line,
			Column: self.Column,
		};
	}

	pub fn ToString(&self) -> String { String::from(format!("{}:{}:{}", self.Offset, self.Line, self.Column)) }
}
