use crate::domain::account::Account;
use crate::domain::session::Session;
use crate::net::client::{count, pick, Api};
use crate::support::error::{AppError, AppResult};

pub fn now() -> i64 {
    chrono::Utc::now().timestamp()
}

pub fn session_from(reply: &serde_json::Value, email: &str) -> AppResult<Session> {
    let access = pick(reply, "access_token");
    let refresh = pick(reply, "refresh_token");
    let user_id = match reply.get("user") {
        Some(user) => pick(user, "id"),
        None => String::new(),
    };
    if access.is_empty() || refresh.is_empty() || user_id.is_empty() {
        return Err(AppError::api("the sign in carried no session"));
    }
    let expires_in = count(reply, "expires_in") as i64;
    let span = if expires_in > 0 { expires_in } else { 3_600 };
    Ok(Session {
        email: email.trim().to_string(),
        user_id,
        access,
        refresh,
        expires_at: now() + span,
    })
}

pub async fn perform(api: &Api, account: &Account) -> AppResult<Session> {
    let reply = api.password(&account.email, &account.password).await?;
    session_from(&reply, &account.email)
}
