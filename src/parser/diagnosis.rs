// Copyright 2024 LangVM Project
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0
// that can be found in the LICENSE file and https://mozilla.org/MPL/2.0/.

use std::fmt;
use std::fmt::Formatter;

use crate::ast::Node;

pub enum SyntaxError {
    UnexpectedNode(UnexpectedNodeError)
}

pub struct UnexpectedNodeError {
    pub Want: Node,
    pub Have: Node,
}

impl fmt::Debug for UnexpectedNodeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result { write!(f, "unexpected node: {} but have {}", self.Have, self.Want) }
}
