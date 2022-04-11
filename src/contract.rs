#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Uint128, WasmMsg, CosmosMsg};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ ExecuteMsg, InstantiateMsg, QueryMsg};
// use crate::state::{ State, STATE};

use cw20::{Cw20ExecuteMsg};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:challenge";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Deposit {amount, contract_addr} => try_deposit(deps, info, _env, amount, contract_addr),
        ExecuteMsg::Withdraw{amount, contract_addr} => try_withdraw(deps, info, _env, amount, contract_addr)
    }
}

pub fn try_withdraw(deps: DepsMut, info: MessageInfo, env: Env, amount: u64, contract_addr: String) -> Result<Response,ContractError>{
    let cw20_transfer_msg = Cw20ExecuteMsg::Transfer{
        recipient: info.sender.into(),
        amount: Uint128::from(amount)
    };

    let cw20_exec_transfer = WasmMsg::Execute{
        contract_addr,
        msg: to_binary(&cw20_transfer_msg)?,
        funds: vec![],
    };

    let cw20_cosmos_msg: CosmosMsg = cw20_exec_transfer.into();
    Ok(Response::new()
        .add_message(cw20_cosmos_msg))
}

pub fn try_deposit(deps: DepsMut,info: MessageInfo, env: Env, amount: u64, contract_addr: String ) -> Result<Response, ContractError>{

    let cw20_transfer_msg = Cw20ExecuteMsg::TransferFrom{
        recipient: env.contract.address.into(),
        amount: Uint128::from(amount),
        owner: info.sender.into(),
    };

    let cw20_exec_transfer = WasmMsg::Execute{
        contract_addr,
        msg: to_binary(&cw20_transfer_msg)?,
        funds: vec![],
    };

    let cw20_cosmos_msg: CosmosMsg = cw20_exec_transfer.into();
    Ok(Response::new()
        .add_message(cw20_cosmos_msg))
    
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{coins, from_binary};

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies(&[]);

        let msg = InstantiateMsg { };
        let info = mock_info("creator", &coins(1000, "earth"));

        // we can just call .unwrap() to assert this was a success
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());

    }

    #[test]
    fn deposit() {
        let mut deps = mock_dependencies(&coins(2, "token"));

        let msg = InstantiateMsg { };
        let info = mock_info("creator", &coins(2, "token"));
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        // beneficiary can release it
        let info = mock_info("anyone", &coins(2, "token"));
        let deposit_msg =  Cw20ReceiveMsg{
            amount: Uint128::from(600000u64),
            sender: info.clone().sender.into(),
            msg: to_binary("data: &T").unwrap()

        };
        // let msg = ExecuteMsg::Deposit { msg: deposit_msg };
        // let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

        // println!("{:#?}", _res);
        // should increase counter by 1
        // let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
        // let value: CountResponse = from_binary(&res).unwrap();
        // assert_eq!(18, value.count);
    }
}
