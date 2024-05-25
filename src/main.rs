use rocket::http::uri::Origin;
use rocket::response::Redirect;

mod ia;
mod chat;

pub struct MyState {
    secret: String,
    assistant_id: String,
}

#[macro_use]
extern crate rocket;

use rocket_dyn_templates::Template;
use std::collections::HashMap;

const GPTHOLA: Origin<'static> = uri!("/longlaoshi");

#[get("/")]
fn index() -> Redirect {
    let msg: Option<&str> = None;
    Redirect::to(uri!(GPTHOLA, longlaoshi_main_page(msg)))
}

// endpoint
#[get("/testing?<msg>")]
fn longlaoshi_main_page(msg: Option<&str>) -> Template {
    // Status::NoContent
    let mut context = HashMap::new();
    context.insert("name", "World");
    Template::render("index", &context)
}

#[shuttle_runtime::main]
async fn main(#[shuttle_runtime::Secrets] secrets: shuttle_runtime::SecretStore) -> shuttle_rocket::ShuttleRocket {

    let secret = secrets.get("OPENAI_API_KEY").unwrap();
    let assistant_id = secrets.get("ASSISTANT_ID").unwrap();
    let state = MyState { secret, assistant_id };

    let rocket = rocket::build()
        .mount("/", routes![index])
        .mount(GPTHOLA, routes![longlaoshi_main_page])
        .mount(chat::BASE, chat::routes())
        .manage(state)
        .attach(Template::fairing());

    Ok(rocket.into())
}
