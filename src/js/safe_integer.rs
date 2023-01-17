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

/// Converts a JavaScript number to an u32.
/// The number must be in the safe integer range, i.e. in
/// [0, 4294967295].
pub fn to_u32(value: f64) -> Option<u32> {
    const MIN: i64 = 0;
    const MAX: i64 = 4294967295;
    let converted = value as i64;
    if value != (converted as f64) {
        return None; // Not an integer
    }
    if !(MIN..=MAX).contains(&converted) {
        return None;
    }
    Some(converted as u32)
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

    #[test]
    fn to_u32_works() {
        let a = to_u32(0.0);
        assert_eq!(a, Some(0));
        let a = to_u32(100.0);
        assert_eq!(a, Some(100));
        let a = to_u32(4294967295.0);
        assert_eq!(a, Some(u32::MAX));

        // too small
        let a = to_u32(-1.0);
        assert_eq!(a, None);
        let a = to_u32(-0.00001);
        assert_eq!(a, None);
        let a = to_u32(f64::MIN);
        assert_eq!(a, None);
        let a = to_u32(f64::NEG_INFINITY);
        assert_eq!(a, None);

        // too large
        let a = to_u32(4294967296.0);
        assert_eq!(a, None);
        let a = to_u32(f64::MAX);
        assert_eq!(a, None);
        let a = to_u32(f64::INFINITY);
        assert_eq!(a, None);

        // non-zero fractional part
        let a = to_u32(123.023);
        assert_eq!(a, None);

        // NaN
        let a = to_u32(f64::NAN);
        assert_eq!(a, None);
    }
}
