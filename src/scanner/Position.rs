// Copyright 2024 LangVM Project
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0
// that can be found in the LICENSE file and https://mozilla.org/MPL/2.0/.

use std::fmt;
use std::fmt::Formatter;

#[derive(Debug)]
pub struct Position {
    pub Offset: usize,
    pub Line: usize,
    pub Column: usize,
}

impl Default for Position {
    fn default() -> Self {
        Position {
            Offset: 0,
            Line: 0,
            Column: 0,
        }
    }
}

impl Position {
    pub fn clone(&self) -> Position {
        return Position {
            Offset: self.Offset,
            Line: self.Line,
            Column: self.Column,
        };
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result { write!(f, "{:x}:{}:{}", self.Offset, self.Line, self.Column) }
}
