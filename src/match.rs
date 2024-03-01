// Copyright 2024 LangVM Project
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0
// that can be found in the LICENSE file and https://mozilla.org/MPL/2.0/.

#[macro_export]
macro_rules! cmp_enum_tag {
    (
        $e:expr,
        $($pattern:expr => $value:expr), *;
        _ => $default:expr
    ) => {
        match std::mem::discriminant($e) {
            $(
                it if it == std::mem::discriminant($pattern) => $value
            )*
            _ => $default
        }
    };
}

#[macro_export]
macro_rules! match_enum_tag {
    (
        $e:expr,
        $($pattern:expr => $value:expr), *;
        _ => $default:expr
    ) => {
        match std::mem::discriminant($e) {
            $(
                it if it == $pattern => $value
            )*
            _ => $default
        }
    };
}
