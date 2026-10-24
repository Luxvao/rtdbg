/*
    Usage:
    register_fns!(engine {
        "name" => handler,
        ...
    })
*/

use std::{any::type_name, collections::HashMap};

use rhai::{Array, EvalAltResult, Map};

#[macro_export]
macro_rules! register_fns {
    ($engine:expr, {$($name:expr => $handler:ident),+}) => {
        $(
            $engine.register_fn($name, $handler);
        )+
    };
}

#[macro_export]
macro_rules! register_types {
    ($engine:expr, {$($type:ident),+}) => {
        $(
            $engine.build_type::<$type>();
        )+
    };
}

#[macro_export]
macro_rules! register_modules {
    ($engine:expr, {$($enum:ident),+}) => {
        $(
            paste! {
                $engine.register_type_with_name::<$enum>(stringify!($enum))
                    .register_static_module(stringify!($enum), exported_module!([<$enum:lower _module>]).into());
            }
        )+
    };
}

// Same usage as above, just with different parameters
#[macro_export]
macro_rules! register_const {
    ($scope:expr, {$($name:expr => $value:expr),+}) => {
        $(
            $scope.push_constant($name, $value);
        )+
    };
}

// Macro to create enum plugins (from the rhai docs)
#[macro_export]
macro_rules! create_enum_module {
    ($enum:ident => $($variant:ident),+) => {
        paste::paste! {
            #[rhai::export_module]
            pub mod [<$enum:lower _module>] {
                $(
                    #[allow(non_upper_case_globals)]
                    pub const $variant: $enum = <$enum>::$variant;
                )*
            }
        }
    };
}

pub fn parse_array<T: 'static>(value: Array) -> Option<Vec<T>> {
    value
        .into_iter()
        .map(|v| v.try_cast::<T>())
        .collect::<Option<Vec<T>>>()
}

pub fn parse_map<T: 'static>(value: Map) -> Option<HashMap<String, T>> {
    value
        .into_iter()
        .map(|(v, k)| Some((v.into(), k.try_cast::<T>()?)))
        .collect::<Option<HashMap<String, T>>>()
}
