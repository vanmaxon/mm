use crate::args::Args;
use crate::dbio::save_to_file;
use crate::endpoints::errors::render_error;
use crate::i18n::{current_path, I18n};
use crate::pasta::find_by_key;
use crate::util::misc::remove_expired;
use crate::{AppState, Pasta, ARGS};
use actix_multipart::Multipart;
use actix_web::{get, post, web, Error, HttpRequest, HttpResponse};
use askama::Template;
use futures::TryStreamExt;

#[derive(Template)]
#[template(path = "edit.html", escape = "none")]
struct EditTemplate<'a> {
    pasta: &'a Pasta,
    args: &'a Args,
    i18n: I18n,
    current_path: &'a str,
}

#[get("/edit/{id}")]
pub async fn get_edit(
    data: web::Data<AppState>,
    id: web::Path<String>,
    request: HttpRequest,
) -> HttpResponse {
    let mut pastas = data.pastas.lock().unwrap();
    let key = id.into_inner();

    remove_expired(&mut pastas);

    if let Some(index) = find_by_key(&pastas, &key) {
        let pasta = &pastas[index];
        if !pasta.editable {
            return HttpResponse::Found()
                .append_header(("Location", format!("{}/", ARGS.public_path)))
                .finish();
        }
        let path = current_path(&request);
        return HttpResponse::Ok()
            .content_type("text/html")
            .body(
                EditTemplate {
                    pasta,
                    args: &ARGS,
                    i18n: I18n::from_request(&request),
                    current_path: &path,
                }
                .render()
                .unwrap(),
            );
    }

    HttpResponse::Ok()
        .content_type("text/html")
        .body(render_error(&request))
}

#[post("/edit/{id}")]
pub async fn post_edit(
    data: web::Data<AppState>,
    id: web::Path<String>,
    request: HttpRequest,
    mut payload: Multipart,
) -> Result<HttpResponse, Error> {
    if ARGS.readonly {
        return Ok(HttpResponse::Found()
            .append_header(("Location", format!("{}/", ARGS.public_path)))
            .finish());
    }

    let key = id.into_inner();

    let mut pastas = data.pastas.lock().unwrap();

    remove_expired(&mut pastas);

    let mut new_content = String::from("");

    while let Some(mut field) = payload.try_next().await? {
        if field.name() == "content" {
            while let Some(chunk) = field.try_next().await? {
                new_content = std::str::from_utf8(&chunk).unwrap().to_string();
            }
        }
    }

    if let Some(i) = find_by_key(&pastas, &key) {
        if pastas[i].editable {
            pastas[i].content = new_content;
            save_to_file(&pastas);

            return Ok(HttpResponse::Found()
                .append_header((
                    "Location",
                    format!("{}/pasta/{}", ARGS.public_path, pastas[i].public_key()),
                ))
                .finish());
        }
    }

    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(render_error(&request)))
}
