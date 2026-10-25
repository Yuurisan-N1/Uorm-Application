use crate::domain::session::Session;
use crate::net::client::Api;
use crate::support::error::{AppError, AppResult};

pub fn normalize(token: &str) -> AppResult<String> {
    let digits: String = token.chars().filter(|c| c.is_ascii_digit()).collect();
    if digits.len() != 6 {
        return Err(AppError::api("the confirmation code is not six digits"));
    }
    Ok(digits)
}

pub async fn verify(api: &Api, email: &str, code: &str) -> AppResult<Session> {
    let code = normalize(code)?;
    let reply = api.verify_signup(email, &code).await?;
    crate::ops::auth::login::session_from(&reply, email)
}

pub async fn resend(api: &Api, email: &str) -> AppResult<()> {
    api.resend_signup(email).await?;
    Ok(())
}

pub fn needs_code(err: &AppError) -> bool {
    let text = err.to_string().to_ascii_lowercase();
    text.contains("email_not_confirmed")
        || text.contains("email not confirmed")
        || text.contains("not confirmed")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_code_is_six_digits() {
        assert_eq!(normalize(" 123456 ").unwrap(), "123456");
        assert_eq!(normalize("123-456").unwrap(), "123456");
        assert!(normalize("12345").is_err());
        assert!(normalize("1234567").is_err());
        assert!(normalize("abcdef").is_err());
    }

    #[test]
    fn the_confirmation_error_is_recognised() {
        let err = AppError::api("email_not_confirmed");
        assert!(needs_code(&err));
        let err = AppError::api("invalid_credentials");
        assert!(!needs_code(&err));
    }
}
