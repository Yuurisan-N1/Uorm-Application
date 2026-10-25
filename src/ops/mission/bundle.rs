use serde_json::json;

use crate::net::client::Api;
use crate::ops::mission::claim::reward_of;
use crate::support::consts::net::RPC_CLAIM_BUNDLE;
use crate::support::error::AppResult;

pub const KINDS: [&str; 2] = ["daily", "weekly"];

pub async fn perform(api: &Api, kind: &str) -> AppResult<f64> {
    let reply = api
        .rpc(RPC_CLAIM_BUNDLE, &json!({ "p_type": kind }))
        .await?;
    Ok(reward_of(&reply))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_two_bundles_are_offered() {
        assert_eq!(KINDS.len(), 2);
        assert!(KINDS.contains(&"daily"));
        assert!(KINDS.contains(&"weekly"));
    }
}
