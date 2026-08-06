/*
    Usage:
    register_fns!(engine {
        "name" => handler,
        ...
    })
*/

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
macro_rules! register_enums {
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
        paste! {
            #[export_module]
            pub mod [<$enum:lower _module>] {
                $(
                    #[allow(non_upper_case_globals)]
                    pub const $variant: $enum = <$enum>::$variant;
                )*
            }
        }
    };
}
