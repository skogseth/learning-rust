#[macro_export]
macro_rules! add {
    ( $( $x:expr ),* ; $y:ty ) => {
        {
            let mut sum = 0;
            $(
                sum += $x as $y;
            )*
            sum
        }
    };
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