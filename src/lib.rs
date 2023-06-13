//! # nois
//!
//! `nois` is a collection of utilities to help you interact with the Nois network
//! Use this library to :
//! * Integrate your app with the nois proxy.
//! * Safely transform and manipulate your randomness.

mod coinflip;
mod decimal;
mod dice;
mod encoding;
mod integers;
mod js;
mod pick;
mod prng;
mod proxy;
mod select_from_weighted;
mod shuffle;
mod simulator;
mod sub_randomness;

pub use coinflip::{coinflip, Side};
pub use decimal::random_decimal;
pub use dice::roll_dice;
pub use encoding::{randomness_from_str, RandomnessFromStrErr};
pub use integers::{int_in_range, ints_in_range, Int};
pub use pick::pick;
pub use proxy::{NoisCallback, ProxyExecuteMsg, ReceiverExecuteMsg, MAX_JOB_ID_LEN};
pub use select_from_weighted::select_from_weighted;
pub use shuffle::shuffle;
pub use simulator::randomness_simulator;
pub use sub_randomness::{sub_randomness, sub_randomness_with_key, SubRandomnessProvider};
