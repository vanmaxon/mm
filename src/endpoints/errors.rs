use actix_web::{Error, HttpRequest, HttpResponse};
use askama::Template;

use crate::args::{Args, ARGS};
use crate::i18n::{current_path, I18n};

#[derive(Template)]
#[template(path = "error.html")]
pub struct ErrorTemplate<'a> {
    pub args: &'a Args,
    pub i18n: I18n,
    pub current_path: &'a str,
}

pub fn render_error(request: &HttpRequest) -> String {
    let path = current_path(request);
    ErrorTemplate {
        args: &ARGS,
        i18n: I18n::from_request(request),
        current_path: &path,
    }
    .render()
    .unwrap()
}

pub async fn not_found(request: HttpRequest) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(render_error(&request)))
}
