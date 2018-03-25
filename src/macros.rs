#[macro_export]
macro_rules! logf {
    (@single $($x:tt)*) => (());
    (@count $($rest:expr),*) => (<[()]>::len(&[$(logf!(@single $rest)),*]));

    ($($key:expr => $value:expr,)+) => { logf!($($key => $value),+) };
    ($($key:expr => $value:expr),*) => {
        {
            let cap = logf!(@count $($key),*);
            let mut map = ::std::collections::HashMap::with_capacity(cap);
            $(
                let _ = map.insert($key.into(), json!($value));
            )*
            map
        }
    };
}

