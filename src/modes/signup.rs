use crate::net::client::{pick, Api};
use crate::ops::auth;
use crate::support::cfg::loader::Setup;
use crate::support::error::AppResult;
use crate::support::log::format::{lg, ly};

pub async fn execute(setup: &Setup) -> AppResult<()> {
    let email = match std::env::args().nth(2) {
        Some(value) if value.contains('@') => value,
        _ => {
            ly("Usage: uorm-bot signup <email>");
            return Ok(());
        }
    };

    let account = match setup
        .accounts
        .iter()
        .find(|item| item.email.eq_ignore_ascii_case(&email))
    {
        Some(account) => account.clone(),
        None => {
            ly("The email is not listed in data.txt");
            return Ok(());
        }
    };

    let api = Api::open(setup.proxy_for(0))?;
    match api.signup(&account.email, &account.password).await {
        Ok(reply) => {
            let confirmed = pick(&reply, "email_confirmed_at");
            if confirmed.is_empty() {
                lg(&format!(
                    "The account {} was created and a confirmation code was sent",
                    account.masked()
                ));
            } else {
                lg(&format!(
                    "The account {} was created and already confirmed",
                    account.masked()
                ));
            }
        }
        Err(err) => {
            let text = err.to_string().to_ascii_lowercase();
            if text.contains("already") {
                match auth::otp::resend(&api, &account.email).await {
                    Ok(()) => lg(&format!(
                        "The account {} was already there and a new code was sent",
                        account.masked()
                    )),
                    Err(err) => ly(&format!("The code could not be sent because {}", err)),
                }
            } else {
                ly(&format!("The account could not be created because {}", err));
            }
        }
    }

    Ok(())
}
