use serde_json::json;

use crate::domain::mission::Mission;
use crate::net::client::Api;
use crate::support::consts::net::RPC_MARK_SOCIAL;
use crate::support::error::AppResult;

pub fn wanted(missions: &[Mission]) -> Vec<String> {
    missions
        .iter()
        .filter(|item| item.kind == "social")
        .map(|item| item.id.clone())
        .collect()
}

pub async fn mark(api: &Api, mission_id: &str) -> AppResult<()> {
    api.rpc(RPC_MARK_SOCIAL, &json!({ "p_mission_id": mission_id }))
        .await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mission(id: &str, kind: &str) -> Mission {
        Mission {
            id: id.to_string(),
            title: id.to_string(),
            kind: kind.to_string(),
            reward: 1.0,
            requirement_key: "social_x".to_string(),
            requirement_count: 1,
        }
    }

    #[test]
    fn only_the_social_missions_are_picked() {
        let list = vec![
            mission("social_x", "social"),
            mission("daily_open", "daily"),
            mission("social_telegram", "social"),
            mission("weekly_invite", "weekly"),
        ];
        let picked = wanted(&list);
        assert_eq!(picked, vec!["social_x", "social_telegram"]);
    }
}
