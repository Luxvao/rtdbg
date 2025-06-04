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
