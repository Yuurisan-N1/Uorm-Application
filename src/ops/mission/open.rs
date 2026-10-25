use serde_json::json;

use crate::net::client::Api;
use crate::support::consts::net::RPC_PING_OPEN;
use crate::support::error::AppResult;

pub async fn ping(api: &Api) -> AppResult<()> {
    api.rpc(RPC_PING_OPEN, &json!({})).await?;
    Ok(())
}
