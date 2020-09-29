use serde::{Deserialize, Serialize};
use rocket::Route;
use rocket_contrib::json::{Json, JsonValue};

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
enum TodoType {
    Once{},
    Repeatable { times: u32 },
}

#[derive(Serialize, Deserialize)]
struct Todo {
    name: String,
    variant: TodoType,
    description: Option<String>,
    completed: bool,
}

#[post("/", data = "<body>")]
fn create_todo(body: Json<Todo>) -> JsonValue {
    json!({ "todo": body.0 })
}

pub fn routes() -> Vec<Route> {
    routes![ create_todo ]
}

#[cfg(test)]
mod test {
    use rocket;
    use rocket::local::Client;
    use rocket::http::Status;

    #[test]
    fn create_todo_invalid_payload() {
        let client = Client::new(crate::create_rocket()).expect("valid rocket instance");
        let mut response = client.post("/api/todo")
            .body(r#"{"invalid": "payload"}"#)
            .dispatch();
        assert_eq!(response.status(), Status::UnprocessableEntity);
    }

    #[test]
    fn create_todo_valid_payload() {
        let client = Client::new(crate::create_rocket()).expect("valid rocket instance");
        let mut response = client.post("/api/todo")
            .body(r#"{"completed": false, "name": "linde12", "variant": { "times": 42, "type": "Repeatable" } }"#)
            .dispatch();
        assert_eq!(response.status(), Status::Ok);
    }

    #[test]
    fn create_todo_some_description() {
        let client = Client::new(crate::create_rocket()).expect("valid rocket instance");
        let mut response = client.post("/api/todo")
            .body(r#"{"completed": false, "name": "linde12", "variant": { "times": 42, "type": "Repeatable" }, "description": "hello" }"#)
            .dispatch();
        assert_eq!(response.status(), Status::Ok);
    }
}
