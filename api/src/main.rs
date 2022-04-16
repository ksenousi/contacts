#[macro_use]
extern crate rocket;

use rocket::serde::json::Json;
use rocket::State;
use rusqlite::Connection;
use std::sync::{Arc, Mutex};

mod db;
use db::contact::Contact;

type DbConnection = Arc<Mutex<Connection>>;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/")]
fn get_contacts(conn: &State<DbConnection>) -> Json<Vec<Contact>> {
    let c = &conn.lock().unwrap();

    match db::get(c) {
        Ok(contacts) => Json(contacts),
        Err(error) => {
            println!("could not retrieve contacts {}", error);
            Json(Vec::new())
        }
    }
}

#[post("/", data = "<contact>")]
fn add_contact(
    conn: &State<DbConnection>,
    contact: Json<Contact>,
) -> Result<Json<Contact>, String> {
    let c = &conn.lock().unwrap();
    match db::add(c, &contact) {
        Ok(_) => Ok(contact),
        Err(error) => Err(format!("could not add contact {}", error)),
    }
}

#[launch]
fn rocket() -> _ {
    let conn = match db::init_db() {
        Ok(conn) => {
            println!("db initialised");
            conn
        }
        Err(error) => panic!("Failed to initialise db: {}", error),
    };

    rocket::build()
        .manage(Arc::new(Mutex::new(conn)))
        .mount("/", routes![index])
        .mount("/contacts", routes![get_contacts, add_contact])
}
