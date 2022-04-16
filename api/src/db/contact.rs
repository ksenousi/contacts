use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Contact {
    pub id: Option<i32>,
    pub name: String,
}
