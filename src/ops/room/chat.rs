use serde_json::json;

use crate::net::client::Api;
use crate::support::consts::net::TABLE_ROOMS;
use crate::support::error::{AppError, AppResult};

pub async fn send(api: &Api, room_id: &str, text: &str) -> AppResult<()> {
    let user_id = api
        .user_id()
        .ok_or_else(|| AppError::api("the session is not open"))?;
    let body = json!({
        "room_id": room_id,
        "user_id": user_id,
        "content": text
    });
    api.write(TABLE_ROOMS, &body).await?;
    Ok(())
}
