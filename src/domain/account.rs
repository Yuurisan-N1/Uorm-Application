use crate::support::consts::app::CREDENTIAL_SEPARATOR;
use crate::support::log::format::mail;

#[derive(Clone)]
pub struct Account {
    pub email: String,
    pub password: String,
}

impl Account {
    pub fn parse_line(line: &str) -> Result<Account, String> {
        let trimmed = line.trim();
        let (email, password) = trimmed
            .split_once(CREDENTIAL_SEPARATOR)
            .ok_or_else(|| format!("expected email{}password", CREDENTIAL_SEPARATOR))?;
        let email = email.trim();
        let password = password.trim();
        if email.is_empty() {
            return Err("the email is empty".to_string());
        }
        if !email.contains('@') {
            return Err("the email is not an address".to_string());
        }
        if password.is_empty() {
            return Err("the password is empty".to_string());
        }
        if password.contains(CREDENTIAL_SEPARATOR) {
            return Err(format!(
                "the password holds a second {} separator",
                CREDENTIAL_SEPARATOR
            ));
        }
        Ok(Account {
            email: email.to_string(),
            password: password.to_string(),
        })
    }

    pub fn masked(&self) -> String {
        mail(&self.email)
    }
}

impl std::fmt::Debug for Account {
    fn fmt(&self, out: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        out.debug_struct("Account")
            .field("email", &self.masked())
            .field("password", &"[REDACTED]")
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pipe_separator_is_the_only_shape() {
        let account = Account::parse_line("workersample1@gmail.com|secret").unwrap();
        assert_eq!(account.email, "workersample1@gmail.com");
        assert_eq!(account.password, "secret");
    }

    #[test]
    fn colon_separator_is_rejected() {
        assert!(Account::parse_line("workersample1@gmail.com:secret").is_err());
    }

    #[test]
    fn missing_parts_are_rejected() {
        assert!(Account::parse_line("|secret").is_err());
        assert!(Account::parse_line("mail|").is_err());
        assert!(Account::parse_line("no-at-sign|secret").is_err());
        assert!(Account::parse_line("mail@x|a|b").is_err());
    }

    #[test]
    fn debug_redacts_the_password() {
        let account = Account::parse_line("a@b.com|secret").unwrap();
        let text = format!("{:?}", account);
        assert!(text.contains("[REDACTED]"));
        assert!(!text.contains("secret"));
    }
}
