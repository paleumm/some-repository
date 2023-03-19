use rocket::http::uri::Origin;
use rocket::http::Status;
use rocket::response::Redirect;
use rocket::serde::json::{json, Value};

#[macro_use]
extern crate rocket;

const RELEASE_PREFIX: Origin<'static> = uri!("/backend");

#[get("/")]
fn index() -> Redirect {
    let msg: Option<&str> = None;
    Redirect::to(uri!(
        RELEASE_PREFIX,
        google_keep_desktop_api("windows-x86_64", "v1.0.14", msg)
    ))
}

#[get("/google-keep-desktop/<_platform>/<current_version>?<msg>")]
fn google_keep_desktop_api(
    _platform: &str,
    current_version: &str,
    msg: Option<&str>,
) -> Result<Value, Status> {
    if let Some(msg) = msg {
        println!("{msg}");
        return Err(Status::NoContent);
    }
    Ok(json!({
        "notes":"IT WORKS"
    }))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount(RELEASE_PREFIX, routes![google_keep_desktop_api])
}
