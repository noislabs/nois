#![cfg(feature = "js")]

mod safe_integer;

///! This module contains a wrapper for this library for JavaScript.
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[allow(dead_code)] // exported via wasm_bindgen
pub fn coinflip(randomness: &str) -> Result<String, JsValue> {
    Ok(implementations::coinflip_impl(randomness)?)
}

// Returns a value from 1 to 6 (inclusive)
#[wasm_bindgen]
#[allow(dead_code)] // exported via wasm_bindgen
pub fn roll_dice(randomness: &str) -> Result<u8, JsValue> {
    Ok(implementations::roll_dice_impl(randomness)?)
}

/// Returns an integer between begin (inclusive) and end (inclusive).
///
/// Both bounds must be numbers in the safe integer range.
#[wasm_bindgen]
#[allow(dead_code)] // exported via wasm_bindgen
pub fn int_in_range(randomness: &str, begin: JsValue, end: JsValue) -> Result<JsValue, JsValue> {
    Ok(implementations::int_in_range_impl(randomness, begin, end)?)
}

/// Returns multiple integers between begin (inclusive) and end (inclusive).
///
/// Both bounds must be numbers in the safe integer range.
#[wasm_bindgen]
#[allow(dead_code)] // exported via wasm_bindgen
pub fn ints_in_range(
    randomness: &str,
    count: u32,
    begin: JsValue,
    end: JsValue,
) -> Result<Box<[JsValue]>, JsValue> {
    Ok(implementations::ints_in_range_impl(
        randomness, count, begin, end,
    )?)
}

/// Returns a Decimal d with 0 <= d < 1.
/// The Decimal is in string representation and has 18 decimal digits.
#[wasm_bindgen]
#[allow(dead_code)] // exported via wasm_bindgen
pub fn random_decimal(randomness: &str) -> Result<String, JsValue> {
    Ok(implementations::random_decimal_impl(randomness)?.to_string())
}

/// Returns sub-randomness that is derives from the given randomness.
#[wasm_bindgen]
#[allow(dead_code)] // exported via wasm_bindgen
pub fn sub_randomness(randomness: &str, count: u32) -> Result<Box<[JsValue]>, JsValue> {
    let strings = implementations::sub_randomness_impl(randomness, count)?;
    Ok(strings
        .into_iter()
        .map(|s| JsValue::from_str(&s))
        .collect::<Vec<_>>()
        .into_boxed_slice())
}

// Takes a JavaScript array and returns a shuffled version of it. In contrast to the
// Rust implementation this does not shuffle in-place.
#[wasm_bindgen]
#[allow(dead_code)] // exported via wasm_bindgen
pub fn shuffle(randomness: &str, input: Box<[JsValue]>) -> Result<Box<[JsValue]>, JsValue> {
    Ok(implementations::shuffle_impl(randomness, input)?)
}

mod implementations {
    use super::safe_integer::to_safe_integer;
    use crate::{
        coinflip, int_in_range, ints_in_range, random_decimal, randomness_from_str, roll_dice,
        shuffle, sub_randomness, RandomnessFromStrErr,
    };
    use cosmwasm_std::Decimal;
    use wasm_bindgen::JsValue;

    pub struct JsError(String);

    impl From<RandomnessFromStrErr> for JsError {
        fn from(source: RandomnessFromStrErr) -> Self {
            Self(source.to_string())
        }
    }

    impl From<JsError> for wasm_bindgen::JsValue {
        fn from(source: JsError) -> wasm_bindgen::JsValue {
            wasm_bindgen::JsValue::from_str(&source.0)
        }
    }

    pub fn coinflip_impl(randomness_hex: &str) -> Result<String, JsError> {
        let randomness = randomness_from_str(randomness_hex)?;
        let side = coinflip(randomness);
        Ok(side.to_string())
    }

    pub fn roll_dice_impl(randomness_hex: &str) -> Result<u8, JsError> {
        let randomness = randomness_from_str(randomness_hex)?;
        Ok(roll_dice(randomness))
    }

    pub fn int_in_range_impl(
        randomness_hex: &str,
        begin: JsValue,
        end: JsValue,
    ) -> Result<JsValue, JsError> {
        let begin = begin
            .as_f64()
            .ok_or_else(|| JsError("begin is not of type number".to_string()))?;
        let end = end
            .as_f64()
            .ok_or_else(|| JsError("end is not of type number".to_string()))?;
        let begin = to_safe_integer(begin)
            .ok_or_else(|| JsError("begin is not a safe integer".to_string()))?;
        let end =
            to_safe_integer(end).ok_or_else(|| JsError("end is not a safe integer".to_string()))?;

        // Without this check we'd get a panic in Wasm (unreachable) when creating the range,
        // which is hard to debug.
        if end < begin {
            return Err(JsError(
                "end must be larger than or equal to begin".to_string(),
            ));
        }
        let randomness = randomness_from_str(randomness_hex)?;
        let out = int_in_range(randomness, begin..=end);
        Ok(JsValue::from_f64(out as f64))
    }

    pub fn ints_in_range_impl(
        randomness_hex: &str,
        count: u32,
        begin: JsValue,
        end: JsValue,
    ) -> Result<Box<[JsValue]>, JsError> {
        let begin = begin
            .as_f64()
            .ok_or_else(|| JsError("begin is not of type number".to_string()))?;
        let end = end
            .as_f64()
            .ok_or_else(|| JsError("end is not of type number".to_string()))?;
        let begin = to_safe_integer(begin)
            .ok_or_else(|| JsError("begin is not a safe integer".to_string()))?;
        let end =
            to_safe_integer(end).ok_or_else(|| JsError("end is not a safe integer".to_string()))?;

        // Without this check we'd get a panic in Wasm (unreachable) when creating the range,
        // which is hard to debug.
        if end < begin {
            return Err(JsError(
                "end must be larger than or equal to begin".to_string(),
            ));
        }
        let randomness = randomness_from_str(randomness_hex)?;
        let count = count as usize; // usize is 32 bit (wasm32) or 64 bit (dev machines)
        let out = ints_in_range(randomness, count, begin..=end)
            .into_iter()
            .map(|i| JsValue::from_f64(i as f64))
            .collect::<Vec<_>>()
            .into_boxed_slice();
        Ok(out)
    }

    pub fn random_decimal_impl(randomness_hex: &str) -> Result<Decimal, JsError> {
        let randomness = randomness_from_str(randomness_hex)?;
        Ok(random_decimal(randomness))
    }

    pub fn sub_randomness_impl(randomness_hex: &str, count: u32) -> Result<Vec<String>, JsError> {
        let randomness = randomness_from_str(randomness_hex)?;
        let count = count as usize;
        let mut out = Vec::with_capacity(count);
        for sub_randomness in sub_randomness(randomness).take(count) {
            out.push(hex::encode(sub_randomness));
        }
        Ok(out)
    }

    pub fn shuffle_impl(
        randomness_hex: &str,
        input: Box<[JsValue]>,
    ) -> Result<Box<[JsValue]>, JsError> {
        let randomness = randomness_from_str(randomness_hex)?;
        let mut a: Vec<JsValue> = input.into();
        shuffle(randomness, &mut a);
        Ok(a.into_boxed_slice())
    }
}
