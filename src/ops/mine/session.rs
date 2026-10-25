use chrono::{DateTime, Utc};
use serde_json::{json, Value};

use crate::net::client::{count, money, pick, Api};
use crate::support::consts::net::{
    RPC_APPLY_BOOST, RPC_BOOST_STATUS, RPC_CLAIM_SESSION, RPC_START_SESSION, TABLE_SESSIONS,
};
use crate::support::error::{AppError, AppResult};

#[derive(Clone, Debug)]
pub struct Active {
    pub id: String,
    pub started_at: DateTime<Utc>,
    pub duration_ms: i64,
    pub rate: f64,
    pub multiplier: f64,
    pub boost_expires: Option<DateTime<Utc>>,
}

impl Active {
    pub fn elapsed_ms(&self) -> i64 {
        (Utc::now() - self.started_at).num_milliseconds()
    }

    pub fn complete(&self) -> bool {
        self.elapsed_ms() >= self.duration_ms
    }

    pub fn remaining_ms(&self) -> i64 {
        (self.duration_ms - self.elapsed_ms()).max(0)
    }

    pub fn boosted(&self) -> bool {
        match self.boost_expires {
            Some(expiry) => expiry > Utc::now(),
            None => false,
        }
    }
}

fn parse_time(value: &Value, key: &str) -> Option<DateTime<Utc>> {
    let text = pick(value, key);
    if text.is_empty() {
        return None;
    }
    DateTime::parse_from_rfc3339(&text)
        .ok()
        .map(|stamp| stamp.with_timezone(&Utc))
}

fn read_one(row: &Value) -> Option<Active> {
    let id = pick(row, "id");
    if id.is_empty() {
        return None;
    }
    let started_at = parse_time(row, "started_at")?;
    let duration_ms = match count(row, "session_duration_ms") {
        0 => 86_400_000,
        value => value as i64,
    };
    Some(Active {
        id,
        started_at,
        duration_ms,
        rate: money(row, "rate_per_hour"),
        multiplier: money(row, "boost_multiplier"),
        boost_expires: parse_time(row, "boost_expires_at"),
    })
}

pub async fn active(api: &Api) -> AppResult<Option<Active>> {
    let query = "select=id,started_at,session_duration_ms,rate_per_hour,boost_multiplier,boost_expires_at&claimed=eq.false&order=started_at.desc&limit=1";
    let rows = api.read(TABLE_SESSIONS, query).await?;
    Ok(rows
        .as_array()
        .and_then(|list| list.first())
        .and_then(read_one))
}

pub async fn start(api: &Api, rate: f64, duration_ms: i64) -> AppResult<Active> {
    let body = json!({ "p_rate_per_hour": rate, "p_duration_ms": duration_ms });
    let reply = api.rpc(RPC_START_SESSION, &body).await?;
    read_one(&reply).ok_or_else(|| AppError::api("the server did not return the mining session"))
}

pub async fn claim(api: &Api, session_id: &str) -> AppResult<f64> {
    let reply = api
        .rpc(RPC_CLAIM_SESSION, &json!({ "p_session_id": session_id }))
        .await?;
    Ok(money(&reply, "amount"))
}

pub async fn boosts_used(api: &Api) -> AppResult<(u64, u64)> {
    let reply = api.rpc(RPC_BOOST_STATUS, &json!({})).await?;
    Ok((count(&reply, "used"), count(&reply, "limit")))
}

pub async fn boost(api: &Api, session: &Active, multiplier: f64, duration_ms: i64) -> AppResult<f64> {
    let boosted = (session.rate * multiplier * 1_000_000.0).round() / 1_000_000.0;
    let body = json!({
        "p_session_id": session.id,
        "p_multiplier": multiplier,
        "p_duration_ms": duration_ms,
        "p_boosted_rate": boosted,
    });
    let reply = api.rpc(RPC_APPLY_BOOST, &body).await?;
    Ok(money(&reply, "rate_per_hour"))
}

pub fn describe(ms: i64) -> String {
    let total = ms / 1000;
    let hours = total / 3600;
    let minutes = (total % 3600) / 60;
    if hours > 0 {
        format!("{}h {}m", hours, minutes)
    } else {
        format!("{}m", minutes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> Active {
        Active {
            id: "abc".into(),
            started_at: Utc::now() - chrono::Duration::hours(25),
            duration_ms: 86_400_000,
            rate: 1.3065,
            multiplier: 1.0,
            boost_expires: None,
        }
    }

    #[test]
    fn a_session_past_its_duration_is_complete() {
        assert!(sample().complete());
        assert_eq!(sample().remaining_ms(), 0);
    }

    #[test]
    fn a_fresh_session_is_not_complete() {
        let mut session = sample();
        session.started_at = Utc::now() - chrono::Duration::hours(2);
        assert!(!session.complete());
        assert!(session.remaining_ms() > 0);
    }

    #[test]
    fn a_missing_boost_is_not_running() {
        assert!(!sample().boosted());
        let mut session = sample();
        session.boost_expires = Some(Utc::now() + chrono::Duration::minutes(30));
        assert!(session.boosted());
    }

    #[test]
    fn the_duration_is_written_for_a_human() {
        assert_eq!(describe(86_400_000), "24h 0m");
        assert_eq!(describe(3_600_000), "1h 0m");
        assert_eq!(describe(600_000), "10m");
    }
}
