use std::{fs};

use actix_web::{web, HttpResponse, Responder};
use mime_guess::from_path;
use rust_embed::RustEmbed;

use crate::args::ARGS;

#[derive(RustEmbed)]
#[folder = "templates/assets/"]
struct Asset;

fn handle_embedded_file(path: &str) -> HttpResponse {
    match Asset::get(path) {
        Some(content) => HttpResponse::Ok()
            .content_type(from_path(path).first_or_octet_stream().as_ref())
            .body(content.data.into_owned()),
        None => HttpResponse::NotFound().body("404 Not Found"),
    }
}

#[actix_web::get("/static/{_:.*}")]
async fn static_resources(path: web::Path<String>) -> impl Responder {
    handle_embedded_file(path.as_str())
}

#[actix_web::get("/robots.txt")]
async fn static_resources_robots() -> impl Responder {
    match &ARGS.robots_txt_path {
        Some(path) => {
            match fs::read_to_string(path) {
                Ok(content) => HttpResponse::Ok()
                    .content_type(from_path(path).first_or_octet_stream().as_ref())
                    .body(content),
                Err(_) => HttpResponse::InternalServerError().body("Misconfigured path. Please contact server administrator")
            }
        },
        None => handle_embedded_file("robots.txt"),
    }
}