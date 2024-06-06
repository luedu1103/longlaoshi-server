use rocket::fs::{FileServer, relative};
use rocket::http::uri::Origin;
use rocket::response::Redirect;
use rocket::http::Header;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::{Request, Response};

mod ia;
mod chat_routes;
mod install_route;
mod voice;

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
    Redirect::to(uri!(GPTHOLA, longlaoshi_main_page()))
}

// endpoint
#[get("/testing")]
fn longlaoshi_main_page() -> Template {
    // Status::NoContent
    let mut context = HashMap::new();
    context.insert("hi","These are the static files c:");
    Template::render("index", &context)
}

// CORS thing
pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET, PATCH, OPTIONS"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}


#[shuttle_runtime::main]
async fn main(#[shuttle_runtime::Secrets] secrets: shuttle_runtime::SecretStore) -> shuttle_rocket::ShuttleRocket {

    let secret = secrets.get("OPENAI_API_KEY").unwrap();
    let assistant_id = secrets.get("ASSISTANT_ID").unwrap();
    let state = MyState { secret, assistant_id };

    let rocket = rocket::build()
        .mount("/", routes![index])
        .mount(GPTHOLA, routes![longlaoshi_main_page])
        .mount(chat_routes::BASE, chat_routes::routes())
        .mount(install_route::BASE, install_route::routes())
        .mount(voice::BASE, voice::routes())
        .mount("/static", FileServer::from(relative!("static")))
        .manage(state)
        .attach(CORS)
        .attach(Template::fairing());

    Ok(rocket.into())
}
