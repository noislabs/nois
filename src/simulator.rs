use cosmwasm_std::{Env, HexBinary};
use sha2::{Digest, Sha256};

/// Creates a predictable randomness seed
///
/// This generates a seed based on sha256 hash of the block height. to be used as a predictable randomness seed.
/// Warning!! If you need an unpredictalble randomness do not use this function.
/// Warning!! This function is intended for predictable randomness usecases. Or for a testnet environment.
///
/// ## Examples
///
/// Get a predictable coinflip using the randomness simulator:
///
/// ```
/// use nois::{coinflip, Side,randomness_simulator};
/// use cosmwasm_std::testing::mock_env;
///
/// let env = mock_env();
/// let result = coinflip(randomness_simulator(env).to_array().unwrap());
/// assert_eq!(result, Side::Tails);
///
/// ```
///
pub fn randomness_simulator(env: Env) -> HexBinary {
    let block_height = env.block.height;
    let mut hasher = Sha256::new();
    hasher.update((block_height).to_be_bytes());
    let hash: [u8; 32] = hasher.finalize().into();
    // Should we make here some safety before unsafely unwrapping? Is there really a risk here? In that case the function would return a result instead?
    HexBinary::try_from(hash).unwrap()
}

#[cfg(test)]
mod tests {

    use crate::{coinflip, Side};
    use cosmwasm_std::{testing::mock_env, Addr, BlockInfo, ContractInfo, Env, Timestamp};

    use crate::randomness_simulator;

    #[test]
    fn simulator_works() {
        let seed = randomness_simulator(mock_env());
        println!("{}", seed);
        let result = coinflip(seed.to_array().unwrap());
        assert_eq!(result, Side::Tails);

        // mock another env with the same block height andmake sure the outcome is the same
        let env = Env {
            block: BlockInfo {
                height: 12345,
                time: Timestamp::from_nanos(1),
                chain_id: "nois-1".to_string(),
            },
            transaction: None,
            contract: ContractInfo {
                address: Addr::unchecked("address"),
            },
        };
        let seed2 = randomness_simulator(env);
        assert_eq!(seed, seed2);

        // mock another env with a different block height and make sure the outcome is different

        let env = Env {
            block: BlockInfo {
                height: 12347,
                time: Timestamp::from_nanos(1),
                chain_id: "nois-1".to_string(),
            },
            transaction: None,
            contract: ContractInfo {
                address: Addr::unchecked("address"),
            },
        };
        let seed3 = randomness_simulator(env);
        assert_ne!(seed, seed3);
        let result = coinflip(seed3.to_array().unwrap());
        assert_eq!(result, Side::Heads);
    }
}
