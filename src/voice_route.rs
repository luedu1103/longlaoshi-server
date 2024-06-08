use rocket::http::uri::Origin;
use rocket::{Route, State};
use rocket::serde::json::{json, Value};
use rocket::http::Status;
use rocket::serde::{json::Json, Deserialize, Serialize};

use crate::ia;
use crate::MyState;

pub const BASE: Origin<'static> = uri!("/longlaoshi");

pub fn routes() -> Vec<Route> {
    routes![text_to_speech, handle_options_request]
}

#[derive(Debug, Deserialize, Serialize)]
struct Message {
    content: String,
}

#[post("/testing-voice", format = "json", data = "<message_json>")]
async fn text_to_speech(state: &State<MyState>, message_json: Json<Message>) -> Result<Value, Status>{
    match ia::text_speech::generate_speech(&message_json.content, state.secret.to_owned()) {
        Ok(_) => {
            Ok(json!({
                "it works?": "it does!!!!"
            }))
        },
        Err(_) => {
            Ok(json!({
                "it works?": "it doesn´t"
            }))
        },
    }
}

#[options("/testing-voice")]
fn handle_options_request() -> Status {
    Status::NoContent
}