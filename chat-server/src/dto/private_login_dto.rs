use serde::Deserialize;
use std::fmt::{Display, Formatter};

#[derive(Deserialize, Debug)]
pub struct PrivateLoginDTO {
    pub username: String,
    pub password: String
}

impl Display for PrivateLoginDTO {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {

        write!(f, "\"name\": {}, \"password\": {}", self.username, self.password)
    }
}