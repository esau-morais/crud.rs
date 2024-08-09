use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AuthEntity {
    token: String,
}

impl AuthEntity {
    pub fn new(token: String) -> Self {
        Self { token }
    }
}
