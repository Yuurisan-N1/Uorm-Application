use crate::net::client::Api;
use crate::ops::auth;
use crate::ops::auth::session::save;
use crate::support::cfg::loader::Setup;
use crate::support::error::{AppError, AppResult};
use crate::support::log::format::{lg, ly};

pub async fn execute(setup: &Setup) -> AppResult<()> {
    let mut args = std::env::args().skip(2);
    let email = match args.next() {
        Some(value) if value.contains('@') => value,
        _ => {
            ly("Usage: uorm-bot verify <email> <code>");
            return Ok(());
        }
    };
    let code = match args.next() {
        Some(value) => value,
        None => {
            ly("Usage: uorm-bot verify <email> <code>");
            return Ok(());
        }
    };

    let account = setup
        .accounts
        .iter()
        .find(|item| item.email.eq_ignore_ascii_case(&email))
        .cloned()
        .ok_or_else(|| AppError::config("the email is not listed in data.txt"))?;

    let api = Api::open(setup.proxy_for(0))?;
    let session = match auth::otp::verify(&api, &account.email, &code).await {
        Ok(session) => session,
        Err(err) => {
            ly(&format!("The confirmation code was refused because {}", err));
            return Ok(());
        }
    };

    save(&setup.root, &session)?;
    lg(&format!(
        "The email {} was confirmed and the session was stored",
        account.masked()
    ));
    Ok(())
}
