use actix_web::{web, HttpResponse, Responder};
use crate::models::loans::{Loan, NewLoan};
use crate::models::app_state::AppState;
use crate::db_operations::loans::{add_loan, get_all_loans, get_loan_by_id, update_loan_by_id, delete_loan_by_id};
use log::{error, info};

pub async fn create_loan(form: web::Form<NewLoan>, state: web::Data<AppState>) -> impl Responder {
    let mut connection_guard = state.db_connection.lock().unwrap();
    match add_loan(&mut *connection_guard, form.into_inner()) {
        Ok(_) => HttpResponse::Created().finish(),
        Err(e) => {
            error!("Error creating loan: {:?}", e);
            HttpResponse::InternalServerError().finish()
        },
    }
}

pub async fn view_loans(state: web::Data<AppState>) -> impl Responder {
    let mut connection_guard = state.db_connection.lock().unwrap();
    match get_all_loans(&mut *connection_guard) {
        Ok(loans) => {
            let template = LoanListTemplate { loans };
            HttpResponse::Ok().content_type("text/html").body(template.render().unwrap())
        }
        Err(e) => {
            error!("Error viewing loans: {:?}", e);
            HttpResponse::InternalServerError().finish()
        },
    }
}

pub async fn view_loan(web::Path(loan_id): web::Path<i32>, state: web::Data<AppState>) -> impl Responder {
    let mut connection_guard = state.db_connection.lock().unwrap();
    match get_loan_by_id(&mut *connection_guard, loan_id) {
        Ok(loan) => {
            let template = LoanDetailTemplate { loan };
            HttpResponse::Ok().content_type("text/html").body(template.render().unwrap())
        }
        Err(e) => {
            error!("Error viewing loan: {:?}", e);
            HttpResponse::NotFound().finish()
        },
    }
}

pub async fn update_loan(web::Path(loan_id): web::Path<i32>, form: web::Form<Loan>, state: web::Data<AppState>) -> impl Responder {
    let mut connection_guard = state.db_connection.lock().unwrap();
    match update_loan_by_id(&mut *connection_guard, loan_id, form.into_inner()) {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            error!("Error updating loan: {:?}", e);
            HttpResponse::InternalServerError().finish()
        },
    }
}

pub async fn delete_loan(web::Path(loan_id): web::Path<i32>, state: web::Data<AppState>) -> impl Responder {
    let mut connection_guard = state.db_connection.lock().unwrap();
    match delete_loan_by_id(&mut *connection_guard, loan_id) {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            error!("Error deleting loan: {:?}", e);
            HttpResponse::InternalServerError().finish()
        },
    }
}
