mod controllers;
mod db_operations;
mod models;
mod schema;

use crate::controllers::loans::{create_loan, delete_loan, update_loan, view_loan, view_loans};
use crate::controllers::users::{
    dashboard_page, login_page, login_user, register_page, register_user,
};
use crate::models::app_state::AppState;
use crate::models::users::{LoginForm, Users};
use actix_files as fs;
use actix_session::{storage::CookieSessionStore, SessionMiddleware};
use chrono::Utc;
use db_operations::db;
use diesel::prelude::*;
use dotenvy::dotenv;
use log::info;

use actix_session::config::PersistentSession;
use actix_web::{
    cookie::{time::Duration, Key},
    middleware, web, App, HttpResponse, HttpServer, Responder,
};
use askama::Template;
use std::sync::Mutex;

use crate::controllers::dashboard::{protected, unprotected};
use crate::controllers::home::default_handler;
use crate::models::ui::LoginTemplate;
use actix_web::cookie::SameSite;
use actix_web::{dev::ServiceRequest, dev::ServiceResponse, Error, HttpMessage};

const ONE_MINUTE: Duration = Duration::minutes(1);

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // load env variables
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    info!("starting HTTP server at http://localhost:8080");

    let secret_key = Key::generate();

    HttpServer::new(move || {
        // Initialize application state
        let app_state = web::Data::new(AppState {
            db_connection: Mutex::new(db::establish_connection()),
        });
        // todo improve above to use a  pool not a single connection

        App::new()
            .app_data(app_state.clone())
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), secret_key.clone())
                    .cookie_secure(false) // set to true if using HTTPS
                    .cookie_http_only(true)
                    .cookie_same_site(SameSite::Lax)
                    .build(),
            )
            .service(fs::Files::new("/static", "./static").show_files_listing())
            .route("/dashboard", web::get().to(dashboard_page))
            .route("/login", web::get().to(login_page))
            .route("/login", web::post().to(login_user))
            .route("/register", web::get().to(register_page))
            .route("/register", web::post().to(register_user))
            .route("/create-loan", web::post().to(create_loan))
            .route("/create-loan", web::post().to(create_loan))
            .route("/view-loans", web::get().to(view_loans))
            .route("/view-loan/{loan_id}", web::get().to(view_loan))
            .route("/update-loan/{loan_id}", web::post().to(update_loan)) 
            .route("/delete-loan/{loan_id}", web::delete().to(delete_loan))
            .service(web::resource("/protected").route(web::get().to(protected)))
            .service(web::resource("/unprotected").route(web::get().to(unprotected)))
            .default_service(web::to(default_handler))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
