use crate::args::{Args, ARGS};
use crate::dbio::save_to_file;
use crate::endpoints::errors::render_error;
use crate::i18n::{current_path, I18n};
use crate::pasta::{find_by_key, Pasta};
use crate::util::misc::remove_expired;
use crate::AppState;

use actix_web::{get, web, HttpRequest, HttpResponse};
use askama::Template;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Template)]
#[template(path = "pasta.html", escape = "none")]
struct PastaTemplate<'a> {
    pasta: &'a Pasta,
    args: &'a Args,
    i18n: I18n,
    current_path: &'a str,
}

#[get("/pasta/{id}")]
pub async fn getpasta(
    data: web::Data<AppState>,
    id: web::Path<String>,
    request: HttpRequest,
) -> HttpResponse {
    // get access to the pasta collection
    let mut pastas = data.pastas.lock().unwrap();

    let key = id.into_inner();

    // remove expired pastas (including this one if needed)
    remove_expired(&mut pastas);

    if let Some(index) = find_by_key(&pastas, &key) {
        // increment read count
        pastas[index].read_count += 1;

        // save the updated read count
        save_to_file(&pastas);

        // serve pasta in template
        let path = current_path(&request);
        let response = HttpResponse::Ok().content_type("text/html").body(
            PastaTemplate {
                pasta: &pastas[index],
                args: &ARGS,
                i18n: I18n::from_request(&request),
                current_path: &path,
            }
            .render()
            .unwrap(),
        );

        // get current unix time in seconds
        let timenow: i64 = match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(n) => n.as_secs(),
            Err(_) => {
                log::error!("SystemTime before UNIX EPOCH!");
                0
            }
        } as i64;

        // update last read time
        pastas[index].last_read = timenow;

        return response;
    }

    // otherwise
    // send pasta not found error
    HttpResponse::Ok()
        .content_type("text/html")
        .body(render_error(&request))
}

#[get("/url/{id}")]
pub async fn redirecturl(
    data: web::Data<AppState>,
    id: web::Path<String>,
    request: HttpRequest,
) -> HttpResponse {
    // get access to the pasta collection
    let mut pastas = data.pastas.lock().unwrap();

    let key = id.into_inner();

    // remove expired pastas (including this one if needed)
    remove_expired(&mut pastas);

    if let Some(index) = find_by_key(&pastas, &key) {
        // increment read count
        pastas[index].read_count += 1;

        // save the updated read count
        save_to_file(&pastas);

        // send redirect if it's a url pasta
        if pastas[index].pasta_type == "url" {
            let response = HttpResponse::Found()
                .append_header(("Location", String::from(&pastas[index].content)))
                .finish();

            // get current unix time in seconds
            let timenow: i64 = match SystemTime::now().duration_since(UNIX_EPOCH) {
                Ok(n) => n.as_secs(),
                Err(_) => {
                    log::error!("SystemTime before UNIX EPOCH!");
                    0
                }
            } as i64;

            // update last read time
            pastas[index].last_read = timenow;

            return response;
        // send error if we're trying to open a non-url pasta as a redirect
        } else {
            return HttpResponse::Ok()
                .content_type("text/html")
                .body(render_error(&request));
        }
    }

    // otherwise
    // send pasta not found error
    HttpResponse::Ok()
        .content_type("text/html")
        .body(render_error(&request))
}

#[get("/raw/{id}")]
pub async fn getrawpasta(
    data: web::Data<AppState>,
    id: web::Path<String>,
    request: HttpRequest,
) -> String {
    // get access to the pasta collection
    let mut pastas = data.pastas.lock().unwrap();

    let key = id.into_inner();

    // remove expired pastas (including this one if needed)
    remove_expired(&mut pastas);

    if let Some(index) = find_by_key(&pastas, &key) {
        // increment read count
        pastas[index].read_count += 1;

        // get current unix time in seconds
        let timenow: i64 = match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(n) => n.as_secs(),
            Err(_) => {
                log::error!("SystemTime before UNIX EPOCH!");
                0
            }
        } as i64;

        // update last read time
        pastas[index].last_read = timenow;

        // save the updated read count
        save_to_file(&pastas);

        // send raw content of pasta
        return pastas[index].content.to_owned();
    }

    // otherwise
    // send pasta not found error as raw text
    I18n::from_request(&request).text("pasta_not_found").to_owned()
}
