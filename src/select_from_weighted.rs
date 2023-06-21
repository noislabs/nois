use crate::int_in_range;

/// Selects one element from a given weighted list.
///
/// In contrast to [`pick`] this does not move the selected element from the input list
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
/// use nois::{randomness_from_str, select_from_weighted};
///
/// let randomness = randomness_from_str("9e8e26615f51552aa3b18b6f0bcf0dae5afbe30321e8d7ea7fa51ebeb1d8fe62").unwrap();
///
/// let list = vec![
///     ("green hat", 40),
///     ("viking helmet", 55),
///     ("rare golden crown", 5)
/// ];
///
/// let selected = select_from_weighted(randomness, &list).unwrap();
///
/// assert_eq!(selected, "viking helmet");
/// ```
pub fn select_from_weighted<T: Clone>(
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
    use crate::RANDOMNESS1;

    use super::*;

    #[test]
    fn select_from_weighted_works() {
        let elements: Vec<(char, u32)> = vec![('a', 1), ('b', 5), ('c', 4)];
        let picked = select_from_weighted(RANDOMNESS1, &elements).unwrap();
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
        let picked = select_from_weighted(RANDOMNESS1, &elements).unwrap();
        assert_eq!(picked, Color("orange".to_string()));

        // Pick from slice
        let selection = &elements[0..3];
        let picked = select_from_weighted(RANDOMNESS1, selection).unwrap();
        assert_eq!(picked, Color("green".to_string()));
    }

    #[test]
    fn select_from_weighted_fails_on_empty_list() {
        //This will check that the list is empty
        let elements: Vec<(i32, u32)> = vec![];

        let err = select_from_weighted(RANDOMNESS1, &elements).unwrap_err();

        // Check that the selected element has the expected weight
        assert_eq!(err, "List must not be empty");
    }

    #[test]
    fn select_from_weighted_fails_on_element_weight_less_than_1() {
        let elements: Vec<(i32, u32)> = vec![(1, 5), (2, 4), (-3, 0)];

        let err = select_from_weighted(RANDOMNESS1, &elements).unwrap_err();

        // Check that the selected element has the expected weight
        assert_eq!(err, "All element weights should be >= 1");
    }

    #[test]
    fn select_from_weighted_fails_with_total_weight_too_high() {
        let elements: Vec<(i32, u32)> = vec![(1, u32::MAX), (2, 1)];

        let err = select_from_weighted(RANDOMNESS1, &elements).unwrap_err();

        // Check that the selected element has the expected weight
        assert_eq!(err, "Total weight is greater than maximum value of u32");
    }

    #[test]
    fn select_from_weighted_distribution_is_uniform() {
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
            result.push(select_from_weighted(subrand, &elements).unwrap());
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
                (estimated_count_for_uniform_distribution * (1_f32 - ACCURACY)) as i32;
            let estimation_max: i32 =
                (estimated_count_for_uniform_distribution * (1_f32 + ACCURACY)) as i32;
            println!(
                "estimation {}, max: {}, min: {}",
                estimated_count_for_uniform_distribution, estimation_max, estimation_min
            );
            println!("{}: {}", bin, count);
            assert!(count >= estimation_min && count <= estimation_max);
        }
    }
}
