use cosmwasm_schema::cw_serde;
use cosmwasm_std::{HexBinary, Timestamp};

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

/// This must be accepted in an `Receive { callback: NoisCallback }` enum case
/// in the ExecuteMsg of the app.
#[cw_serde]
pub struct NoisCallback {
    /// The ID chosen by the caller for this job. Use this field to map responses to requests.
    pub job_id: String,
    pub randomness: HexBinary,
}

/// This is just a helper to properly serialize the above callback.
/// The actual receiver should include this variant in the larger ExecuteMsg enum.
#[cw_serde]
pub enum ReceiverExecuteMsg {
    Receive { callback: NoisCallback },
}
