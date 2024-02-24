use serde::Deserialize;
use std::fmt::{Display, Formatter};

#[derive(Deserialize, Debug)]
pub struct PublicLoginDTO {
    pub name: String
}

impl Display for PublicLoginDTO {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {

        write!(f, " \"name\": {}", self.name)
    }
}