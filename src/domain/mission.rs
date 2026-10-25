use serde_json::Value;

use crate::net::client::{count, money, pick};

#[derive(Debug, Clone)]
pub struct Mission {
    pub id: String,
    pub title: String,
    pub kind: String,
    pub reward: f64,
    pub requirement_key: String,
    pub requirement_count: u64,
}

impl Mission {
    pub fn from_value(value: &Value) -> Mission {
        Mission {
            id: pick(value, "id"),
            title: pick(value, "title"),
            kind: pick(value, "type"),
            reward: money(value, "reward_amount"),
            requirement_key: pick(value, "requirement_key"),
            requirement_count: count(value, "requirement_count"),
        }
    }

    pub fn from_list(value: &Value) -> Vec<Mission> {
        match value.as_array() {
            Some(rows) => rows.iter().map(Mission::from_value).collect(),
            None => Vec::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Progress {
    pub mission_id: String,
    pub progress: u64,
    pub completed: bool,
    pub claimed: bool,
}

impl Progress {
    pub fn from_value(value: &Value) -> Progress {
        Progress {
            mission_id: pick(value, "mission_id"),
            progress: count(value, "progress"),
            completed: value
                .get("completed")
                .and_then(Value::as_bool)
                .unwrap_or(false),
            claimed: value
                .get("claimed")
                .and_then(Value::as_bool)
                .unwrap_or(false),
        }
    }

    pub fn from_list(value: &Value) -> Vec<Progress> {
        match value.as_array() {
            Some(rows) => rows.iter().map(Progress::from_value).collect(),
            None => Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn the_mission_reads_the_supabase_row() {
        let row = json!({
            "id": "daily_open",
            "title": "Daily Check-In",
            "type": "daily",
            "reward_amount": 0.75,
            "requirement_key": "open_app",
            "requirement_count": 1,
            "is_active": true
        });
        let mission = Mission::from_value(&row);
        assert_eq!(mission.id, "daily_open");
        assert_eq!(mission.reward, 0.75);
        assert_eq!(mission.requirement_key, "open_app");
    }

    #[test]
    fn the_progress_reads_the_flags() {
        let row = json!({
            "mission_id": "daily_open",
            "progress": 1,
            "completed": true,
            "claimed": false
        });
        let progress = Progress::from_value(&row);
        assert!(progress.completed);
        assert!(!progress.claimed);
        assert_eq!(progress.progress, 1);
    }

    #[test]
    fn an_empty_list_stays_empty() {
        assert!(Mission::from_list(&json!([])).is_empty());
        assert!(Progress::from_list(&json!(null)).is_empty());
    }
}
