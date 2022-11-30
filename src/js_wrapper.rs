use wasm_bindgen::prelude::*;
use super::coinflip::{coinflip,Side};
use super::int_in_range;

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

fn _cast_vec_to_array<T, const N: usize>(v: Vec<T>) -> [T; N] {
    v.try_into()
        .unwrap_or_else(|v: Vec<T>| panic!("Expected a Vec of length {} but it was {}", N, v.len()))
}

#[wasm_bindgen]
pub fn _coinflip_js(randomness: &str)->Side {
    let hex_randomness = hex::decode(randomness).unwrap();
    let hex_randomness_array=_cast_vec_to_array(hex_randomness);
    coinflip(hex_randomness_array)
}

#[wasm_bindgen]
pub fn _int_in_range_single_dice_js(randomness: &str)->u8 {
    let hex_randomness = hex::decode(randomness).unwrap();
    let hex_randomness_array=_cast_vec_to_array(hex_randomness);
    int_in_range(hex_randomness_array, 1..6)
}