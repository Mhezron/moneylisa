use crate::db_operations::users::{add_user, get_a_user_by_id, get_a_user_by_mail};
use crate::models::app_state::AppState;
use crate::models::ui::{DashboardTemplate, LoginTemplate, RegisterTemplate};
use crate::models::users::{LoginForm, NewUser, RegisterForm, Users};
use actix_session::Session;
use actix_web::web::Redirect;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use askama::Template;
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::NaiveDate;
use chrono::{DateTime, NaiveDateTime, Utc};
use log::{debug, error, info};

async fn handle_register_error(error: &str) -> HttpResponse {
    let template = RegisterTemplate {
        error: Some(error.to_string()),
    };
    HttpResponse::Ok()
        .content_type("text/html")
        .body(template.render().unwrap())
}

pub async fn register_page(error: Option<String>) -> impl Responder {
    let template = RegisterTemplate { error };

    HttpResponse::Ok()
        .content_type("text/html")
        .body(template.render().unwrap())
}

async fn handle_login_information(error: &str) -> HttpResponse {
    let template = LoginTemplate {
        error: None,
        message: Some(error.to_string()),
    };
    HttpResponse::Ok()
        .content_type("text/html")
        .body(template.render().unwrap())
}

pub async fn login_page(error: Option<String>, message: Option<String>) -> impl Responder {
    let template = LoginTemplate { error, message };
    HttpResponse::Ok()
        .content_type("text/html")
        .body(template.render().unwrap())
}

pub async fn dashboard_page(
    state: web::Data<AppState>,
    session: Session,
    req: HttpRequest,
) -> Result<HttpResponse, actix_web::Error> {
    // Retrieve user_id from session
    let user_id = match session.get::<String>("user_id") {
        Ok(Some(user_id)) => user_id,
        Ok(None) => {
            // Redirect to login page if not authenticated
            return Ok(HttpResponse::Found()
                .append_header((actix_web::http::header::LOCATION, "/login"))
                .finish());
        }
        Err(e) => {
            log::error!("Session error: {:?}", e);
            return Err(actix_web::error::ErrorInternalServerError("Session error"));
        }
    };

    // Fetch user data from the database
    let mut connection_guard = state.db_connection.lock().unwrap();
    let user = get_a_user_by_mail(&mut *connection_guard, user_id.clone());

    // Check if user exists
    let user = match user {
        Some(user) => user,
        None => {
            return Ok(HttpResponse::Found()
                .append_header((actix_web::http::header::LOCATION, "/login"))
                .finish());
        }
    };

    // Define dynamic values
    let user_name = format!("{} {}", user.fname, user.lname);
    let email = user.email;

    // Handle Option<NaiveDateTime> properly
    let member_since = match user.created_at {
        Some(date) => date.format("%B %Y").to_string(),
        None => "Date not available".to_string(),
    };

    // Handle recent_activity, assuming it's a String for this example
    let recent_activity = "Logged in 2 hours ago".to_string();

    // Create template with dynamic data
    let dashboard_template = DashboardTemplate {
        user_name,
        email,
        member_since,
        recent_activity,
    };

    // Render the template
    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(dashboard_template.render().map_err(|e| {
            log::error!("Template rendering error: {:?}", e);
            actix_web::error::ErrorInternalServerError("Template error")
        })?))
}

pub async fn login_user(
    form: web::Form<LoginForm>,
    state: web::Data<AppState>,
    session: Session,
) -> Result<HttpResponse, actix_web::Error> {
    let mut connection_guard = state.db_connection.lock().unwrap();

    let user_exist = get_a_user_by_mail(&mut *connection_guard, form.email.clone());
    match user_exist {
        Some(user) => {
            if verify(&form.password, &user.password).unwrap_or(false) {
                session.insert("user_id", form.email.clone())?;
                // Redirect to the dashboard route
                Ok(HttpResponse::Found()
                    .append_header((actix_web::http::header::LOCATION, "/dashboard"))
                    .finish())
            } else {
                let error_message = "Wrong password.".to_string();
                let template = LoginTemplate {
                    error: Some(error_message),
                    message: None,
                };
                Ok(HttpResponse::Ok()
                    .content_type("text/html")
                    .body(template.render().unwrap()))
            }
        }
        None => {
            let error_message = "Email not found".to_string();
            let template = LoginTemplate {
                error: Some(error_message),
                message: None,
            };
            Ok(HttpResponse::Ok()
                .content_type("text/html")
                .body(template.render().unwrap()))
        }
    }
}

pub async fn register_user(
    item: web::Form<RegisterForm>,
    state: web::Data<AppState>,
) -> HttpResponse {
    println!("Data is {:#?}", item);
    if item.fname.is_empty()
        || item.email.is_empty()
        || item.sname.is_empty()
        || item.password.is_empty()
    {
        println!("Empty fields detected");
        return handle_register_error("All fields are required").await;
    }

    println!("All fields have content");
    // Hash and salt the password
    let hashed_password = match hash(&item.password, DEFAULT_COST) {
        Ok(hashed) => hashed,
        Err(er) => {
            println!("error {}", er);
            return handle_register_error("error hashing password").await;
        }
    };

    // Parse the dob string to NaiveDate
    let dob = match NaiveDate::parse_from_str(&item.dob, "%Y-%m-%d") {
        Ok(date) => Some(date),
        Err(_) => {
            println!("Invalid date format");
            return handle_register_error("Invalid date format").await;
        }
    };

    let new_user = NewUser {
        fname: item.fname.clone(),
        sname: item.sname.clone(),
        lname: item.lname.clone(),
        email: item.email.clone(),
        username: item.username.clone(),
        password: hashed_password,
        dob,
    };

    let mut connection_guard = state.db_connection.lock().unwrap();
    let res = add_user(new_user, &mut *connection_guard);

    match res {
        Ok(user) => {
            return handle_login_information("Account created, please login to continue").await;
        }
        Err(err) => {
            println!("db error {:#?}", err);
            return handle_register_error("error creating account").await;
        }
    }
}

async fn update_user(item: web::Json<Users>, state: web::Data<AppState>) -> impl Responder {
    log::info!("to implment");
    HttpResponse::Ok().finish()
}
