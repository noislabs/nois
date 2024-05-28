use cosmwasm_std::Env;
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
/// let result = coinflip(randomness_simulator(&env));
/// assert_eq!(result, Side::Tails);
///
/// ```
///
pub fn randomness_simulator(env: &Env) -> [u8; 32] {
    let block_height = env.block.height;
    let mut hasher = Sha256::new();
    hasher.update(block_height.to_be_bytes());
    hasher.finalize().into()
}

#[cfg(test)]
mod tests {

    use crate::{coinflip, Side};
    use cosmwasm_std::{
        testing::mock_env, Addr, BlockInfo, ContractInfo, Env, HexBinary, Timestamp,
    };

    use crate::randomness_simulator;

    #[test]
    fn simulator_works() {
        let seed = randomness_simulator(&mock_env());
        let result = coinflip(seed);
        assert_eq!(result, Side::Tails);

        // mock another env with the same block height and make sure the outcome is the same
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
        let seed2 = randomness_simulator(&env);
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

        let seed3 = randomness_simulator(&env);
        assert_ne!(seed, seed3);
        print!("{}", HexBinary::from(seed3));
        let result = coinflip(seed3);
        assert_eq!(result, Side::Heads);
    }

    #[test]
    fn coinflip_distribution_is_uniform() {
        /// This test will loop through many blocks
        /// and throws a coin with every block randomness
        /// then checks that the distribution is expected within a range of 1%
        use std::collections::HashMap;

        const TEST_SAMPLE_SIZE: usize = 100_000;
        const ACCURACY: f32 = 0.01;

        let mut env = mock_env();

        let mut result = vec![];

        for block in 1..TEST_SAMPLE_SIZE + 1 {
            env.block.height = block as u64;
            let simulated_randomness = randomness_simulator(&env);
            //println!("{}", HexBinary::try_from(simulated_randomness).unwrap());
            let flip = coinflip(simulated_randomness);

            result.push(flip.is_heads());
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
