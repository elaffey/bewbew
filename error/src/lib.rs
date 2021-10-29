use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Error {
    details: String,
}

impl Error {
    #[must_use]
    pub fn new(details: String) -> Self {
        Self { details }
    }

    #[must_use]
    pub fn wrap(msg: &str, inner: impl std::fmt::Display) -> Self {
        let details = format!("{} - {}", msg, inner);
        Self::new(details)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.details)
    }
}
