#[inline]
pub fn iter(n: u128) -> u128 {
    (1..n + 1).rev().product()
}

#[inline]
pub fn looping(n: u128) -> u128 {
    let mut acc = n;
    for i in (1..n).rev() {
        acc *= i;
    }
    acc
}

#[inline]
pub fn recursive(n: u128) -> u128 {
    if n == 0 {
        1
    } else {
        n * recursive(n - 1)
    }
}

#[inline]
pub fn recursive_with_tail(n: u128) -> u128 {
    tail(n, n-1)
}

#[inline]
fn tail(acc: u128, n: u128) -> u128 {
    if n == 0 {
        acc
    } else {
        tail(n * acc, n - 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEN: u128 = 3628800;
    
    #[test]
    fn test_iter() {
        assert_eq!(iter(10), TEN);
    }

    #[test]
    fn test_looping() {
        assert_eq!(looping(10), TEN);
    }

    #[test]
    fn test_recursive() {
        assert_eq!(recursive(10), TEN);
    }

    #[test]
    fn test_recursive_with_tail() {
        assert_eq!(recursive_with_tail(10), TEN);
    }
}
