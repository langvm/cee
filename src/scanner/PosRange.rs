// Copyright 2024 LangVM Project
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0
// that can be found in the LICENSE file and https://mozilla.org/MPL/2.0/.

use crate::scanner::Position::Position;

pub struct PosRange {
    pub Begin: Position,
    pub End: Position,
}

impl PosRange {
    pub fn clone(&self) -> PosRange {
        PosRange {
			Begin: self.Begin.clone(),
			End: self.End.clone(),
		}
    }

    pub fn to_string(&self) -> String { format!("{} -> {}", self.Begin.to_string(), self.End.to_string()) }
}
