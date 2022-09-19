# Nois standard library

[![nois on crates.io](https://img.shields.io/crates/v/nois.svg)](https://crates.io/crates/nois)
[![nois on docs.rs](https://img.shields.io/docsrs/nois.svg)](https://docs.rs/nois)

Use this library to integrate your app with the a nois proxy.

## Storing the proxy address

```rust
#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let nois_proxy_addr = deps
        .api
        .addr_validate(&msg.nois_proxy)
        .map_err(|_| ContractError::InvalidProxyAddress)?;
    NOIS_PROXY.save(deps.storage, &nois_proxy_addr)?;
    Ok(Response::new()
        .add_attribute("action", "instantiate")
        .add_attribute("nois_proxy", msg.nois_proxy))
}
```

## Sending a request

```rust
use nois::ProxyExecuteMsg;

pub fn execute_estimate_pi(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    job_id: String,
) -> Result<Response, ContractError> {
    let nois_proxy = NOIS_PROXY.load(deps.storage)?; // Nois proxy address stored in init

    let res = Response::new().add_message(WasmMsg::Execute {
        contract_addr: nois_proxy.into(),
        msg: to_binary(&ProxyExecuteMsg::GetNextRandomness { job_id })?,
        funds: vec![],
    });
    Ok(res)
}
```

## Processing the callback

Create a `ExecuteMsg` enum case called `Receive`

```rust
use cosmwasm_schema::{cw_serde, QueryResponses};

use nois::NoisCallback;

#[cw_serde]
pub enum ExecuteMsg {
    // ...

    Receive {
        callback: NoisCallback,
    },
}
```

and use it:

```rust
#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        // ...

        ExecuteMsg::Receive { callback } => execute_receive(deps, env, info, callback),
    }
}

// ...

pub fn execute_receive(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    callback: NoisCallback,
) -> Result<Response, ContractError> {
    let proxy = NOIS_PROXY.load(deps.storage)?;
    ensure_eq!(info.sender, proxy, ContractError::UnauthorizedReceive);

    let NoisCallback { job_id, randomness } = callback;
    let randomness: [u8; 32] = randomness
        .to_array()
        .map_err(|_| ContractError::InvalidRandomness)?;

    // use randomness 🎉
}
```
