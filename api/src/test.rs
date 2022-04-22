use super::rocket;
use pretty_assertions::assert_eq;
use rocket::http::Status;
use rocket::local::blocking::Client;

#[test]
fn index() {
    let client = Client::tracked(rocket()).expect("valid rocket instace");
    let response = client.get("/").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.into_string(), Some("Hello, world!".into()));
}
