use crate::domain::session::Session;
use crate::net::client::Api;
use crate::support::error::AppResult;

pub async fn perform(api: &Api, cached: &Session) -> AppResult<Session> {
    let reply = api.renew(&cached.refresh).await?;
    crate::ops::auth::login::session_from(&reply, &cached.email)
}
