pub use crate::parser::{Alias, Dialog, Line, Mention, Requirements, Section, State};

pub use crate::player::{Cursor, DialogState};

#[macro_export]
macro_rules! const_expr_count {
    () => (0);
    ($e:expr) => (1);
    ($e:expr; $($other_e:expr);*) => ({
        1 $(+ $crate::const_expr_count!($other_e) )*
    });

    ($e:expr; $($other_e:expr);* ; ) => (
        $crate::const_expr_count! { $e; $($other_e);* }
    );
}

#[macro_export]
macro_rules! character_requirements {
    ($($key:expr => [$($val:expr),*]),*) => ({
        let start_capacity = $crate::const_expr_count!($($key);*);
        #[allow(unused_mut)]
        let mut map = ::std::collections::HashMap::with_capacity(start_capacity);
        $(
            map.insert(::dlg::prelude::Alias($key.to_string()), ::dlg::prelude::Requirements{
                states: {
                    let vec_capacity = $crate::const_expr_count!($($val);*);
                    #[allow(unused_mut)]
                    let mut v = ::std::vec::Vec::with_capacity(vec_capacity);
                    $(
                        v.push(::dlg::prelude::State::Named($val.to_string()));
                    )*

                    v
                }
            });
        )*
        map
    });
}
