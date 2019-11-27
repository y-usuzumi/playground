#[macro_export]
macro_rules! count_exprs {
    () => {
        0
    };
    // Note the following pattern is required because the last pattern will only match with a "comma"
    ($head:expr) => {
        1
    };
    ($head:expr, $($tail:expr),*) => {
        1 + count_exprs!($($tail),*)
    };
}

#[macro_export]
macro_rules! recurrence {
    ($name:ident [$idx:ident] : $sty:ty = $($inits:expr),+ , ... , $recur:expr) => {{
        use std::ops::Index;

        struct Recurrence {
            mem: [$sty; count_exprs!($($inits),+)],
            pos: usize,
        }
    }};
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        recurrence![x[y]: u32 = 0, 1 , ... , x[y-1] + x[y-2]];
    }
}
