use rand::{
    distributions::{uniform::SampleUniform, Distribution, Uniform},
    Rng,
};

use crate::prng::make_prng;

/// Derives a random integer in the range [begin, end], i.e. including both bounds.
/// Use this method to avoid a modulo bias.
///
/// ## Example
///
/// ```
/// use nois::int_in_range;
///
/// # let randomness: [u8; 32] = [0x77; 32];
/// let dice = int_in_range(randomness, 1, 6);
/// assert!(dice >= 1);
/// assert!(dice <= 6);
/// ```
pub fn int_in_range<T>(randomness: [u8; 32], begin: T, end: T) -> T
where
    T: SampleUniform + Int,
{
    let mut rng = make_prng(randomness);
    rng.gen_range(begin..=end)
}

/// Derives random integers in the given range.
/// Use this method to avoid a modulo bias.
/// The resulting vector will contain exactly `count` elements.
///
/// Using this is potentially more efficient than multiple calls of [`int_in_range`].
///
/// ## Example
///
/// A round of [Yahtzee](https://en.wikipedia.org/wiki/Yahtzee) with five dices:
///
/// ```
/// use nois::ints_in_range;
///
/// # let randomness: [u8; 32] = [0x77; 32];
/// let dices = ints_in_range(randomness, 5, 1..=6);
/// assert_eq!(dices.len(), 5);
/// assert!(dices[0] >= 1 && dices[0] <= 6);
/// assert!(dices[1] >= 1 && dices[1] <= 6);
/// assert!(dices[2] >= 1 && dices[2] <= 6);
/// assert!(dices[3] >= 1 && dices[3] <= 6);
/// assert!(dices[4] >= 1 && dices[4] <= 6);
/// ```
pub fn ints_in_range<T, R>(randomness: [u8; 32], count: usize, range: R) -> Vec<T>
where
    T: SampleUniform + Int,
    R: Into<Uniform<T>>,
{
    let mut rng = make_prng(randomness);
    let uniform: Uniform<T> = range.into();
    let mut out = Vec::with_capacity(count);
    for _ in 0..count {
        out.push(uniform.sample(&mut rng));
    }
    debug_assert_eq!(out.len(), count); // this is guaranteed by the API definition
    debug_assert_eq!(out.capacity(), count); // this is not guaranteed but handy
    out
}

/// A trait to restrict int types for [`int_in_range`]
pub trait Int: PartialOrd + Default + Copy {}

impl Int for u8 {}
impl Int for u16 {}
impl Int for u32 {}
impl Int for u64 {}
impl Int for u128 {}
impl Int for usize {}
impl Int for i8 {}
impl Int for i16 {}
impl Int for i32 {}
impl Int for i64 {}
impl Int for i128 {}
impl Int for isize {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn int_in_range_works() {
        // Half-open interval
        let result = int_in_range(
            [
                88, 85, 86, 91, 61, 64, 60, 71, 234, 24, 246, 200, 35, 73, 38, 187, 54, 59, 96, 9,
                237, 27, 215, 103, 148, 230, 28, 48, 51, 114, 203, 219,
            ],
            4,
            18,
        );
        assert_eq!(result, 11);

        let result = int_in_range(
            [
                207, 251, 10, 105, 100, 223, 244, 6, 207, 231, 253, 206, 157, 68, 143, 184, 209,
                222, 70, 249, 114, 160, 213, 73, 147, 94, 136, 191, 94, 98, 99, 170,
            ],
            4,
            18,
        );
        assert_eq!(result, 17);

        let result = int_in_range(
            [
                43, 140, 160, 0, 187, 41, 212, 6, 218, 53, 58, 198, 80, 209, 171, 239, 222, 247,
                30, 23, 184, 79, 79, 221, 192, 225, 217, 142, 135, 164, 169, 255,
            ],
            4,
            18,
        );
        assert_eq!(result, 6);

        let result = int_in_range(
            [
                43, 140, 160, 0, 187, 41, 212, 6, 218, 53, 58, 198, 80, 209, 171, 239, 222, 247,
                30, 23, 184, 79, 79, 221, 192, 225, 217, 142, 135, 164, 169, 255,
            ],
            123,
            123,
        );
        assert_eq!(result, 123);

        // Negative numbers
        let result = int_in_range(
            [
                74, 71, 86, 169, 247, 21, 60, 71, 234, 24, 246, 215, 35, 73, 38, 187, 54, 59, 96,
                9, 237, 27, 215, 103, 14, 230, 28, 48, 51, 114, 203, 219,
            ],
            -100,
            100,
        );
        assert_eq!(result, -28);

        // u128
        let result = int_in_range(
            [
                74, 71, 86, 169, 247, 21, 60, 71, 234, 24, 246, 215, 35, 73, 38, 187, 54, 59, 96,
                9, 237, 27, 215, 103, 14, 230, 28, 48, 51, 114, 203, 219,
            ],
            17u128,
            u128::MAX,
        );
        assert_eq!(result, 226364637901700064573816523306429827859);
    }

    #[test]
    #[should_panic = "cannot sample empty range"]
    fn int_in_range_panicks_for_empty() {
        let result = int_in_range(
            [
                52, 187, 72, 255, 102, 110, 115, 233, 50, 165, 124, 255, 217, 131, 112, 209, 253,
                176, 108, 99, 102, 225, 12, 36, 82, 107, 106, 207, 99, 107, 197, 84,
            ],
            4,
            3,
        );
        assert_eq!(result, 5);
    }

    #[test]
    fn ints_in_range_works() {
        let randomness = [
            88, 85, 86, 91, 61, 64, 60, 71, 234, 24, 246, 200, 35, 73, 38, 187, 54, 59, 96, 9, 237,
            27, 215, 103, 148, 230, 28, 48, 51, 114, 203, 219,
        ];

        // Zero outputs
        let result = ints_in_range(randomness, 0, 4..19);
        assert!(result.is_empty());

        // One output
        let result = ints_in_range(randomness, 1, 4..19);
        assert_eq!(result, [11]);

        // Two outputs
        let result = ints_in_range(randomness, 2, 4..19);
        assert_eq!(result, [11, 16]);

        // 75 bytes outputs
        let result = ints_in_range(randomness, 75, u8::MIN..u8::MAX);
        assert_eq!(
            result,
            [
                127, 204, 5, 41, 250, 214, 33, 11, 185, 160, 253, 125, 4, 137, 64, 219, 173, 74,
                162, 30, 75, 198, 203, 42, 80, 96, 219, 231, 129, 132, 225, 48, 224, 64, 154, 28,
                69, 241, 231, 80, 185, 142, 220, 185, 28, 40, 195, 246, 219, 235, 57, 194, 180,
                193, 45, 16, 226, 81, 1, 190, 167, 212, 233, 159, 107, 93, 55, 173, 137, 218, 192,
                87, 58, 251, 242
            ]
        );
    }
}
