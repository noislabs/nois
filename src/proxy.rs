use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, HexBinary, Timestamp};

/// Max length that the job ID is allowed to have (in bytes)
///
/// The value is chosen to be enough for 32 byte hashes (such as sha256)
/// in hex representation. But the choice of the job ID format is up to the
/// dapp and can be anything that respects this length limit.
pub const MAX_JOB_ID_LEN: usize = 64;

#[cw_serde]
pub enum ProxyExecuteMsg {
    /// Gets the next randomness.
    GetNextRandomness {
        /// A job ID chosen by the caller.
        ///
        /// Then length of this must not exceed [`MAX_JOB_ID_LEN`].
        job_id: String,
    },
    /// Gets a randomness that is published after the provided timestamp.
    ///
    /// For example you can request a randomness in e.g. 25 hours for a game
    /// round that runs for the upcoming 24 hours.
    ///
    /// Working with this message is only inteded for advanced use cases.
    /// You need to ensure in the calling app that no action can be performed
    /// anymore once `after` is reached. You need to consider that the BFT blocktime
    /// can be behind and add an appriate safety margin.
    GetRandomnessAfter {
        /// The publish time of the randomness needs to be > `after`.
        after: Timestamp,
        /// A job ID chosen by the caller.
        ///
        /// Then length of this must not exceed [`MAX_JOB_ID_LEN`].
        job_id: String,
    },
}

/// This must be accepted in an `NoisReceive { callback: NoisCallback }` enum case
/// in the ExecuteMsg of the app.
#[cw_serde]
pub struct NoisCallback {
    /// The ID chosen by the caller for this job. Use this field to map responses to requests.
    pub job_id: String,
    /// The point in time when the randomness was first published. This information is provided
    /// by the randomness provider. This is not the time when the randomness was processed on chain.
    pub published: Timestamp,
    /// The randomness. This is guaranteed to be 32 bytes long.
    pub randomness: HexBinary,
    /// The relayer that brings the packet from Nois to the consumer chain.
    /// Might be useful if the Dapp wants to incentivise the relayer operator
    pub relayer: Addr,
}

/// This is just a helper to properly serialize the above callback.
/// The actual receiver should include this variant in the larger ExecuteMsg enum.
#[cw_serde]
pub enum ReceiverExecuteMsg {
    /// This is sent as `{"nois_receive": {"callback": {"job_id": "...", "randomness": "aabbddff.."}}}`
    /// to the contract. We prefix the enum variant with `nois_` in order to avoid
    /// a collision with other contracts (see https://github.com/noislabs/nois/issues/4).
    NoisReceive { callback: NoisCallback },
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::to_vec;

    #[test]
    fn receiver_execute_msg_serializes_nicely() {
        let msg = ReceiverExecuteMsg::NoisReceive {
            callback: NoisCallback {
                job_id: "first".to_string(),
                published: Timestamp::from_seconds(1682086395),
                randomness: HexBinary::from_hex(
                    "aabbccddaabbccddaabbccddaabbccddaabbccddaabbccddaabbccddaabbccdd",
                )
                .unwrap(),
                relayer: Addr::unchecked("relayer"),
            },
        };
        let ser = to_vec(&msg).unwrap();
        assert_eq!(
            ser,
            br#"{"nois_receive":{"callback":{"job_id":"first","published":"1682086395000000000","randomness":"aabbccddaabbccddaabbccddaabbccddaabbccddaabbccddaabbccddaabbccdd","relayer":"relayer"}}}"#
        );
    }
}
