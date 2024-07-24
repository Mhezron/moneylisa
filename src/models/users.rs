use diesel::prelude::*;
use chrono::NaiveDate;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable,  Serialize, Deserialize)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Users {
    pub id: i32,
    pub fname: String,
    pub sname: String,
    pub lname: String,
    pub email: String,
    pub dob: Option<NaiveDate>,
    pub username: String,
    pub password: String,
    pub created_at: Option<NaiveDateTime>,
}




#[derive(Queryable, Selectable, Debug, Insertable,  Serialize, Deserialize)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewUser {
    pub fname: String,
    pub sname: String,
    pub lname: String,
    pub email: String,
    pub dob: Option<NaiveDate>,
    pub username: String,
    pub password: String,
}



#[derive(Deserialize, Serialize)]
pub struct LoginForm {
    pub  email: String,
    pub  password: String,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct RegisterForm {
    pub fname: String,
    pub sname: String,
    pub lname: String,
    pub username: String,
    pub email: String,
    pub dob: String,
    pub password: String
}