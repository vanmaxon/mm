use crate::dbio::save_to_file;
use crate::i18n::{current_path, I18n};
use crate::pasta::{generated_key, key_is_available, normalize_custom_key, PastaFile};
use crate::util::misc::is_valid_url;
use crate::{AppState, Pasta, ARGS};
use actix_multipart::Multipart;
use actix_web::{get, web, Error, HttpRequest, HttpResponse, Responder};
use askama::Template;
use bytesize::ByteSize;
use futures::TryStreamExt;
use log::warn;
use rand::Rng;
use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate<'a> {
    args: &'a ARGS,
    i18n: I18n,
    current_path: &'a str,
    error_message: Option<&'a str>,
    custom_key: &'a str,
    content: &'a str,
}

#[get("/")]
pub async fn index(request: HttpRequest) -> impl Responder {
    let path = current_path(&request);
    HttpResponse::Ok()
        .content_type("text/html")
        .body(
            IndexTemplate {
                args: &ARGS,
                i18n: I18n::from_request(&request),
                current_path: &path,
                error_message: None,
                custom_key: "",
                content: "",
            }
            .render()
            .unwrap(),
        )
}

fn render_error(
    request: &HttpRequest,
    status: actix_web::http::StatusCode,
    error_message: &'static str,
    custom_key: &str,
    content: &str,
) -> HttpResponse {
    let path = current_path(request);
    HttpResponse::build(status)
        .content_type("text/html")
        .body(
            IndexTemplate {
                args: &ARGS,
                i18n: I18n::from_request(request),
                current_path: &path,
                error_message: Some(error_message),
                custom_key,
                content,
            }
            .render()
            .unwrap(),
        )
}

fn remove_staged_file(key: &str, file: Option<&PastaFile>) {
    if let Some(file) = file {
        let path = format!("./pasta_data/public/{}/{}", key, file.name());
        if std::fs::remove_file(path).is_err() {
            log::warn!("Failed to clean up staged file {}", file.name());
        }
        if std::fs::remove_dir(format!("./pasta_data/public/{}/", key)).is_err() {
            log::warn!("Failed to clean up staged directory {}", key);
        }
    }
}

