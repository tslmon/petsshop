#[macro_use]
extern crate diesel_migrations;
use actix_cors::Cors;
use actix_web::{App, *};
use petsshop_api::api_routes;
use api::common::{blocking, AuthContext};
use db_queries::get_database_url_from_env;
use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};
use reqwest::Client;
use std::sync::Arc;
// use std::thread;
use tokio::sync::Mutex;
use utils::{
    rate_limit::{rate_limiter::RateLimiter, RateLimit},
    settings::structs::Settings,
    AuthError,
};

embed_migrations!();

#[actix_web::main]
async fn main() -> Result<(), AuthError> {
    env_logger::init();
    let settings = Settings::get();

    // Set up the r2d2 connection pool
    let db_url = match get_database_url_from_env() {
        Ok(url) => url,
        Err(_) => settings.get_database_url(),
    };
    let manager = ConnectionManager::<PgConnection>::new(&db_url);
    let pool = Pool::builder()
        .max_size(settings.database().pool_size)
        .build(manager)
        .unwrap_or_else(|_| panic!("Error connecting to {}", db_url));

    // Run the migrations from code
    blocking(&Some(pool.clone()), move |conn| {
        embedded_migrations::run(conn)?;
        Ok(()) as Result<(), AuthError>
    })
    .await??;

    // let pool2 = pool.clone();
    // thread::spawn(move || {
    //     scheduled_tasks::setup(pool2);
    // });

    // Set up the rate limiter
    let rate_limiter = RateLimit {
        rate_limiter: Arc::new(Mutex::new(RateLimiter::default())),
    };

    println!(
        "Starting http server at {}:{}",
        settings.bind(),
        settings.port()
    );
    // Get session key from settings
    // let session_key = settings.session_key();
    // println!("settings.session_key() = {:?}", session_key);

    // Create Http server
    HttpServer::new(move || {
        let context = AuthContext::create(Some(pool.clone()), Client::default());
        // let context = AuthContext::create(None, Client::default());

        let cors = Cors::default()
            .allow_any_origin()
            //.allowed_origin("http://localhost:3003")
            //.allowed_origin_fn(|origin, _req_head| origin.as_bytes().ends_with(b".localhost:3003"))
            .allow_any_method()
            //.allowed_methods(vec!["GET", "POST"])
            //.allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            //.allowed_header(http::header::CONTENT_TYPE)
            .allow_any_header()
            .max_age(3600);

        let rate_limiter = rate_limiter.clone();

        App::new()
            //.wrap(HiSession)
            //.wrap(CookieSession::signed(&[0; 32]).secure(false))
            .wrap(middleware::Logger::default())
            .wrap(cors)
            .app_data(web::Data::new(context))
            // The routes
            .configure(|cfg| api_routes::config(cfg, &rate_limiter))
    })
    .bind((settings.bind(), settings.port()))?
    .run()
    .await?;

    Ok(())
}
