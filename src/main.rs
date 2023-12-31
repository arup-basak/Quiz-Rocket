#[macro_use] extern crate rocket;

use rocket::Request;
use rocket::serde::json::Json;
use rocket::serde::Serialize;

#[derive(Serialize)]
struct Message {
    body: String
}

#[get("/")]
fn index() -> Json<Message> {
    let data = Message {
        body: String::from("Hello World"),
    };
    Json(data)
}

#[catch(500)]
fn internal_error() -> &'static str {
    "Whoops! Looks like we messed up."
}

#[catch(404)]
fn not_found(req: &Request) -> String {
    format!("I couldn't find '{}'. Try something else?", req.uri())
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .register("/", catchers![internal_error, not_found])
}