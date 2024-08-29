use crate::{bindings::query::NeutronQuery, NeutronResult};
use cosmwasm_std::{Coin, Deps};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct MinInterchainQueryDepositResponse {
    pub query_deposit: Vec<Coin>,
}

pub fn query_min_interchain_query_deposit(
    deps: Deps<NeutronQuery>,
) -> NeutronResult<MinInterchainQueryDepositResponse> {
    let query = NeutronQuery::MinInterchainQueryDeposit {};
    Ok(deps.querier.query(&query.into())?)
}
