#[macro_export]
macro_rules! add {
    ( $( $x:expr ),* ) => {
        {
            let mut sum = 0;
            $(
                sum += $x;
            )*
            sum
        }
    };
}