/// Converts a JavaScript number to an i64.
/// The number must be in the safe integer range, i.e. in
/// [Number.MIN_SAFE_INTEGER, Number.MAX_SAFE_INTEGER].
pub fn to_safe_integer(value: f64) -> Option<i64> {
    const MIN_SAFE_INTEGER: i64 = -9007199254740991; // Number.MIN_SAFE_INTEGER
    const MAX_SAFE_INTEGER: i64 = 9007199254740991; // Number.MAX_SAFE_INTEGER
    let converted = value as i64;
    if value != (converted as f64) {
        return None; // Not an integer
    }
    if !(MIN_SAFE_INTEGER..=MAX_SAFE_INTEGER).contains(&converted) {
        return None;
    }
    Some(converted)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_safe_integer_works() {
        let a = to_safe_integer(0.0);
        assert_eq!(a, Some(0));
        let a = to_safe_integer(100.0);
        assert_eq!(a, Some(100));
        let a = to_safe_integer(-3.0);
        assert_eq!(a, Some(-3));
        let a = to_safe_integer(-9007199254740991.0);
        assert_eq!(a, Some(-9007199254740991));
        let a = to_safe_integer(9007199254740991.0);
        assert_eq!(a, Some(9007199254740991));

        // too small
        let a = to_safe_integer(-9007199254740992.0);
        assert_eq!(a, None);
        let a = to_safe_integer(f64::MIN);
        assert_eq!(a, None);
        let a = to_safe_integer(f64::NEG_INFINITY);
        assert_eq!(a, None);

        // too large
        let a = to_safe_integer(9007199254740992.0);
        assert_eq!(a, None);
        let a = to_safe_integer(f64::MAX);
        assert_eq!(a, None);
        let a = to_safe_integer(f64::INFINITY);
        assert_eq!(a, None);

        // non-zero fractional part
        let a = to_safe_integer(123.023);
        assert_eq!(a, None);

        // NaN
        let a = to_safe_integer(f64::NAN);
        assert_eq!(a, None);
    }
}
