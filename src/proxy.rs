use cosmwasm_schema::cw_serde;

use crate::HexBinary;

#[cw_serde]
pub enum ProxyExecuteMsg {
    /// Get's the next randomness.
    GetNextRandomness {
        // A job ID chosen by the caller
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
