// Copyright 2024 LangVM Project
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0
// that can be found in the LICENSE file and https://mozilla.org/MPL/2.0/.


pub enum ParserError {}

impl ParserError {
	pub fn Error(self) -> String { format!("") }
}

pub struct UnexpectedNodeError {}

impl UnexpectedNodeError {
	pub fn Error(self) -> String { format!("") }
}
