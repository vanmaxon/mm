use actix_web::{get, web, HttpResponse};
use askama::Template;

use crate::args::{Args, ARGS};
use crate::i18n::{current_path, I18n};
use crate::pasta::Pasta;
use crate::util::misc::remove_expired;
use crate::AppState;
use actix_web::HttpRequest;

#[derive(Template)]
#[template(path = "pastalist.html")]
struct PastaListTemplate<'a> {
    pastas: &'a [Pasta],
    args: &'a Args,
    i18n: I18n,
    current_path: &'a str,
}

#[get("/pastalist")]
pub async fn list(data: web::Data<AppState>, request: HttpRequest) -> HttpResponse {
    if ARGS.no_listing {
        return HttpResponse::Found()
            .append_header(("Location", format!("{}/", ARGS.public_path)))
            .finish();
    }

    let mut pastas = data.pastas.lock().unwrap();

    remove_expired(&mut pastas);

    let path = current_path(&request);
    HttpResponse::Ok().content_type("text/html").body(
        PastaListTemplate {
            pastas: &pastas,
            args: &ARGS,
            i18n: I18n::from_request(&request),
            current_path: &path,
        }
        .render()
        .unwrap(),
    )
}
