#[macro_export]
macro_rules! add {
    ( $( $x:expr ),+ ; $y:ty ) => {
        {
            0
            $(
                + $x as $y
            )*
        }
    };
    ( $( $x:expr ),+ ) => {
        {
            0
            $(
                + $x
            )*
        }
    };
}