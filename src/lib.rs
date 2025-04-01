use cosmwasm_schema::cw_serde;
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, StdResult, entry_point};

use epoching::{Coin, MsgDelegate, MsgWrappedDelegate};

mod epoching;
mod shim;

#[cw_serde]
struct InstantiateMsg {}

#[cw_serde]
enum ExecuteMsg {
    Delegate {},
}

#[cfg_attr(not(feature = "library"), entry_point)]
fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> StdResult<Response> {
    Ok(Response::new())
}

#[cfg_attr(not(feature = "library"), entry_point)]
fn execute(deps: DepsMut, env: Env, info: MessageInfo, msg: ExecuteMsg) -> StdResult<Response> {
    match msg {
        ExecuteMsg::Delegate {} => {
            let msg = MsgWrappedDelegate {
                msg: Some(MsgDelegate {
                    delegator_address: info.sender.to_string(),
                    validator_address: "bbnvaloper109x4ruspxarwt62puwcenhclw36l9v7j92f0ex"
                        .to_string(),
                    amount: Some(Coin {
                        denom: "ubbn".to_string(),
                        amount: "100000".to_string(),
                    }),
                }),
            };

            Ok(Response::new().add_message(msg))
        }
    }
}
