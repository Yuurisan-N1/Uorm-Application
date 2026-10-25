use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("{0}")]
    Config(String),
    #[error("{0}")]
    Network(String),
    #[error("{0}")]
    Api(String),
}

impl AppError {
    pub fn config(message: impl Into<String>) -> AppError {
        AppError::Config(message.into())
    }

    pub fn network(message: impl Into<String>) -> AppError {
        AppError::Network(message.into())
    }

    pub fn api(message: impl Into<String>) -> AppError {
        AppError::Api(message.into())
    }

    pub fn kind(&self) -> &'static str {
        match self {
            AppError::Config(_) => "Config",
            AppError::Network(_) => "Network",
            AppError::Api(_) => "Api",
        }
    }
}

impl From<reqwest::Error> for AppError {
    fn from(err: reqwest::Error) -> AppError {
        if err.is_timeout() {
            AppError::Network("the request timed out".to_string())
        } else if err.is_connect() {
            AppError::Network("the connection could not be established".to_string())
        } else if err.is_decode() {
            AppError::Network("the reply could not be read".to_string())
        } else {
            AppError::Network("the request failed".to_string())
        }
    }
}

impl From<serde_json::Error> for AppError {
    fn from(_: serde_json::Error) -> AppError {
        AppError::Network("the reply could not be read".to_string())
    }
}

pub type AppResult<T> = Result<T, AppError>;

pub fn refusal(message: &str) -> String {
    let text = message.to_ascii_lowercase();
    let known = if text.contains("email not confirmed") {
        "the email is not confirmed yet"
    } else if text.contains("invalid login") || text.contains("invalid_credentials") {
        "a wrong email and password pair"
    } else if text.contains("otp_expired") || text.contains("token has expired") {
        "the confirmation code expired"
    } else if text.contains("session not complete") {
        "the mining session has not finished yet"
    } else if text.contains("no active") || text.contains("no session") {
        "no mining session is running"
    } else if text.contains("already claimed") || text.contains("duplicate") {
        "the reward was already claimed"
    } else if text.contains("not completed") || text.contains("incomplete") {
        "the requirement was not met yet"
    } else if text.contains("cooldown") || text.contains("too soon") {
        "the request came before the cooldown finished"
    } else if text.contains("rate limit") || text.contains("too many") {
        "the server is rate limiting the account"
    } else if text.contains("domain not allowed") {
        "the email domain is not allowed"
    } else if text.contains("user already registered") || text.contains("already registered") {
        "the email is already registered"
    } else if text.contains("jwt") || text.contains("unauthorized") {
        "the session was refused"
    } else {
        return short(message);
    };
    known.to_string()
}

fn short(message: &str) -> String {
    let clean: String = message
        .chars()
        .filter(|c| !c.is_control())
        .take(120)
        .collect();
    let clean = clean.trim();
    if clean.is_empty() {
        "the server refused the request without a reason".to_string()
    } else {
        format!("the server said {}", clean)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_refusal_is_written_for_a_human() {
        assert_eq!(
            refusal("session not complete"),
            "the mining session has not finished yet"
        );
        assert_eq!(
            refusal("Invalid login credentials"),
            "a wrong email and password pair"
        );
        assert_eq!(refusal("otp_expired"), "the confirmation code expired");
    }

    #[test]
    fn an_unknown_refusal_keeps_the_server_words() {
        let text = refusal("P0001: something odd");
        assert!(text.starts_with("the server said "));
        assert!(text.contains("something odd"));
    }

    #[test]
    fn an_empty_refusal_still_reads() {
        assert_eq!(
            refusal("   "),
            "the server refused the request without a reason"
        );
    }

    #[test]
    fn the_kind_names_the_layer() {
        assert_eq!(AppError::api("x").kind(), "Api");
        assert_eq!(AppError::config("x").kind(), "Config");
    }
}
