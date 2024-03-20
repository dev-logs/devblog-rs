#[macro_export]
macro_rules! include_js {
    ($($tt:tt)*) => {
        stringify!({
            $($tt)*
        })
    };
}
