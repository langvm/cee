// Copyright 2024 LangVM Project
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0
// that can be found in the LICENSE file and https://mozilla.org/MPL/2.0/.

use std::fmt;
use std::fmt::Formatter;

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
}

impl fmt::Display for PosRange {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result { write!(f, "{} -> {}", self.Begin, self.End) }
}
