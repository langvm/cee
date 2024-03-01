// Copyright 2024 LangVM Project
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0
// that can be found in the LICENSE file and https://mozilla.org/MPL/2.0/.

#[macro_export]
macro_rules! def_rule {
    (
        $self:expr,
        $ast: ty,
        $($field:ident: $f:ident), *;
    ) => {{
        let begin = $self.GetPos();
        
        $ast {
            $(
                $field: $self.$f()?,
            )*
            Pos: begin_end!(begin, $self),
        }
    }};
}

#[macro_export]
macro_rules!  {
    () => {};
}
