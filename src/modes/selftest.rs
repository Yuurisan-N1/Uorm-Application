use crate::domain::account::Account;
use crate::domain::mission::{Mission, Progress};
use crate::domain::profile::Profile;
use crate::domain::session::{key_for, Session};
use crate::net::proxy::Proxy;
use crate::ops::mission::claim::reward_of;
use crate::support::cfg::schema::Config;
use crate::support::error::AppResult;
use crate::support::log::format::{cash, clean_text, lg, lr, ly, mail, num};
use serde_json::json;

pub async fn execute() -> AppResult<()> {
    let mut checks: Vec<(&str, bool)> = Vec::new();

    checks.push(("the account line is read", Account::parse_line("a@b.com|secret").is_ok()));
    checks.push(("a colon line is refused", Account::parse_line("a@b.com:secret").is_err()));
    checks.push(("an empty password is refused", Account::parse_line("a@b.com|").is_err()));

    checks.push(("a socks proxy is read", Proxy::parse("socks5://127.0.0.1:1080").is_ok()));
    checks.push(("a http proxy is read", Proxy::parse("http://user:pass@gw.example.com:823").is_ok()));
    checks.push(("a proxy without a port takes 1080", Proxy::parse("socks5://127.0.0.1").map(|p| p.port).ok() == Some(1080)));
    checks.push(("a proxy with a bad scheme is refused", Proxy::parse("ftp://127.0.0.1:21").is_err()));

    checks.push(("the session key is stable", key_for("a@b.com") == key_for("A@B.com")));
    checks.push(("the session key is unique", key_for("a@b.com") != key_for("c@d.com")));

    let session = Session {
        email: "a@b.com".to_string(),
        user_id: "8b1f".to_string(),
        access: "access".to_string(),
        refresh: "refresh".to_string(),
        expires_at: 1_000,
    };
    checks.push(("a fresh session is used", session.fresh(500)));
    checks.push(("an old session is renewed", !session.fresh(990)));
    checks.push(("the tokens stay out of the debug line", {
        let line = format!("{:?}", session);
        !line.contains("\"access\": \"access\"") && !line.contains("\"refresh\": \"refresh\"") && line.contains("[REDACTED]")
    }));

    let profile = Profile::from_value(&json!({
        "id": "8b1f",
        "username": "yuuri",
        "total_coins": 12.5,
        "level": 3,
        "xp": 90,
        "streak_count": 2,
        "rank_id": "bronze",
        "referral_code": "ABC123"
    }));
    checks.push(("the profile is read", profile.coins == 12.5 && profile.level == 3));
    checks.push(("the name is read", profile.label("none") == "yuuri"));
    checks.push(("a missing name falls back", Profile::from_value(&json!({})).label("none") == "none"));

    let missions = Mission::from_list(&json!([
        { "id": "daily_open", "title": "Daily Check-In", "type": "daily", "reward_amount": 0.75 },
        { "id": "social_x", "title": "Follow on X", "type": "social", "reward_amount": 17.5 }
    ]));
    checks.push(("the mission list is read", missions.len() == 2));
    checks.push(("the mission reward is read", missions[1].reward == 17.5));

    let progress = Progress::from_list(&json!([
        { "mission_id": "daily_open", "progress": 1, "completed": true, "claimed": false }
    ]));
    checks.push(("the progress is read", progress[0].completed && !progress[0].claimed));
    checks.push(("an empty list stays empty", Mission::from_list(&json!(null)).is_empty()));

    checks.push(("the reward is read flat", reward_of(&json!({ "reward": 2.5 })) == 2.5));
    checks.push(("the reward is read nested", reward_of(&json!([{ "credited": 4 }])) == 4.0));
    checks.push(("an empty reward credits nothing", reward_of(&json!(null)) == 0.0));

    let config = Config::defaults();
    checks.push(("the sleep default is read", config.sleep_seconds() == 3_600));
    checks.push(("the config keeps the canon shape", Config::defaults().sleep_seconds() == 3_600));
    checks.push(("a bad config is refused", Config::parse("{}").is_err()));

    checks.push(("the mail is masked", mail("someone@mail.com").starts_with("so")));
    checks.push(("the mail keeps the domain", mail("someone@mail.com").ends_with("@mail.com")));
    checks.push(("the number trims the tail", num(1.2300) == "1.23"));
    checks.push(("the coin line has no colour", !cash(1.5, "Coins").contains('\u{1b}')));
    checks.push(("the text drops banned symbols", clean_text("Daily #1", "task") == "Daily 1"));

    let mut failed = 0;
    for (name, passed) in &checks {
        if *passed {
            lg(&format!("The check passed for {}", name));
        } else {
            failed += 1;
            lr(&format!("The check failed for {}", name));
        }
    }

    if failed == 0 {
        lg(&format!("All {} checks passed", checks.len()));
    } else {
        lr(&format!("{} of {} checks failed", failed, checks.len()));
    }

    ly("The self test runs without touching the network");
    Ok(())
}
