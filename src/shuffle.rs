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
    use crate::RANDOMNESS1;

    use super::*;

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

    #[test]
    fn shuffle_distribution_is_uniform() {
        /// This test takes a vector of characters as data
        /// Then it will generate many shuffled combinations out of it with sub_randomness
        /// Then for each common index of these sampled vectors it will make a histogram of what characters were selectd for that index.
        /// The result will be 10 histograms and every histogram will show how many "a" how many "b" ... "k"
        /// Example we have 3 samples:
        /// 1- vec!["a", "b", "c", "d", "e", "f", "h", "i", "j", "k"]
        /// 2- vec!["e", "f", "h", "i", "j", "k", "a", "b", "c", "d",]
        /// 3- vec!["a", "c", "b", "d", "e", "f", "h", "i", "j", "k"]
        /// The first histogram will have "a" -> 2, "e"-> 1, and the rest 0
        /// Then we make 10 assertions per histogram on whether that character was represented as expected within that index with 5% accuracy
        /// We do this for all the 10 histograms, so 100 assertions in total.
        /// This test pretty much tests Fisher Yates algorithm and our implementation of it
        use crate::sub_randomness::sub_randomness;
        use std::collections::HashMap;

        const TEST_SAMPLE_SIZE: usize = 100_000;
        const ACCURACY: f32 = 0.05;

        let data = vec!["a", "b", "c", "d", "e", "f", "h", "i", "j", "k"];

        let mut result = vec![];

        for subrand in sub_randomness(RANDOMNESS1).take(TEST_SAMPLE_SIZE) {
            result.push(shuffle(subrand, data.clone()));
        }
        //let acc_max = 1 as f32 * ACCURACY;
        let estimation_min = (TEST_SAMPLE_SIZE / data.len()) as f32 * (1_f32 - ACCURACY);
        let estimation_max = (TEST_SAMPLE_SIZE / data.len()) as f32 * (1_f32 + ACCURACY);
        println!(
            "estimation min: {} estimation max: {}  ",
            estimation_min, estimation_max
        );

        for i in 0..data.len() {
            let mut histogram = HashMap::new();

            for vec in &result {
                let element = vec.get(i).unwrap();
                let count = histogram.entry(element).or_insert(0);
                *count += 1;
            }

            for (bin, count) in histogram {
                println!("Histogram index: {} - {}: {}", i, bin, count);
                assert!(count >= estimation_min as i32 && count <= estimation_max as i32);
            }
        }
    }
}
