/// Finds the greatest common divisor
///
/// # Examples
///
/// ```
/// let a = 10;
/// let b = 15;
///
/// assert_eq!(math_utils::gcd(a, b), 5);
/// ```
///
pub fn gcd(mut a: u64, mut b: u64) -> u64 {
    assert!(a != 0 && b != 0);

    while b != 0 {
        if b < a {
            let t = b;
            b = a;
            a = t;
        }

        b = b % a;
    }
    a
}

#[cfg(test)]
mod tests {
    use ::*;
    #[test]
    fn test_gdc() {
        assert_eq!(gcd(14, 15), 1);

        assert_eq!(gcd(2 * 3 * 5 * 11 * 17, 3 * 7 * 11 * 13 * 19), 3 * 11);
    }
}
