use crate::int_in_range;

/// Returns a number from 1-6.
///
/// # Example
///
/// Single dice
///
/// ```
/// use nois::{randomness_from_str, roll_dice};
///
/// let randomness = randomness_from_str("9e8e26615f51552aa3b18b6f0bcf0dae5afbe30321e8d7ea7fa51ebeb1d8fe62").unwrap();
///
/// let number = roll_dice(randomness);
/// assert_eq!(number, 2);
/// ```
///
/// Double dice
///
/// ```
/// use nois::{roll_dice, sub_randomness};
///
/// let randomness: [u8; 32]
///     = hex::decode("e2ee31c20afbb20530f6a18d5d96fb20f766fe11799d3611b04bf9edbd2cffcb").unwrap().try_into().unwrap();
/// let mut provider = sub_randomness(randomness);
///
/// let number1 = roll_dice(provider.provide());
/// assert_eq!(number1, 3);
///
/// let number2 = roll_dice(provider.provide());
/// assert_eq!(number2, 6);
/// ```
pub fn roll_dice(randomness: [u8; 32]) -> u8 {
    int_in_range(randomness, 1..=6)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sub_randomness;
    use hex_literal::hex;
    use std::collections::HashMap;

    #[test]
    fn roll_dice_works() {
        let randomness: [u8; 32] =
            hex!("9e8e26615f51552aa3b18b6f0bcf0dae5afbe30321e8d7ea7fa51ebeb1d8fe62");
        let number = roll_dice(randomness);
        assert_eq!(number, 2);
    }

    #[test]
    fn roll_dice_is_uniformly_distributes() {
        let randomness: [u8; 32] =
            hex!("5ec7020fe74dff44d50b255c1a680c362dc83de69bd3c865e0ef5f914bea6f7b");
        let mut results = HashMap::<u8, usize>::new();
        for sub_randomness in sub_randomness(randomness).take(600_000) {
            let number = roll_dice(sub_randomness);
            let current = results.get(&number).copied().unwrap_or_default();
            results.insert(number, current + 1);
        }
        let ones = results.get(&1).copied().unwrap_or_default();
        let twos = results.get(&2).copied().unwrap_or_default();
        let threes = results.get(&3).copied().unwrap_or_default();
        let fours = results.get(&4).copied().unwrap_or_default();
        let fives = results.get(&5).copied().unwrap_or_default();
        let sixes = results.get(&6).copied().unwrap_or_default();
        // println!("{ones} {twos} {threes} {fours} {fives} {sixes}");
        assert!(ones > 99_000 && ones < 101_000);
        assert!(twos > 99_000 && twos < 101_000);
        assert!(threes > 99_000 && threes < 101_000);
        assert!(fours > 99_000 && fours < 101_000);
        assert!(fives > 99_000 && fives < 101_000);
        assert!(sixes > 99_000 && sixes < 101_000);
        assert_eq!(results.values().sum::<usize>(), 600_000);
    }
}
