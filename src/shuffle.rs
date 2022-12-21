use rand::Rng;

use crate::prng::make_prng;

/// Shuffles a vector using the Fisher-Yates algorithm.
///
/// This consumes the vector of elements for efficientcy reasons. Applications that do
/// not need the original data anymore benefit from an allocation-free in-place implementation.
///
/// ## Examples
///
/// Shuffle a vector of integers:
///
/// ```
/// use nois::{randomness_from_str, shuffle};
///
/// let randomness = randomness_from_str("9e8e26615f51552aa3b18b6f0bcf0dae5afbe30321e8d7ea7fa51ebeb1d8fe62").unwrap();
///
/// // We are randomly shuffling a vector of integers [1,2,3,4]
/// let data = vec![1, 2, 3, 4];
/// let shuffled = shuffle(randomness, data);
/// // The length of the vector is the same but the order of the elements has changed
/// assert_eq!(shuffled.len(), 4);
/// assert_eq!(shuffled, vec![2, 4, 3, 1]);
/// ```
///
/// Shuffle a vector of strings:
///
/// ```
/// use nois::{randomness_from_str, shuffle};
///
/// let randomness = randomness_from_str("9e8e26615f51552aa3b18b6f0bcf0dae5afbe30321e8d7ea7fa51ebeb1d8fe62").unwrap();
///
/// let data = vec!["bob".to_string(), "mary".to_string(), "su".to_string(), "marc".to_string()];
/// let shuffled = shuffle(randomness, data);
/// // The length of the vector is the same but the order of the elements has changed
/// assert_eq!(shuffled.len(), 4);
/// assert_eq!(shuffled, vec!["mary".to_string(), "marc".to_string(), "su".to_string(), "bob".to_string()]);
/// ```
///
/// Keep a copy of the original list
///
/// ```
/// use nois::{randomness_from_str, shuffle};
///
/// let randomness = randomness_from_str("9e8e26615f51552aa3b18b6f0bcf0dae5afbe30321e8d7ea7fa51ebeb1d8fe62").unwrap();
///
/// let original = vec!["bob".to_string(), "mary".to_string(), "su".to_string(), "marc".to_string()];
/// let shuffled = shuffle(randomness, original.clone());
/// // The length of the vector is the same but the order of the elements has changed
/// assert_eq!(shuffled.len(), original.len());
/// assert_ne!(shuffled, original);
/// ```
pub fn shuffle<T>(randomness: [u8; 32], mut data: Vec<T>) -> Vec<T> {
    let mut rng = make_prng(randomness);
    for i in (1..data.len()).rev() {
        let j = rng.gen_range(0..=i);
        data.swap(i, j);
    }
    data
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
        let data: Vec<i32> = vec![];
        let shuffled = shuffle(RANDOMNESS1, data);
        assert_eq!(shuffled, Vec::<i32>::new());

        let data = vec![5];
        let shuffled = shuffle(RANDOMNESS1, data);
        assert_eq!(shuffled, vec![5]);

        // Order has changed for larger vector
        let data = vec![1, 2, 3, 4];
        let shuffled = shuffle(RANDOMNESS1, data);
        assert_eq!(shuffled.len(), 4);
        assert_ne!(shuffled, vec![1, 2, 3, 4]);
    }
}
