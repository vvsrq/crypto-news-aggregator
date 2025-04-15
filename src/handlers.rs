use actix_web::{web, get, Responder, HttpResponse};
use askama::Template;
use serde::Deserialize;

use crate::api_client::ApiClient;
use crate::models::NewsArticle;
use crate::errors::AppError;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate<'a> {
    previous_query: Option<&'a str>,
}

#[derive(Template)]
#[template(path = "news_results.html")]
struct NewsResultsTemplate<'a> {
    query: &'a str,
    articles: Vec<NewsArticle>,
}

#[derive(Deserialize, Debug)]
pub struct NewsQuery {
    query: String,
}

#[get("/")]
async fn index() -> Result<HttpResponse, AppError> {
    let template = IndexTemplate { previous_query: None };
    let html_body = template.render()?;

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html_body))
}

#[get("/news")]
async fn get_news(
    query_params: web::Query<NewsQuery>,
    api_client: web::Data<ApiClient>,
) -> Result<HttpResponse, AppError> {
    let search_term = &query_params.query;

    log::info!("Received news request for query: {}", search_term);

    if search_term.trim().is_empty() {
        log::warn!("Empty query received.");
        return Ok(HttpResponse::BadRequest()
            .content_type("text/html; charset=utf-8")
            .body("<h1>Error</h1><p>Search query cannot be empty.</p><a href='/'>Try again</a>"));
    }

    let articles = api_client.fetch_newsapi(search_term).await?;

    log::info!("Successfully fetched {} articles for query: {}", articles.len(), search_term);

    let template = NewsResultsTemplate {
        query: search_term,
        articles,
    };

    let html_body = template.render()?;

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html_body))
}