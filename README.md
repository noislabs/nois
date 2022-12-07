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

    NoisReceive {
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

        ExecuteMsg::NoisReceive { callback } => execute_nois_receive(deps, env, info, callback),
    }
}

// ...

pub fn execute_nois_receive(
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

## Build for JavaScript

The Nois Toolbox can be compiled to JavaScript via WebAssembly. This way you can simulate
the outputs for every randomness value. The results match exactly those of CosmWasm contracts
using the same tools.

In order to keep the JS/Wasm interface simple, there is a wrapper in the module `lib/js` which takes
randomness inputs in hex format and uses types and error handling that plays well with JS.
JS/Wasm bindings are created using wasm-bindgen.

The JS does not match 100% the contract implementation. The differences are documented here.

| Contract function        | JS function      | Status     | Note                                                                 |
| ------------------------ | ---------------- | ---------- | -------------------------------------------------------------------- |
| [`nois::coinflip`]       | `coinflip`       | ✅ Ready   | Returns string instead of enum                                       |
| [`nois::roll_dice`]      | `roll_dice`      | ✅ Ready   |                                                                      |
| [`nois::int_in_range`]   | `int_in_range`   | ✅ Ready   | Only supports half-oen range, i.e. the end value is always exluded   |
| [`nois::ints_in_range`]  | `ints_in_range`  | 🚫 Missing |                                                                      |
| [`nois::random_decimal`] | `random_decimal` | ✅ Ready   | Encodes result Decimal as string                                     |
| [`nois::sub_randomness`] | `sub_randomness` | ✅ Ready   | Takes a `count` argument and returns an Array instead of an iterator |
| [`nois::shuffle`]        |                  | 🚫 Missing |                                                                      |

[`nois::coinflip`]: https://docs.rs/nois/latest/nois/fn.coinflip.html
[`nois::roll_dice`]: https://docs.rs/nois/latest/nois/fn.roll_dice.html
[`nois::int_in_range`]: https://docs.rs/nois/latest/nois/fn.int_in_range.html
[`nois::ints_in_range`]: https://docs.rs/nois/latest/nois/fn.ints_in_range.html
[`nois::random_decimal`]: https://docs.rs/nois/latest/nois/fn.random_decimal.html
[`nois::sub_randomness`]: https://docs.rs/nois/latest/nois/fn.sub_randomness.html
[`nois::shuffle`]: https://docs.rs/nois/latest/nois/fn.shuffle.html

**Installation**

We need this:

```
$ cargo install wasm-pack -f
$ wasm-pack --version
wasm-pack 0.10.3
```

**For Node.js**

This creates a CommonJS module that is loaded synchonously.

```
$ wasm-pack build --target nodejs -- --features js
$ node
> const { coinflip, roll_dice, random_decimal, sub_randomness, int_in_range } = require('./pkg/nois');

// Round 2497992

> coinflip("c59f098f3c12b8c36ed81f5c17660c72414a1ed63467888a374af529a205c584")
'tails'

// Round 2497994

> coinflip("2267ba7356c01a58e405d4194a31bddc3fd3eb1f0a86758f7a609ba8a47420ba")
'heads'
> roll_dice("2267ba7356c01a58e405d4194a31bddc3fd3eb1f0a86758f7a609ba8a47420ba")
6
> random_decimal("2267ba7356c01a58e405d4194a31bddc3fd3eb1f0a86758f7a609ba8a47420ba")
'0.126047856387596389'
> sub_randomness("2267ba7356c01a58e405d4194a31bddc3fd3eb1f0a86758f7a609ba8a47420ba", 3)
[
  'ac7b151d67cd4263520b16e450e6d1fc01750dab80b5d8b7cdc4075c99daf80a',
  '33622b0865f1ab35142e3e63a91c25cf89311b04b9540ca15e49413a4a114ce1',
  'f08927af18d4995c28f15f07e4038407f32d966087771314b9e64b6a33a9101c'
]
> int_in_range("2267ba7356c01a58e405d4194a31bddc3fd3eb1f0a86758f7a609ba8a47420ba", 5, 9)
8
```

**For browsers and other JS environments**

You need to change the target in order to get a suiteable package. E.g.

```
$ wasm-pack build --target web -- --features js
$ ls ./pkg
```

for browsers. Please refer to the wasm-bindgen handbook [to learn more about targets](https://rustwasm.github.io/docs/wasm-bindgen/reference/deployment.html).
