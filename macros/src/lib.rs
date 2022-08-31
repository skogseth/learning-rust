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

#[macro_export]
macro_rules! railway {
    ( $function:ident($result:expr) ) => {
        match $result {
            Ok(val) => $function(val),
            Err(e) => Err(e),
        }
    }
}