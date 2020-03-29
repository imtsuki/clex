#[macro_export]
macro_rules! assert_debug_snapshot {
    ($left:expr, $right:expr) => {
        assert_eq!(format!("{:#?}", $left), $right.trim());
    };
}
