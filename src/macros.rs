#[macro_export]
macro_rules! s(
    ($e:expr) => {{
        let s: &'static str = $e;
        String::from(s)
    }}
);
