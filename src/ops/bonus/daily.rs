use serde_json::json;

use crate::net::client::Api;
use crate::support::consts::net::RPC_CLAIM_DAILY;
use crate::support::error::AppResult;

pub async fn perform(api: &Api) -> AppResult<f64> {
    let reply = api.rpc(RPC_CLAIM_DAILY, &json!({})).await?;
    Ok(crate::ops::mission::claim::reward_of(&reply))
}
