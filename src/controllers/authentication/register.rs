use crate::controllers::authentication::session::User;
use crate::controllers::constants::Configuration;
use crate::model::authentication::register::register_user;
use actix_http::header::LOCATION;
use actix_web::http::header::ContentType;
use actix_web::{web, HttpResponse};
use handlebars::Handlebars;
use magic_crypt::MagicCryptTrait;
use serde_json::json;

pub async fn get_register(
    handlebars: web::Data<Handlebars<'_>>,
) -> Result<HttpResponse, actix_web::Error> {
    let html = handlebars
        .render("auth-register-basic", &json!({"yy":"welcome"}))
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(html))
}

pub async fn register(
    form: web::Form<User>,
    config: web::Data<Configuration>,
) -> Result<HttpResponse, actix_web::Error> {
    let user = &form.username;
    let password = &form.password;
    let mcrypt = &config.magic_key;
    let db = &config.database_connection;
    let encrypted_password = mcrypt.encrypt_str_to_base64(password);
    register_user(user, encrypted_password, db)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::SeeOther()
        .insert_header((LOCATION, "/login"))
        .finish())
}
