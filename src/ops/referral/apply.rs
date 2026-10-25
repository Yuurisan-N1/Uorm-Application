use serde_json::json;

use crate::net::client::Api;
use crate::support::consts::net::RPC_APPLY_REFERRAL;
use crate::support::error::AppResult;

pub async fn perform(api: &Api, code: &str) -> AppResult<()> {
    api.rpc(RPC_APPLY_REFERRAL, &json!({ "p_code": code }))
        .await?;
    Ok(())
}
