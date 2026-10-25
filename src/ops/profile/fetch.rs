use crate::domain::profile::Profile;
use crate::net::client::Api;
use crate::support::consts::net::TABLE_PROFILES;
use crate::support::error::{AppError, AppResult};

pub async fn perform(api: &Api) -> AppResult<Profile> {
    let user_id = api
        .user_id()
        .ok_or_else(|| AppError::api("the session is not open"))?;
    let query = format!("select=*&id=eq.{}", user_id);
    let rows = api.read(TABLE_PROFILES, &query).await?;
    let row = rows
        .as_array()
        .and_then(|list| list.first())
        .ok_or_else(|| AppError::api("the profile was not found"))?;
    Ok(Profile::from_value(row))
}
