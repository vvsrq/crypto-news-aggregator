use actix_web::{get, web, App, HttpServer, Responder, HttpResponse};
use actix_files::Files;
use dotenv::dotenv;
use std::io::Result;
use env_logger::Env;
use log;
use std::env;
use actix_web::middleware::Logger;

mod handlers;
mod api_client;
mod models;
mod errors;

#[get("/health")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("OK")
}

#[actix_web::main]
async fn main() ->Result<()>{
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    log::info!("Starting server...");
    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port_str = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let port: u16 = port_str
                        .parse()
                        .expect("PORT environment variable must be a valid u16 number");

    let api_client = match api_client::ApiClient::new() {
        Ok(client) => {
            log::info!("ApiClient initialized successfully.");
            web::Data::new(client)
        }
        Err(e) => {
            log::error!("Failed to initialize ApiClient: {}", e);
            eprintln!("FATAL ERROR: Could not initialize API client. Please check configuration (e.g., NEWSAPI_KEY). Error: {}", e);
            std::process::exit(1);
        }
    };

    log::info!("Server is starting on http://{}:{}", host, port);

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(api_client.clone())

            .service(handlers::index) // "/"
            .service(handlers::get_news) // "/news"

            // Запросы к `/static/*` будут искать файлы в папке `./static`.
            // .service(Files::new("/static", "./static")
            //          //.show_files_listing() // Можно включить для отладки, показывает список файлов в директории
            //          .use_last_modified(true)) // Помогает с кешированием в браузере
    })

    .bind((host.as_str(), port))?
    .workers(4)
    .run()
    .await
}
