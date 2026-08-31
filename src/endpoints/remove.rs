use actix_web::{get, web, HttpRequest, HttpResponse};

use crate::args::ARGS;
use crate::dbio::save_to_file;
use crate::endpoints::errors::render_error;
use crate::pasta::{find_by_key, PastaFile};
use crate::util::misc::remove_expired;
use crate::AppState;
use std::fs;

#[get("/remove/{id}")]
pub async fn remove(
    data: web::Data<AppState>,
    id: web::Path<String>,
    request: HttpRequest,
) -> HttpResponse {
    if ARGS.readonly {
        return HttpResponse::Found()
            .append_header(("Location", format!("{}/", ARGS.public_path)))
            .finish();
    }

    let mut pastas = data.pastas.lock().unwrap();

    let key = id.into_inner();

    if let Some(i) = find_by_key(&pastas, &key) {
        let pasta = &pastas[i];
        // remove the file itself
        if let Some(PastaFile { name, .. }) = &pasta.file {
            if fs::remove_file(format!(
                "./pasta_data/public/{}/{}",
                pasta.public_key(),
                name
            ))
            .is_err()
            {
                log::error!("Failed to delete file {}!", name)
            }

            // and remove the containing directory
            if fs::remove_dir(format!("./pasta_data/public/{}/", pasta.public_key())).is_err() {
                log::error!("Failed to delete directory {}!", name)
            }
        }
        // remove it from in-memory pasta list
        pastas.remove(i);
        save_to_file(&pastas);
        return HttpResponse::Found()
            .append_header(("Location", format!("{}/pastalist", ARGS.public_path)))
            .finish();
    }

    remove_expired(&mut pastas);

    HttpResponse::Ok()
        .content_type("text/html")
        .body(render_error(&request))
}
