// Copyright 2024 LangVM Project
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0
// that can be found in the LICENSE file and https://mozilla.org/MPL/2.0/.

use crate::Position::Position;

pub struct PosRange {
	pub Begin: Position,
	pub End: Position,
}

impl PosRange {
	pub fn ToString(&self) -> String { format!("{} -> {}", self.Begin.ToString(), self.End.ToString()) }
}
