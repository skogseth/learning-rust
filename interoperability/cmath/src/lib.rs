#![allow(dead_code)]

use cty::c_int;

extern {
    pub fn multiply(a: c_int, b: c_int) -> c_int;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn multiply_two_numbers() {
        let a = 3;
        let b = 4;
        let c = unsafe { multiply(a, b) };
        assert_eq!(c, 12);
    }
}