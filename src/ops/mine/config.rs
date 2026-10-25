use serde_json::Value;

use crate::net::client::{count, money, Api};
use crate::support::error::AppResult;

const TABLE_CONFIG: &str = "app_config";

#[derive(Clone, Copy, Debug)]
pub struct Mining {
    pub base_rate: f64,
    pub session_hours: f64,
    pub boost_multiplier: f64,
    pub boost_minutes: f64,
    pub max_boosts: u64,
}

impl Default for Mining {
    fn default() -> Self {
        Mining {
            base_rate: 1.3,
            session_hours: 24.0,
            boost_multiplier: 2.0,
            boost_minutes: 60.0,
            max_boosts: 2,
        }
    }
}

impl Mining {
    pub fn duration_ms(&self) -> i64 {
        (self.session_hours * 3_600_000.0).round() as i64
    }

    pub fn boost_duration_ms(&self) -> i64 {
        (self.boost_minutes * 60_000.0).round() as i64
    }

    fn merge(&mut self, row: &Value) {
        let key = match row.get("key").and_then(|value| value.as_str()) {
            Some(key) => key,
            None => return,
        };
        let value = row.get("value").unwrap_or(&Value::Null);
        match key {
            "mining.base_rate_per_hour" => self.base_rate = money(row, "value"),
            "mining.session_hours" => self.session_hours = money(row, "value"),
            "mining.boost_multiplier" => self.boost_multiplier = money(row, "value"),
            "mining.boost_minutes" => self.boost_minutes = money(row, "value"),
            "mining.max_boosts_per_day" => self.max_boosts = count(row, "value"),
            _ => {
                let _ = value;
            }
        }
    }
}

pub async fn fetch(api: &Api) -> AppResult<Mining> {
    let mut mining = Mining::default();
    let rows = api
        .read(TABLE_CONFIG, "select=key,value&key=like.mining.*")
        .await?;
    if let Some(list) = rows.as_array() {
        for row in list {
            mining.merge(row);
        }
    }
    if mining.session_hours < 1.0 {
        mining.session_hours = 1.0;
    }
    if mining.boost_minutes < 1.0 {
        mining.boost_minutes = 1.0;
    }
    if mining.boost_multiplier < 1.0 {
        mining.boost_multiplier = 1.0;
    }
    Ok(mining)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn the_defaults_match_the_shipped_app() {
        let mining = Mining::default();
        assert_eq!(mining.duration_ms(), 86_400_000);
        assert_eq!(mining.boost_duration_ms(), 3_600_000);
    }

    #[test]
    fn the_server_rows_override_the_defaults() {
        let mut mining = Mining::default();
        mining.merge(&json!({ "key": "mining.session_hours", "value": 12 }));
        mining.merge(&json!({ "key": "mining.base_rate_per_hour", "value": 1.7 }));
        mining.merge(&json!({ "key": "mining.max_boosts_per_day", "value": 3 }));
        assert_eq!(mining.duration_ms(), 43_200_000);
        assert!((mining.base_rate - 1.7).abs() < 1e-9);
        assert_eq!(mining.max_boosts, 3);
    }
}
