use crate::args::{Args, ARGS};
use crate::i18n::{current_path, I18n};
use crate::pasta::Pasta;
use crate::AppState;
use actix_web::{get, web, HttpRequest, HttpResponse};
use askama::Template;

#[derive(Template)]
#[template(path = "info.html")]
struct Info<'a> {
    args: &'a Args,
    pastas: &'a [Pasta],
    status: &'a str,
    version_string: &'a str,
    message: String,
    i18n: I18n,
    current_path: &'a str,
}

#[get("/info")]
pub async fn info(data: web::Data<AppState>, request: HttpRequest) -> HttpResponse {
    // get access to the pasta collection
    let pastas = data.pastas.lock().unwrap();

    // todo status report more sophisticated
    let mut status = "OK";
    let mut message = "";

    if ARGS.public_path.to_string() == "" {
        status = "WARNING";
        message = "warning_no_public_url"
    }

    let i18n = I18n::from_request(&request);
    let path = current_path(&request);
    HttpResponse::Ok().content_type("text/html").body(
        Info {
            args: &ARGS,
            pastas: &pastas,
            status: i18n.status(status),
            version_string: "1.2.0-20221107",
            message: i18n.text(message).to_owned(),
            i18n,
            current_path: &path,
        }
        .render()
        .unwrap(),
    )
}
