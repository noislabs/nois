use std::fmt;

/// The side of a coin. This is the result type of [`coinflip`]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Side {
    Heads = 0,
    Tails = 1,
}

// Displays as "heads" or "tails"
impl fmt::Display for Side {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Side::Heads => write!(f, "heads"),
            Side::Tails => write!(f, "tails"),
        }
    }
}

impl Side {
    pub fn is_heads(&self) -> bool {
        match self {
            Side::Heads => true,
            Side::Tails => false,
        }
    }

    pub fn is_tails(&self) -> bool {
        !self.is_heads()
    }
}

/// Takes a randomness and returns the result of a coinflip (heads or tails).
///
/// ## Example
///
/// ```
/// use nois::{coinflip, Side};
///
/// let randomness: [u8; 32] = [0x77; 32];
/// let side = coinflip(randomness);
/// println!("Result: {side}");
/// match side {
///     Side::Heads => {
///         // Player A starts the game
///     },
///     Side::Tails => {
///         // Player B starts the game
///     },
/// }
/// ```
pub fn coinflip(randomness: [u8; 32]) -> Side {
    if randomness[0] % 2 == 0 {
        Side::Heads
    } else {
        Side::Tails
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const RANDOMNESS1: [u8; 32] = [
        88, 85, 86, 91, 61, 64, 60, 71, 234, 24, 246, 200, 35, 73, 38, 187, 54, 59, 96, 9, 237, 27,
        215, 103, 148, 230, 28, 48, 51, 114, 203, 219,
    ];
    const RANDOMNESS2: [u8; 32] = [
        207, 251, 10, 105, 100, 223, 244, 6, 207, 231, 253, 206, 157, 68, 143, 184, 209, 222, 70,
        249, 114, 160, 213, 73, 147, 94, 136, 191, 94, 98, 99, 170,
    ];
    const RANDOMNESS3: [u8; 32] = [
        43, 140, 160, 0, 187, 41, 212, 6, 218, 53, 58, 198, 80, 209, 171, 239, 222, 247, 30, 23,
        184, 79, 79, 221, 192, 225, 217, 142, 135, 164, 169, 255,
    ];
    const RANDOMNESS4: [u8; 32] = [
        52, 187, 72, 255, 102, 110, 115, 233, 50, 165, 124, 255, 217, 131, 112, 209, 253, 176, 108,
        99, 102, 225, 12, 36, 82, 107, 106, 207, 99, 107, 197, 84,
    ];

    #[test]
    fn side_is_heads_and_is_tails_works() {
        assert!(Side::Heads.is_heads());
        assert!(!Side::Heads.is_tails());

        assert!(Side::Tails.is_tails());
        assert!(!Side::Tails.is_heads());
    }

    #[test]
    fn side_implements_display() {
        let heads = Side::Heads;
        let embedded = format!("Side: {}", heads);
        assert_eq!(embedded, "Side: heads");
        assert_eq!(heads.to_string(), "heads");

        let tails = Side::Tails;
        let embedded = format!("Side: {}", tails);
        assert_eq!(embedded, "Side: tails");
        assert_eq!(tails.to_string(), "tails");
    }

    #[test]
    fn coinflip_works() {
        let result = coinflip(RANDOMNESS1);
        assert_eq!(result, Side::Heads);

        let result = coinflip(RANDOMNESS2);
        assert_eq!(result, Side::Tails);

        let result = coinflip(RANDOMNESS3);
        assert_eq!(result, Side::Tails);

        let result = coinflip(RANDOMNESS4);
        assert_eq!(result, Side::Heads);
    }
    #[test]
    fn coinflip_distribution_is_uniform() {
        /// This test will generate a huge amount  of subrandomness
        /// and throws a coin with every subrandomness
        /// then checks that the distribution is expected within a range of 1%
        use crate::sub_randomness::sub_randomness;
        use std::collections::HashMap;

        const TEST_SAMPLE_SIZE: usize = 300_000;
        const ACCURACY: f32 = 0.01;

        let mut result = vec![];

        for subrand in sub_randomness(RANDOMNESS1).take(TEST_SAMPLE_SIZE) {
            result.push(coinflip(subrand).is_heads());
        }

        let mut histogram = HashMap::new();

        for element in result {
            let count = histogram.entry(element).or_insert(0);
            *count += 1;
        }

        let estimated_count_for_uniform_distribution = (TEST_SAMPLE_SIZE / 2) as f32;
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
}
