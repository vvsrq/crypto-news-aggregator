use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NewsArticle {
    pub title: String,
    pub source: String,
    pub published_at: Option<DateTime<Utc>>,
    pub summary: String,
    pub url: String,
}

#[derive(Deserialize, Debug)]
pub struct NewsApiResponse {
    pub status: String,
    #[serde(rename = "totalResults")]
    pub total_results: Option<u32>,
   
    #[serde(default)] 
    pub articles: Vec<NewsApiArticle>,
    pub code: Option<String>,
    pub message: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct NewsApiArticle {
    /// Источник статьи представлен как вложенный объект.
    pub source: NewsApiSource,
    /// Автор статьи 
    pub author: Option<String>,
    pub title: String,
    /// Описание или краткое содержание 
    pub description: Option<String>,
    /// URL оригинальной статьи.
    pub url: String,

    /// URL изображения к статье 
    #[serde(rename = "urlToImage")]
    pub url_to_image: Option<String>,

    #[serde(rename = "publishedAt")]
    pub published_at: Option<DateTime<Utc>>,
    /// Полное содержание статьи 
    pub content: Option<String>,
}

/// Представляет источник новости внутри ответа NewsAPI.org.
#[derive(Deserialize, Debug)]
pub struct NewsApiSource {
    pub id: Option<String>,
    pub name: String,
}