use wasm_bindgen::prelude::*;

use crate::{coinflip, int_in_range, roll_dice};

fn cast_vec_to_array<T, const N: usize>(v: Vec<T>) -> [T; N] {
    v.try_into()
        .unwrap_or_else(|v: Vec<T>| panic!("Expected a Vec of length {} but it was {}", N, v.len()))
}

#[wasm_bindgen]
#[allow(dead_code)] // exported via wasm_bindgen
pub fn coinflip_js(randomness: &str) -> String {
    let hex_randomness = hex::decode(randomness).unwrap();
    let hex_randomness_array = cast_vec_to_array(hex_randomness);
    let side = coinflip(hex_randomness_array);
    side.to_string()
}

// Returns a value from 1 to 6 (inclusive)
#[wasm_bindgen]
#[allow(dead_code)] // exported via wasm_bindgen
pub fn roll_dice_js(randomness: &str) -> u8 {
    let hex_randomness = hex::decode(randomness).unwrap();
    let hex_randomness_array = cast_vec_to_array(hex_randomness);
    roll_dice(hex_randomness_array)
}

/// Returns an integer between begin (inclusive) and end (exclusive).
///
/// Both bounds must be in the uint32 range.
#[wasm_bindgen]
#[allow(dead_code)] // exported via wasm_bindgen
pub fn int_in_range_js(randomness: &str, begin: u32, end: u32) -> u32 {
    let hex_randomness = hex::decode(randomness).unwrap();
    let hex_randomness_array = cast_vec_to_array(hex_randomness);
    int_in_range(hex_randomness_array, begin..end)
}
