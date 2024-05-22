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

const GPTHOLA: Origin<'static> = uri!("/long-laoshi");

#[get("/")]
fn index() -> Redirect {
    let msg: Option<&str> = None;
    Redirect::to(uri!(GPTHOLA, google_keep_desktop_api("windows-x86_64", "v1.0.14", msg)))
}

// endpoint
// /tauri-releases/google-keep-desktop&win64&1.18.0?msg=""
#[get("/google-keep-desktop/<_platform>/<_current_version>?<msg>")]
fn google_keep_desktop_api(_platform: &str, _current_version: &str, msg: Option<&str>) -> Result<Value, Status> {
    // Status::NoContent
    if let Some(msg) = msg {
        println!("{msg}");
        return  Err(Status::NoContent);
    }

    Ok(json!({
        "notes": "it works",
    }))
}

#[get("/chat_with_ai?<msg>")]
fn open_ai_chat(state: &State<MyState>, msg: String) -> Result<Value, Status> {
    let string = chat_ai::chat(msg, state.secret.to_owned()).unwrap_or(Some("un error parece".to_string()));

    Ok(json!({
        "notes": string
    }))
}

#[get("/conversation?<msg>")]
fn open_ai_convo(state: &State<MyState>, msg: String) -> Result<Value, Status> {
    let string = chat_ai::conversation(state.secret.to_owned(), msg).unwrap_or("un error parece".to_string());
    // println!("{:?}", chat_ai::conversation(state.secret.to_owned(), msg));
    Ok(json!({
        "notes": string
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
        .mount(GPTHOLA, routes![google_keep_desktop_api])
        .mount(GPTHOLA, routes![open_ai_chat])
        .mount("/", routes![open_ai_convo])
        .manage(state);

    Ok(rocket.into())
}
