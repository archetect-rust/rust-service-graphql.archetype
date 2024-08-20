use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Claims {
    sub: String,
    iss: String,
    scope: String,
}

impl Claims {
    pub fn subject(&self) -> &str {
        &self.sub
    }

    pub fn iss(&self) -> &str {
        &self.iss
    }

    pub fn scope(&self) -> &str {
        &self.scope
    }
}

pub struct Auth {
    token: String,
    claims: Claims,
}

impl Auth {
    pub fn new<T: Into<String>>(token: T, claims: Claims) -> Auth {
        Auth {
            token: token.into(),
            claims,
        }
    }

    /// Raw JWT
    ///
    /// This is provided for passing along to downstream APIs
    pub fn token(&self) -> &str {
        self.token.as_str()
    }

    pub fn claims(&self) -> &Claims {
        &self.claims
    }
}