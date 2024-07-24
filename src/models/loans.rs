use chrono::NaiveDate;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use crate::schema::loans;

#[derive(Queryable, Selectable, Debug, Serialize, Deserialize)]
#[diesel(table_name = loans)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Loan {
    pub loan_id: i32,
    pub user_id: i32,
    pub amount: f64,
    pub created_at: NaiveDateTime,
    pub return_date: Option<NaiveDate>,
    pub loan_type: Option<String>,
    pub status: Option<String>,
    pub interest_rate: Option<f64>,
}



#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = loans)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewLoan {
    pub user_id: i32,
    pub amount: f64,
    pub created_at: NaiveDateTime,
    pub return_date: Option<NaiveDate>,
    pub loan_type: Option<String>,
    pub status: Option<String>,
    pub interest_rate: Option<f64>,
}