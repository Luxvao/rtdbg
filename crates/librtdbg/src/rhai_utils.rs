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

// Same usage as above, just with different parameters
#[macro_export]
macro_rules! register_const {
    ($scope:expr, {$($name:expr => $value:expr),+}) => {
        $(
            $scope.push_constant($name, $value);
        )+
    };
}
