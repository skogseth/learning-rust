#[macro_export]
macro_rules! pipes {
    ( $var:expr $( => $function:ident )* ) => {
        {
            let temp = $var;
            $( let temp = $function(temp); )*
            temp
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn single() {
        let a = pipes!{ 4 + 3 + 2 + 1 };
        assert_eq!(a, 10);
    }

    #[test]
    fn simple() {
        let a = pipes!{ 4 => square };
        assert_eq!(a, 16);
    }

    #[test]
    fn double() {
        let a = pipes!{ 4 => add_one => square };
        assert_eq!(a, 25);
    }

    #[test]
    fn type_changing() {
        let a = pipes!{ 3 + 2 => square => half };
        assert_eq!(a, 12.5);
    }

    fn add_one(i: i32) -> i32 {
        i + 1
    }

    fn square(i: i32) -> i32 {
        i * i
    }

    fn half(i: i32) -> f32 {
        (i as f32) / 2.
    }
}

