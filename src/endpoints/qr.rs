use crate::args::{Args, ARGS};
use crate::endpoints::errors::render_error;
use crate::i18n::{current_path, I18n};
use crate::pasta::{find_by_key, Pasta};
use crate::util::misc::{self, remove_expired};
use crate::AppState;
use actix_web::{get, web, HttpRequest, HttpResponse};
use askama::Template;

#[derive(Template)]
#[template(path = "qr.html", escape = "none")]
struct QRTemplate<'a> {
    qr: &'a String,
    pasta: &'a Pasta,
    args: &'a Args,
    i18n: I18n,
    current_path: &'a str,
}

#[get("/qr/{id}")]
pub async fn getqr(
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
        // generate the QR code as an SVG - if its a file or text pastas, this will point to the /pasta endpoint, otherwise to the /url endpoint, essentially directly taking the user to the url stored in the pasta
        let svg: String = match pastas[index].pasta_type.as_str() {
            "url" => misc::string_to_qr_svg(format!("{}/url/{}", &ARGS.public_path, &key).as_str()),
            _ => misc::string_to_qr_svg(format!("{}/pasta/{}", &ARGS.public_path, &key).as_str()),
        };

        // serve qr code in template
        let path = current_path(&request);
        return HttpResponse::Ok().content_type("text/html").body(
            QRTemplate {
                qr: &svg,
                pasta: &pastas[index],
                args: &ARGS,
                i18n: I18n::from_request(&request),
                current_path: &path,
            }
            .render()
            .unwrap(),
        );
    }

    // otherwise
    // send pasta not found error
    HttpResponse::Ok()
        .content_type("text/html")
        .body(render_error(&request))
}
