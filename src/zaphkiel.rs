#[macro_export]
macro_rules! generate_tests {
    ($($name:ident, $func:ident, $args:expr, $expected:expr);* $(;)?) => {
        $(#[test]
            fn $name() {
                let result = $func($args);
                assert_eq!(result, $expected, "result != expected");
            }
        )*
    };
}
