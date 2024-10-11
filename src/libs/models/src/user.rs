use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct AddUser {
    pub uuid: String,
    pub email: String,
    pub password: String,
}
