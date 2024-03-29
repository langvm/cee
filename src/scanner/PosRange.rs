// Copyright 2024 LangVM Project
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0
// that can be found in the LICENSE file and https://mozilla.org/MPL/2.0/.

use crate::scanner::Position::Position;

#[derive(Clone, Default)]
pub struct PosRange {
    pub Begin: Position,
    pub End: Position,
}

impl std::fmt::Display for PosRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "{} -> {}", self.Begin, self.End) }
}
