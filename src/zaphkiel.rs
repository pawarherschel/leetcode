#[macro_export]
macro_rules! generate_tests {
    ($map:expr;
    $($name:ident, $func:ident, $args:expr, $expected:expr);* $(;)?) => {
        $(#[test]
            fn $name() {
                let (result, duration) = time_it!(stringify!($name) => $func($args));
                assert_eq!(result, $expected);
                $map.write().unwrap().insert(stringify!($name).to_string(), duration);
            }
        )*
    };
}

#[macro_export]
macro_rules! time_it {
    ($comment:expr => $stmt:stmt) => {{
        let start = std::time::Instant::now();
        let result = { $stmt };
        let duration = start.elapsed();
        println!("{} => {:?}", $comment, duration);
        (result, duration)
    }};
}
