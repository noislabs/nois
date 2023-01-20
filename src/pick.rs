use rand::Rng;

use crate::int_in_range;
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
/// assert_eq!(picked, vec![7, 33, 18, 22, 8, 10]);
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
    for i in ((data.len() - n)..data.len()).rev() {
        let j = rng.gen_range(0..=i);
        data.swap(i, j);
    }

    // Get last n elements
    data.split_off(data.len() - n)
}

/// Picks 1 element from a given weighted list.
///
/// In contrast to [`pick`] this does not move the picked element from the input list
/// but requires elements to be `Clone`able. This because only one element is needed.
/// It could be implemented differently though.
///
/// The list must not be empty. Each element must have a non-zeo weight.
/// The total weight must not exceed the u32 range.
///
/// ## Examples
///
/// Pick 1 hat out of 3 hats with different rarity:
///
/// ```
/// use nois::{randomness_from_str, pick_one_from_weighted_list};
///
/// let randomness = randomness_from_str("9e8e26615f51552aa3b18b6f0bcf0dae5afbe30321e8d7ea7fa51ebeb1d8fe62").unwrap();
///
/// let list = vec![
///     ("green hat", 40),
///     ("viking helmet", 55),
///     ("rare golden crown", 5)
/// ];
///
/// let picked = pick_one_from_weighted_list(randomness, &list).unwrap();
///
/// assert_eq!(picked, "viking helmet");
/// ```
pub fn pick_one_from_weighted_list<T: Clone>(
    randomness: [u8; 32],
    list: &[(T, u32)],
) -> Result<T, String> {
    if list.is_empty() {
        return Err(String::from("List must not be empty"));
    }

    let mut total_weight: u32 = 0;
    for (_, weight) in list {
        if *weight == 0 {
            return Err(String::from("All element weights should be >= 1"));
        }
        total_weight = total_weight
            .checked_add(*weight)
            .ok_or_else(|| String::from("Total weight is greater than maximum value of u32"))?;
    }

    debug_assert!(
        total_weight > 0,
        "we know we have a non-empty list of non-zero elements"
    );

    let r = int_in_range(randomness, 1, total_weight);
    let mut weight_sum = 0;
    for element in list {
        weight_sum += element.1;
        if r <= weight_sum {
            return Ok(element.0.clone());
        }
    }
    // This point should never be reached
    panic!("No element selected")
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

        // Element type is neither Copy nor Clone, i.e. the result is moved ot of the input data.
        #[derive(PartialEq, Debug)]
        struct Continent(String);
        let data = vec![
            Continent("Africa".into()),
            Continent("America".to_string()),
            Continent("Antarctica".to_string()),
            Continent("Australia".to_string()),
            Continent("Eurasia".to_string()),
        ];
        let picked = pick(RANDOMNESS1, 2, data);
        assert_eq!(picked.len(), 2);
        assert_eq!(
            picked,
            vec![Continent("America".into()), Continent("Eurasia".into())]
        );
    }

    #[test]
    #[should_panic = "attempt to pick more elements than the input length"]
    fn pick_panicks_for_n_greater_than_len() {
        let data = vec![1, 2, 3, 4];
        let _picked = pick(RANDOMNESS1, 5, data);
    }

    #[test]
    fn pick_one_from_weighted_list_works() {
        let elements: Vec<(char, u32)> = vec![('a', 1), ('b', 5), ('c', 4)];
        let picked = pick_one_from_weighted_list(RANDOMNESS1, &elements).unwrap();
        assert_eq!(picked, 'c');

        // Element type is Clone but not Copy
        #[derive(PartialEq, Debug, Clone)]
        struct Color(String);
        let elements = vec![
            (Color("red".into()), 12),
            (Color("blue".to_string()), 15),
            (Color("green".to_string()), 8),
            (Color("orange".to_string()), 21),
            (Color("pink".to_string()), 11),
        ];
        let picked = pick_one_from_weighted_list(RANDOMNESS1, &elements).unwrap();
        assert_eq!(picked, Color("orange".to_string()));

        // Pick from slice
        let selection = &elements[0..3];
        let picked = pick_one_from_weighted_list(RANDOMNESS1, selection).unwrap();
        assert_eq!(picked, Color("green".to_string()));
    }

    #[test]
    fn test_pick_one_from_weighted_list_fails_on_empty_list() {
        //This will check that the list is empty
        let elements: Vec<(i32, u32)> = vec![];

        let err = pick_one_from_weighted_list(RANDOMNESS1, &elements).unwrap_err();

        // Check that the selected element has the expected weight
        assert_eq!(err, "List must not be empty");
    }

    #[test]
    fn test_pick_one_from_weighted_list_fails_on_element_weight_less_than_1() {
        let elements: Vec<(i32, u32)> = vec![(1, 5), (2, 4), (-3, 0)];

        let err = pick_one_from_weighted_list(RANDOMNESS1, &elements).unwrap_err();

        // Check that the selected element has the expected weight
        assert_eq!(err, "All element weights should be >= 1");
    }

    #[test]
    fn test_pick_one_from_weighted_list_fails_with_total_weight_too_high() {
        let elements: Vec<(i32, u32)> = vec![(1, u32::MAX), (2, 1)];

        let err = pick_one_from_weighted_list(RANDOMNESS1, &elements).unwrap_err();

        // Check that the selected element has the expected weight
        assert_eq!(err, "Total weight is greater than maximum value of u32");
    }

    #[test]
    fn pick_one_from_weighted_list_distribution_is_uniform() {
        /// This test will generate a huge amount  of subrandomness
        /// then checks that the distribution is expected within a range of 1%
        use crate::sub_randomness::sub_randomness;
        use std::collections::HashMap;

        const TEST_SAMPLE_SIZE: usize = 1_000_000;
        const ACCURACY: f32 = 0.005;
        // This test needs the sum of the weights to be equal to 1.
        // Although the function should work as expected for weights that do not equal 1
        let elements: Vec<(String, u32)> = vec![
            (String::from("a"), 100),
            (String::from("b"), 200),
            (String::from("c"), 30),
            (String::from("d"), 70),
            (String::from("e"), 600),
        ];
        let total_weight = elements.iter().map(|element| element.1).sum::<u32>();
        println!("total weight: {}", total_weight);

        let mut result = vec![];

        for subrand in sub_randomness(RANDOMNESS1).take(TEST_SAMPLE_SIZE) {
            result.push(pick_one_from_weighted_list(subrand, &elements).unwrap());
        }

        let mut histogram = HashMap::new();

        for element in result {
            let count = histogram.entry(element).or_insert(0);
            *count += 1;
        }

        // This will assert on all the elements of the data 1 by 1 and check if their occurence is within the 1% expected range
        for (bin, count) in histogram {
            let probability = elements.iter().find(|e| e.0 == bin).map(|e| e.1).unwrap() as f32
                / total_weight as f32;
            let estimated_count_for_uniform_distribution = TEST_SAMPLE_SIZE as f32 * probability;
            let estimation_min: i32 =
                (estimated_count_for_uniform_distribution as f32 * (1_f32 - ACCURACY)) as i32;
            let estimation_max: i32 =
                (estimated_count_for_uniform_distribution as f32 * (1_f32 + ACCURACY)) as i32;
            println!(
                "estimation {}, max: {}, min: {}",
                estimated_count_for_uniform_distribution, estimation_max, estimation_min
            );
            println!("{}: {}", bin, count);
            assert!(count >= estimation_min && count <= estimation_max);
        }
    }

    #[test]
    fn pick_distribution_is_uniform() {
        /// This test will generate a huge amount  of subrandomness and picks n elements from the list
        /// It will then test that the outcome of every possibility within the picked value falls with 1% close
        /// To what it should be in a uniform distribution
        /// For this test to work properly for a 10 element size data consider choosing a TEST_SAMPLE_SIZE higher than 100_000
        use crate::sub_randomness::sub_randomness;
        use std::collections::HashMap;

        const TEST_SAMPLE_SIZE: usize = 300_000;
        const N_PICKED_ELEMENTS: usize = 3;
        const ACCURACY: f32 = 0.01;

        let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];

        let mut result = vec![vec![]];

        for subrand in sub_randomness(RANDOMNESS1).take(TEST_SAMPLE_SIZE) {
            result.push(pick(subrand, N_PICKED_ELEMENTS, data.clone()));
        }

        let mut histogram = HashMap::new();

        for row in result {
            for element in row {
                let count = histogram.entry(element).or_insert(0);
                *count += 1;
            }
        }
        let estimated_count_for_uniform_distribution =
            (TEST_SAMPLE_SIZE * N_PICKED_ELEMENTS / data.len()) as f32;
        let estimation_min: i32 =
            (estimated_count_for_uniform_distribution * (1_f32 - ACCURACY)) as i32;
        let estimation_max: i32 =
            (estimated_count_for_uniform_distribution * (1_f32 + ACCURACY)) as i32;
        println!(
            "estimation {}, max: {}, min: {}",
            estimated_count_for_uniform_distribution, estimation_max, estimation_min
        );
        // This will assert on all the elements of the data 1 by 1 and check if their occurence is within the 1% expected range
        for (bin, count) in histogram {
            println!("{}: {}", bin, count);
            assert!(count >= estimation_min && count <= estimation_max);
        }
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
