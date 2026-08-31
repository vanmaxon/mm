use actix_web::cookie::{time::Duration, Cookie};
use actix_web::{get, web, HttpRequest, HttpResponse};
use serde::Deserialize;

use crate::i18n::{I18n, Language, LANGUAGE_COOKIE};

#[derive(Deserialize)]
pub struct LanguageQuery {
    next: Option<String>,
}

fn is_safe_redirect(path: &str) -> bool {
    path.starts_with('/')
        && !path.starts_with("//")
        && !path.contains('\\')
        && !path.contains("://")
        && !path.bytes().any(|byte| byte < 0x20 || byte == 0x7f)
}

#[get("/language/{language}")]
pub async fn switch(
    request: HttpRequest,
    language: web::Path<String>,
    query: web::Query<LanguageQuery>,
) -> HttpResponse {
    let i18n = I18n::from_request(&request);
    let language_code = language.into_inner();
    let language = match Language::from_code(&language_code) {
        Some(language) => language,
        None => {
            return HttpResponse::BadRequest()
                .content_type("text/plain; charset=utf-8")
                .body(i18n.text("invalid_locale"))
        }
    };

    let next = query
        .next
        .as_deref()
        .filter(|path| is_safe_redirect(path))
        .unwrap_or("/");
    let cookie = Cookie::build(LANGUAGE_COOKIE, language.code())
        .path("/")
        .http_only(true)
        .max_age(Duration::days(365))
        .finish();

    HttpResponse::Found()
        .append_header(("Location", next))
        .cookie(cookie)
        .finish()
}

#[cfg(test)]
mod tests {
    use super::is_safe_redirect;

    #[test]
    fn accepts_same_origin_relative_paths() {
        assert!(is_safe_redirect("/pasta/ant"));
        assert!(is_safe_redirect("/pasta/ant?view=raw"));
    }

    #[test]
    fn rejects_external_redirects() {
        assert!(!is_safe_redirect("//example.com"));
        assert!(!is_safe_redirect("https://example.com"));
        assert!(!is_safe_redirect("/\\\\example.com"));
    }
}
