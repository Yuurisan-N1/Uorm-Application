use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Clone, Serialize, Deserialize)]
pub struct Session {
    pub email: String,
    pub user_id: String,
    pub access: String,
    pub refresh: String,
    pub expires_at: i64,
}

impl Session {
    pub fn fresh(&self, now: i64) -> bool {
        self.expires_at - now > 120
    }
}

impl std::fmt::Debug for Session {
    fn fmt(&self, out: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        out.debug_struct("Session")
            .field("email", &self.email)
            .field("user_id", &self.user_id)
            .field("access", &"[REDACTED]")
            .field("refresh", &"[REDACTED]")
            .field("expires_at", &self.expires_at)
            .finish()
    }
}

pub fn key_for(email: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(b"uorm-session-v1|");
    hasher.update(email.trim().to_ascii_lowercase().as_bytes());
    let digest = hasher.finalize();
    digest
        .iter()
        .take(8)
        .map(|byte| format!("{:02x}", byte))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_key_is_sixteen_hex_digits() {
        let key = key_for("someone@mail.com");
        assert_eq!(key.len(), 16);
        assert!(key.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn the_key_ignores_the_case() {
        assert_eq!(key_for("Someone@Mail.com"), key_for("someone@mail.com"));
    }

    #[test]
    fn the_key_differs_per_account() {
        assert_ne!(key_for("a@b.com"), key_for("c@d.com"));
    }

    #[test]
    fn a_session_is_fresh_only_while_it_lasts() {
        let session = Session {
            email: "a@b.com".to_string(),
            user_id: "1".to_string(),
            access: "token".to_string(),
            refresh: "renew".to_string(),
            expires_at: 1_000,
        };
        assert!(session.fresh(800));
        assert!(!session.fresh(900));
    }

    #[test]
    fn debug_redacts_the_tokens() {
        let session = Session {
            email: "a@b.com".to_string(),
            user_id: "1".to_string(),
            access: "secret-access".to_string(),
            refresh: "secret-refresh".to_string(),
            expires_at: 1,
        };
        let text = format!("{:?}", session);
        assert!(!text.contains("secret"));
        assert!(text.contains("[REDACTED]"));
    }
}
