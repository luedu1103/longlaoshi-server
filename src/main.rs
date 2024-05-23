use rocket::http::uri::Origin;
use rocket::http::Status;
use rocket::response::Redirect;
use rocket::serde::json::{json, Value};
use rocket::State;

pub mod ia;
use crate::ia::open_ai::chat_ai;

struct MyState {
    secret: String,
}

#[macro_use]
extern crate rocket;

const GPTHOLA: Origin<'static> = uri!("/");

#[get("/")]
fn index() -> Redirect {
    let msg: Option<&str> = None;
    Redirect::to(uri!(GPTHOLA, longlaoshi_main_page(msg)))
}

// endpoint
// /tauri-releases/google-keep-desktop&win64&1.18.0?msg=""
#[get("/longlaoshi?<msg>")]
fn longlaoshi_main_page(msg: Option<&str>) -> Result<Value, Status> {
    // Status::NoContent
    let mut string = "";
    if let Some(msg) = msg {
        string = msg;
    }

    Ok(json!({
        "notes": string
    }))
}

#[get("/chat_with_ai?<msg>")]
fn open_ai_chat(state: &State<MyState>, msg: String) -> Result<Value, Status> {
    let string = chat_ai::chat(msg, state.secret.to_owned()).unwrap_or(Some("Esto es un error".to_string()));

    Ok(json!({
        "notes": string
    }))
}

#[get("/create-conversation")]
fn open_ai_create_conversation(state: &State<MyState>) -> Result<Value, Status> {
    let chat_session = chat_ai::ChatSession::create_conversation(state.secret.to_owned()).unwrap();
    // println!("{:?}", chat_ai::conversation(state.secret.to_owned(), msg));
    Ok(json!({
        "id": chat_session.assistant_id,
        "thread": chat_session.thread_id,
        "message": "its working",
    }))
}

#[get("/chat-with-your-own-assistant/<id>/<thread>?<msg>")]
fn open_ai_chat_with_your_assistant(id: String, thread: String, msg: String, state: &State<MyState>) -> Result<Value, Status> {
    let response = chat_ai::talk(id, thread, msg, state.secret.to_owned()).unwrap();
    Ok(json!({
        "response": response
    }))
}

// #[launch]
// fn rocket() -> _ {
//     rocket::build()
//         .mount("/", routes![index])
//         .mount(GPTHOLA, routes![google_keep_desktop_api])
//         .mount(GPTHOLA, routes![open_ai_chat])
//         .configure(rocket::Config {
//             port: 8000,
//             ..Default::default()
//         })
// }

#[shuttle_runtime::main]
async fn main(#[shuttle_runtime::Secrets] secrets: shuttle_runtime::SecretStore) -> shuttle_rocket::ShuttleRocket {

    let secret = secrets.get("OPENAI_API_KEY").unwrap();
    let state = MyState { secret };

    let rocket = rocket::build()
        .mount("/", routes![index])
        .mount("/", routes![longlaoshi_main_page])
        .mount(GPTHOLA, routes![open_ai_chat])
        .mount("/", routes![open_ai_create_conversation])
        .mount("/", routes![open_ai_chat_with_your_assistant])
        .manage(state);

    Ok(rocket.into())
}
