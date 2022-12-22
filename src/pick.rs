use rand::Rng;

use crate::prng::make_prng;

/// Picks `n` elements from a given list.
///
/// This consumes the vector of elements for efficientcy reasons. Applications that do
/// not need the original data anymore benefit from an efficient in-place implementation.
///
/// ## Examples
///
/// Pick 6 out of 49:
///
/// ```
/// use nois::{randomness_from_str, pick};
///
/// let randomness = randomness_from_str("9e8e26615f51552aa3b18b6f0bcf0dae5afbe30321e8d7ea7fa51ebeb1d8fe62").unwrap();
///
/// // We are randomly shuffling a vector of integers [1,2,3,4]
/// let data = vec![
///   1,  2,  3,  4,  5,  6,  7,  8,  9, 10, 11,
///   12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22,
///   23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33,
///   34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44,
///   45, 46, 47, 48, 49
/// ];
/// let picked = pick(randomness, 6, data);
/// // The length of the vector is the same but the order of the elements has changed
/// assert_eq!(picked.len(), 6);
/// assert_eq!(picked, vec![44, 33, 18, 22, 8, 10]);
/// ```
///
/// Pick two winners from a vector of strings:
///
/// ```
/// use nois::{randomness_from_str, pick};
///
/// let randomness = randomness_from_str("9e8e26615f51552aa3b18b6f0bcf0dae5afbe30321e8d7ea7fa51ebeb1d8fe62").unwrap();
///
/// let data = vec!["bob".to_string(), "mary".to_string(), "su".to_string(), "marc".to_string()];
/// let picked = pick(randomness, 2, data);
/// // The length of the vector is the same but the order of the elements has changed
/// assert_eq!(picked.len(), 2);
/// assert_eq!(picked, vec!["su".to_string(), "bob".to_string()]);
/// ```
pub fn pick<T>(randomness: [u8; 32], n: usize, mut data: Vec<T>) -> Vec<T> {
    if n > data.len() {
        panic!("attempt to pick more elements than the input length");
    }
    let mut rng = make_prng(randomness);
    for i in ((data.len() - n + 1)..data.len()).rev() {
        let j = rng.gen_range(0..=i);
        data.swap(i, j);
    }

    // Get last n elements
    data.split_off(data.len() - n)
}

#[cfg(test)]
mod tests {
    use crate::shuffle;

    use super::*;

    const RANDOMNESS1: [u8; 32] = [
        52, 187, 72, 255, 102, 110, 115, 233, 50, 165, 124, 255, 217, 131, 112, 209, 253, 176, 108,
        99, 102, 225, 12, 36, 82, 107, 106, 207, 99, 107, 197, 84,
    ];

    #[test]
    fn pick_works() {
        let data: Vec<i32> = vec![];
        let picked = pick(RANDOMNESS1, 0, data);
        assert_eq!(picked, Vec::<i32>::new());

        let data = vec![5];
        let picked = pick(RANDOMNESS1, 1, data);
        assert_eq!(picked, vec![5]);

        let data = vec![1, 2, 3, 4];
        let picked = pick(RANDOMNESS1, 3, data);
        assert_eq!(picked.len(), 3);
        assert_ne!(picked, vec![2, 3, 4]);
    }

    #[test]
    #[should_panic = "attempt to pick more elements than the input length"]
    fn pick_panicks_for_n_greater_than_len() {
        let data = vec![1, 2, 3, 4];
        let _picked = pick(RANDOMNESS1, 5, data);
    }

    #[test]
    fn pick_all_performs_full_shuffle_works() {
        let data = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let picked = pick(RANDOMNESS1, data.len(), data.clone());
        let shuffled = shuffle(RANDOMNESS1, data);
        assert_eq!(picked, shuffled);

        let data = vec!["return", "if", "break", "match", "mut", "let"];
        let picked = pick(RANDOMNESS1, data.len(), data.clone());
        let shuffled = shuffle(RANDOMNESS1, data);
        assert_eq!(picked, shuffled);

        let data = Vec::<u32>::new();
        let picked = pick(RANDOMNESS1, data.len(), data.clone());
        let shuffled = shuffle(RANDOMNESS1, data);
        assert_eq!(picked, shuffled);

        let data = vec![true, false];
        let picked = pick(RANDOMNESS1, data.len(), data.clone());
        let shuffled = shuffle(RANDOMNESS1, data);
        assert_eq!(picked, shuffled);

        let data = vec![()];
        let picked = pick(RANDOMNESS1, data.len(), data.clone());
        let shuffled = shuffle(RANDOMNESS1, data);
        assert_eq!(picked, shuffled);
    }
}
