use reqwest::Client;
use std::env;

use crate::models::{NewsArticle, NewsApiResponse, NewsApiArticle}; 
use crate::errors::AppError;

const NEWSAPI_BASE_URL: &str = "https://newsapi.org/v2/everything";
const USER_AGENT: &str = "Rust Crypto News Aggregator ";

#[derive(Debug)] // это для логов 
pub struct ApiClient {
    client: Client,
    newsapi_key: String,
}

impl ApiClient {
    pub fn new() -> Result<Self, AppError> {
        let newsapi_key = env::var("NEWSAPI_KEY")
            .map_err(|_| AppError::ConfigError("NEWSAPI_KEY environment variable not set".into()))?;

        if newsapi_key.is_empty() {
             return Err(AppError::ConfigError("NEWSAPI_KEY environment variable is empty".into()));
        }

        let client = Client::builder()
            .user_agent(USER_AGENT) 
            .build()?; 

        Ok(Self {
            client,
            newsapi_key,
        })
    }

    pub async fn fetch_newsapi(&self, query: &str) -> Result<Vec<NewsArticle>, AppError> {
        let url = format!(
            "{}?q={}&language=en&sortBy=publishedAt&pageSize=20&apiKey={}",
            NEWSAPI_BASE_URL,
            query,
            self.newsapi_key
        );

        log::debug!("Requesting NewsAPI URL: {}", url);

        let response = self.client.get(&url).send().await?;

        let response = response.error_for_status()?;

        let api_response = response.json::<NewsApiResponse>().await?;

        log::debug!("Received NewsAPI response: Status='{}', TotalResults={:?}", api_response.status, api_response.total_results);

        // if api_response.status != "ok" {
        //     let error_message = api_response
        //         .message
        //         .unwrap_or_else(|| format!("Unknown API error code: {:?}", api_response.code));
        //     log::error!("NewsAPI API Error: {}", error_message);
        //     return Err(AppError::external_api_error("NewsAPI.org", error_message));
        // }

        let articles: Vec<NewsArticle> = api_response
            .articles 
            .into_iter() 
            .map(Self::convert_newsapi_article) 
            .collect(); 

        Ok(articles)
    }

    fn convert_newsapi_article(api_article: NewsApiArticle) -> NewsArticle {
        // --- ИСПРАВЛЕНИЕ 3: Измененная логика для summary ---
    let summary_text = api_article.description.as_deref().unwrap_or("No summary available."); // Получаем &str или дефолтный
    let mut summary = summary_text.chars().take(200).collect::<String>(); // Берем первые 200 символов

    // Проверяем, была ли строка длиннее 200 символов *в оригинале*
    if summary_text.chars().count() > 200 {
        summary.push_str("..."); // Добавляем многоточие, если обрезали
    }
        NewsArticle {
            title: api_article.title.clone(),
            source: format!("NewsAPI - {}", api_article.source.name),
            published_at: api_article.published_at,
            summary: summary,
            url: api_article.url,
        }
    }
}