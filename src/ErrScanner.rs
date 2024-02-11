// Copyright 2024 LangVM Project
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0
// that can be found in the LICENSE file and https://mozilla.org/MPL/2.0/.

use crate::BufferScanner::EOFError;

pub enum Error {
	EOF(EOFError),
	Format(FormatError),
}

impl Error {
	pub fn Error(self) -> String {
		match self {
			Error::EOF(err) => { err.Error() }
			Error::Format(err) => { err.Error() }
		}
	}
}

pub struct FormatError {
	pub PosRange: crate::PosRange::PosRange,
}

impl FormatError {
	fn Error(&self) -> String { format!("{}: format error", self.PosRange.ToString()) }
}
