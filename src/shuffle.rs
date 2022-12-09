use rand::Rng;

use crate::prng::make_prng;

/// Shuffles a vector using the Fisher-Yates algorithm
///
/// ## Example
///
/// Shuffle a vector of integers
///
/// ```
/// use nois::{randomness_from_str, shuffle};
///
/// let randomness = randomness_from_str("9e8e26615f51552aa3b18b6f0bcf0dae5afbe30321e8d7ea7fa51ebeb1d8fe62").unwrap();
///
/// // We are randomly shuffling a vector of integers [1,2,3,4]
/// let mut data = vec![1, 2, 3, 4];
/// shuffle(randomness, &mut data);
/// // The length of the vector is the same but the order of the elements has changed
/// assert_eq!(data.len(), 4);
/// assert_ne!(data, vec![1, 2, 3, 4]);
/// ```
pub fn shuffle<T>(randomness: [u8; 32], data: &mut Vec<T>) {
    let mut rng = make_prng(randomness);
    for i in (1..data.len()).rev() {
        let j = rng.gen_range(0..=i);
        data.swap(i, j);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const RANDOMNESS1: [u8; 32] = [
        52, 187, 72, 255, 102, 110, 115, 233, 50, 165, 124, 255, 217, 131, 112, 209, 253, 176, 108,
        99, 102, 225, 12, 36, 82, 107, 106, 207, 99, 107, 197, 84,
    ];

    #[test]
    fn shuffle_works() {
        let mut data: Vec<i32> = vec![];
        shuffle(RANDOMNESS1, &mut data);
        assert_eq!(data, Vec::<i32>::new());

        let mut data = vec![5];
        shuffle(RANDOMNESS1, &mut data);
        assert_eq!(data, vec![5]);

        // Order has changed for larger vector
        let mut data = vec![1, 2, 3, 4];
        shuffle(RANDOMNESS1, &mut data);
        assert_eq!(data.len(), 4);
        assert_ne!(data, vec![1, 2, 3, 4]);
    }
}
