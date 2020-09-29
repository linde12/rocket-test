#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
use rocket::Request;

#[macro_use]
extern crate rocket_contrib;
use rocket_contrib::json::JsonValue;

pub mod todo;

#[get("/")]
fn docs() -> &'static str {
    "hello world!"
}

#[catch(400)]
fn bad_request() -> JsonValue {
    json!({
        "error": "Bad Request",
        "code": 400,
        "message": "Badly formatted JSON"
    })
}

#[catch(422)]
fn unprocessable_entity(req: &Request) -> JsonValue {
    json!({
        "error": "Unprocessable Entity",
        "code": 422,
        "message": "Invalid entity. Maybe you are missing a field?"
    })
}

#[catch(500)]
fn internal_server_error() -> JsonValue {
    json!({
        "error": "Internal Server Error",
        "code": 500,
        "message": "Oops."
    })
}

#[catch(404)]
fn not_found() -> JsonValue {
    json!({
        "error": "Not found",
        "code": 404,
    })
}

fn create_rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/api", routes![ docs ])
        .mount("/api/todo", todo::routes())
        .register(catchers![not_found, internal_server_error, unprocessable_entity, bad_request])
}
fn main() {
    create_rocket().launch();
}

#[cfg(test)]
mod test {
    use rocket;
    use rocket::local::Client;
    use rocket::http::Status;

    #[test]
    fn docs() {
        let client = Client::new(crate::create_rocket()).expect("valid rocket instance");
        let mut response = client.get("/api").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.body_string(), Some("hello world!".into()));
    }
}
