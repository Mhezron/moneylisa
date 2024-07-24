use chrono::Utc;
use crate::models::loans::{NewLoan, Loan};
use crate::models::users::{NewUser, Users, LoginForm};
use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use crate::models::app_state::AppState;

pub async fn create_loan(form: web::Form<LoginForm>) -> impl Responder {
    HttpResponse::Ok().finish()
}

pub async fn get_all_loans(state: web::Data<AppState>) -> impl Responder {
    log::info!("Get all loans to implement");
    let v :Vec<Loan> = Vec::new();
    HttpResponse::Ok().json(&v)
}

pub async fn get_loans_item(state: web::Data<AppState>) -> impl Responder {
    log::info!("Fetching all loans to implement");

    let now = Utc::now().naive_utc();
    let a_loan = Loan {
        loan_id: 0,
        user_id: 1,
        amount: 1000.0,
        created_at: now,
        return_date: Some(now.date()),
        loan_type: Some("Personal".to_string()),
        status: Some("Pending".to_string()),
        interest_rate: Some(5.0),
    };
    
    HttpResponse::Ok().json(&a_loan)
}

pub async fn update_loan(item: web::Json<Loan>, state: web::Data<AppState>) -> impl Responder {
    log::info!("to implment");
    HttpResponse::Ok().finish()
}

pub async fn delete_loan_item(path: web::Path<usize>, state: web::Data<AppState>) -> impl Responder {
    // HttpResponse::Ok().finish()
    log::info!("Register user to implment");
    HttpResponse::NotFound().finish()
}
