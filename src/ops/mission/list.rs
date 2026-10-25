use crate::domain::mission::{Mission, Progress};
use crate::net::client::Api;
use crate::support::consts::net::{TABLE_MISSIONS, TABLE_USER_MISSIONS};
use crate::support::error::{AppError, AppResult};

pub async fn perform(api: &Api) -> AppResult<(Vec<Mission>, Vec<Progress>)> {
    let missions = api
        .read(
            TABLE_MISSIONS,
            "select=*&is_active=eq.true&order=sort_order.asc",
        )
        .await?;
    let user_id = api
        .user_id()
        .ok_or_else(|| AppError::api("the session is not open"))?;
    let query = format!("select=*&user_id=eq.{}", user_id);
    let progress = api.read(TABLE_USER_MISSIONS, &query).await?;
    Ok((Mission::from_list(&missions), Progress::from_list(&progress)))
}
