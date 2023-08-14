use crate::controller::admin::admin_pagination::{
    admin_pagination_with_category, admin_pagination_with_category_2,
};
use crate::controller::constants::ConfigurationConstants;
use crate::controller::guests::common_controller::set_posts_per_page;
use crate::model::category::{category_pagination_controller_database_function, category_pagination_logic, get_all_categories_database};
use crate::model::posts::select_specific_pages_post;
use crate::model::single_posts::{query_single_post, query_single_post_in_struct};
use actix_http::header::LOCATION;
use actix_identity::Identity;
use actix_web::http::header::ContentType;
use actix_web::{http, HttpResponse, web};
use handlebars::Handlebars;
use serde_json::json;

pub async fn get_category_posts_a(
    info: web::Path<(String, i32)>,
    config: web::Data<ConfigurationConstants>,
    handlebars: web::Data<Handlebars<'_>>,
    user: Option<Identity>,
) -> Result<HttpResponse, actix_web::Error> {
    if user.is_none() {
        return Ok(HttpResponse::SeeOther()
            .insert_header((http::header::LOCATION, "/"))
            .body(""));
    }
    let db = &config.database_connection;
    let category_input: String = info.clone().0;
    let params = info.into_inner().1;

    let total_posts_length = category_pagination_logic(&category_input, db)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let all_category = get_all_categories_database(db)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let posts_per_page_constant = set_posts_per_page().await as i64;
    let mut posts_per_page = total_posts_length / posts_per_page_constant;
    let check_remainder = total_posts_length % posts_per_page_constant;
    if check_remainder != 0 {
        posts_per_page += 1;
    }
    let pages_count: Vec<_> = (1..=posts_per_page).collect();
    let count_of_number_of_pages = pages_count.len();
    let cp: usize = params.clone() as usize;

    let pagination_final_string =
        admin_pagination_with_category_2(cp, count_of_number_of_pages, category_input.clone())
            .await
            .map_err(actix_web::error::ErrorInternalServerError)?;

    let category_postinng = category_pagination_controller_database_function(
        category_input,
        db,
        params,
        posts_per_page_constant,
    )
    .await
    .map_err(actix_web::error::ErrorInternalServerError)?;

    let html = handlebars
        .render(
            "admin_separate_categories",
            &json!({"pagination":pagination_final_string,"tiger":&category_postinng,"pages_count":&pages_count,"o":all_category}),
        )
       .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(html))
}

pub async fn show_post(
    path: web::Path<String>,
    config: web::Data<ConfigurationConstants>,
    handlebars: web::Data<Handlebars<'_>>,
) -> Result<HttpResponse, actix_web::Error> {
    let db = &config.database_connection;
    let titles = path.parse::<i32>().unwrap_or_default();
    let single_post = query_single_post(titles, db)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let all_category = get_all_categories_database(db)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let single_post_struct = query_single_post_in_struct(titles, db)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let html = handlebars
        .render(
            "admin_single_post",
            &json!({"o":&single_post,"single_post":single_post_struct,"o":all_category}),
        )
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(html))
}
