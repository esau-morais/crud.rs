use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserEntity {
    pub id: i32,
    pub email: String,
    pub name: String,
    pub role: String,
}
