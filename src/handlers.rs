use actix_web::{get, HttpResponse, Responder, web, error};
use anyhow::{Result};

use crate::config;
use crate::rss;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("hello")
}

#[get("/config")]
async fn conf(config: web::Data::<config::Root>) -> impl Responder {
    HttpResponse::Ok()
        .content_type(actix_web::http::header::ContentType(mime::TEXT_PLAIN_UTF_8))
        .body(config.podcasts.filtered[0].title.clone())
}

#[get("/rss/{name}")]
async fn rss_handler(path: web::Path<String>, config: web::Data::<config::Root>) -> actix_web::Result<impl Responder> {
    let podcast_name = path.into_inner();

    let pconf = match get_podcast_config(&podcast_name, &config) {
        Some(pc) => pc,
        None => return Err(error::ErrorNotFound(format!("Unknown podcast {}", podcast_name)))
    };

    let rss_contenat = 
        load_rss(&pconf.source_url).await.map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    let filtered = rss::filter_rss(&rss_contenat, &pconf.filter, pconf.title.as_str())
        .map_err(|e|actix_web::error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().body(filtered))
}

fn get_podcast_config<'a>(podcast: &str, config: &'a config::Root) -> Option<&'a config::Filtered> {
    config.podcasts.filtered.iter().find(|p| p.route == podcast)
}

async fn load_rss(url: &url::Url) -> Result<String> {
    let response = reqwest::get(url.clone()).await?.bytes().await?;
    let response_string = String::from_utf8(response.to_vec())?;
    Ok(response_string)
}
