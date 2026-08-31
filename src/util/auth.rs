use actix_web::dev::ServiceRequest;
use actix_web::{error, Error};
use actix_web_httpauth::extractors::basic::BasicAuth;

use crate::args::ARGS;
use crate::i18n::I18n;

pub async fn auth_validator(
    req: ServiceRequest,
    credentials: BasicAuth,
) -> Result<ServiceRequest, Error> {
    let invalid_login = I18n::from_request(req.request()).text("invalid_login");
    // check if username matches
    if credentials.user_id().as_ref() == ARGS.auth_username.as_ref().unwrap() {
        return match ARGS.auth_password.as_ref() {
            Some(cred_pass) => match credentials.password() {
                None => Err(error::ErrorBadRequest(invalid_login)),
                Some(arg_pass) => {
                    if arg_pass == cred_pass {
                        Ok(req)
                    } else {
                        Err(error::ErrorBadRequest(invalid_login))
                    }
                }
            },
            None => Ok(req),
        };
    } else {
        Err(error::ErrorBadRequest(invalid_login))
    }
}
