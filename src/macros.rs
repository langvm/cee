// Copyright 2024 LangVM Project
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0
// that can be found in the LICENSE file and https://mozilla.org/MPL/2.0/.

#[macro_export]
macro_rules! string_vec {
    ($e: expr) => { String::from_iter($e)};
}

#[macro_export]
macro_rules! atoi {
    ($t: ty, $str: expr) => {
		($str).parse::<$t>()
	};
}
