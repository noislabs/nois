use wasm_bindgen::prelude::*;

use crate::{coinflip, int_in_range, roll_dice};

//struct VerifyWebError(pub String);
//
//impl From<hex::FromHexError> for VerifyWebError {
//    fn from(source: hex::FromHexError) -> Self {
//        Self(source.to_string())
//    }
//}
//
//impl From<VerifyWebError> for JsValue {
//    fn from(source: VerifyWebError) -> JsValue {
//        JsValue::from_str(&source.0)
//    }
//}

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

#[wasm_bindgen]
#[allow(dead_code)] // exported via wasm_bindgen
pub fn int_in_range_js(randomness: &str, int_begin: u8, int_end: u8) -> u8 {
    let hex_randomness = hex::decode(randomness).unwrap();
    let hex_randomness_array = cast_vec_to_array(hex_randomness);
    int_in_range(hex_randomness_array, int_begin..int_end)
}