pub async fn create(
    data: web::Data<AppState>,
    request: HttpRequest,
    mut payload: Multipart,
) -> Result<HttpResponse, Error> {
    if ARGS.readonly {
        return Ok(HttpResponse::Found()
            .append_header(("Location", format!("{}/", ARGS.public_path)))
            .finish());
    }

    let mut pastas = data.pastas.lock().unwrap();

    let timenow: i64 = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => n.as_secs(),
        Err(_) => {
            log::error!("SystemTime before UNIX EPOCH!");
            0
        }
    } as i64;

    let id = loop {
        let id = rand::thread_rng().gen::<u16>() as u64;
        if !pastas.iter().any(|pasta| pasta.id == id)
            && key_is_available(&pastas, &generated_key(id))
        {
            break id;
        }
    };

    let generated_slug = generated_key(id);
    let mut new_pasta = Pasta {
        id,
        custom_key: None,
        content: String::from("No Text Content"),
        file: None,
        extension: String::from(""),
        private: false,
        editable: false,
        created: timenow,
        read_count: 0,
        burn_after_reads: 0,
        last_read: timenow,
        pasta_type: String::from(""),
        expiration: 0,
    };

    let mut custom_key_input = String::new();
    let mut content_input = String::new();

    while let Some(mut field) = payload.try_next().await? {
        match field.name() {
            "editable" => {
                // while let Some(_chunk) = field.try_next().await? {}
                new_pasta.editable = true;
                continue;
            }
            "private" => {
                // while let Some(_chunk) = field.try_next().await? {}
                new_pasta.private = true;
                continue;
            }
            "custom_key" => {
                while let Some(chunk) = field.try_next().await? {
                    custom_key_input.push_str(std::str::from_utf8(&chunk).unwrap());
                }
                continue;
            }
            "expiration" => {
                while let Some(chunk) = field.try_next().await? {
                    new_pasta.expiration = match std::str::from_utf8(&chunk).unwrap() {
                        "1min" => timenow + 60,
                        "10min" => timenow + 60 * 10,
                        "1hour" => timenow + 60 * 60,
                        "24hour" => timenow + 60 * 60 * 24,
                        "3days" => timenow + 60 * 60 * 24 * 3,
                        "1week" => timenow + 60 * 60 * 24 * 7,
                        "never" => {
                            if ARGS.no_eternal_pasta {
                                timenow + 60 * 60 * 24 * 7
                            } else {
                                0
                            }
                        }
                        _ => {
                            log::error!("{}", "Unexpected expiration time!");
                            timenow + 60 * 60 * 24 * 7
                        }
                    };
                }

                continue;
            }
            "burn_after" => {
                while let Some(chunk) = field.try_next().await? {
                    new_pasta.burn_after_reads = match std::str::from_utf8(&chunk).unwrap() {
                        // give an extra read because the user will be redirected to the pasta page automatically
                        "1" => 2,
                        "10" => 10,
                        "100" => 100,
                        "1000" => 1000,
                        "10000" => 10000,
                        "0" => 0,
                        _ => {
                            log::error!("{}", "Unexpected burn after value!");
                            0
                        }
                    };
                }

                continue;
            }
            "content" => {
                while let Some(chunk) = field.try_next().await? {
                    content_input.push_str(std::str::from_utf8(&chunk).unwrap());
                }
                if !content_input.is_empty() {
                    new_pasta.content = content_input.clone();

                    new_pasta.pasta_type = if is_valid_url(new_pasta.content.as_str()) {
                        String::from("url")
                    } else {
                        String::from("text")
                    };
                }
                continue;
            }
            "syntax-highlight" => {
                while let Some(chunk) = field.try_next().await? {
                    new_pasta.extension = std::str::from_utf8(&chunk).unwrap().to_string();
                }
                continue;
            }
            "file" => {
                if ARGS.no_file_upload {
                    continue;
                }

                let path = field.content_disposition().get_filename();

                let path = match path {
                    Some("") => continue,
                    Some(p) => p,
                    None => continue,
                };

                let mut file = match PastaFile::from_unsanitized(path) {
                    Ok(f) => f,
                    Err(e) => {
                        warn!("Unsafe file name: {e:?}");
                        continue;
                    }
                };

                std::fs::create_dir_all(format!(
                    "./pasta_data/public/{}",
                    &generated_slug
                ))
                .unwrap();

                let filepath = format!(
                    "./pasta_data/public/{}/{}",
                    &generated_slug,
                    &file.name()
                );

                let mut f = web::block(move || std::fs::File::create(filepath)).await??;
                let mut size = 0;
                while let Some(chunk) = field.try_next().await? {
                    size += chunk.len();
                    f = web::block(move || f.write_all(&chunk).map(|_| f)).await??;
                }

                file.size = ByteSize::b(size as u64);

                new_pasta.file = Some(file);
                new_pasta.pasta_type = String::from("text");
            }
            field => {
                log::error!("Unexpected multipart field:  {}", field);
            }
        }
    }

    let custom_key = match normalize_custom_key(&custom_key_input) {
        Ok(custom_key) => custom_key,
        Err(error) => {
            remove_staged_file(&generated_slug, new_pasta.file.as_ref());
            return Ok(render_error(
                &request,
                actix_web::http::StatusCode::UNPROCESSABLE_ENTITY,
                error,
                &custom_key_input,
                &content_input,
            ));
        }
    };

    if let Some(custom_key) = &custom_key {
        if !key_is_available(&pastas, custom_key) {
            remove_staged_file(&generated_slug, new_pasta.file.as_ref());
            return Ok(render_error(
                &request,
                actix_web::http::StatusCode::CONFLICT,
                "duplicate_key",
                custom_key,
                &content_input,
            ));
        }

        if custom_key != &generated_slug && new_pasta.file.is_some() {
            if let Err(error) = std::fs::rename(
                format!("./pasta_data/public/{}", generated_slug),
                format!("./pasta_data/public/{}", custom_key),
            ) {
                remove_staged_file(&generated_slug, new_pasta.file.as_ref());
                return Err(actix_web::error::ErrorInternalServerError(error));
            }
        }
    }
    new_pasta.custom_key = custom_key;

    pastas.push(new_pasta);

    save_to_file(&pastas);

    let slug = pastas.last().unwrap().public_key();
    Ok(HttpResponse::Found()
        .append_header(("Location", format!("{}/pasta/{}", ARGS.public_path, slug)))
        .finish())
}
