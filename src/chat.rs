use rocket::http::uri::Origin;
use rocket::{Route, State};
use rocket::serde::json::{json, Value};
use rocket::http::Status;

use crate::ia;
use crate::MyState;

pub const BASE: Origin<'static> = uri!("/longlaoshi");

pub fn routes() -> Vec<Route> {
    routes![open_ai_create_conversation, open_ai_chat, open_ai_chat_with_your_assistant]
}

#[get("/chat_with_ai?<msg>")]
fn open_ai_chat(state: &State<MyState>, msg: String) -> Result<Value, Status> {
    let string = ia::open_ai::chat(msg, state.secret.to_owned()).unwrap_or(Some("Esto es un error".to_string()));

    Ok(json!({
        "notes": string
    }))
}

#[get("/create-conversation")]
fn open_ai_create_conversation(state: &State<MyState>) -> Result<Value, Status> {
    let chat_session = ia::open_ai::ChatSession::create_conversation(state.secret.to_owned()).unwrap();
    Ok(json!({
        "thread": chat_session.thread_id,
        "message": "its working",
    }))
}

#[get("/chat-with-your-own-assistant/<thread>?<msg>")]
fn open_ai_chat_with_your_assistant(thread: String, msg: String, state: &State<MyState>) -> Result<Value, Status> {
    println!("Received request: thread = {}, msg = {}", thread, msg);
    match ia::open_ai::talk(state.assistant_id.to_owned(), thread, msg, state.secret.to_owned()) {
        Ok(response) => {
            Ok(json!({
                "response": response
            }))
        }
        Err(e) => {
            println!("Error processing request: {:?}", e);
            Err(Status::InternalServerError)
        }
    }
}