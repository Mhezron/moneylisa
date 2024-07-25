use crate::models::loans::{Loan, NewLoan};
use diesel::prelude::*;
use diesel::result::Error;

pub fn get_all_loans(connection: &mut PgConnection) -> Vec<Loan> {
    use crate::schema::loans::dsl::*;

    let results = loans
        .limit(5)
        .select(Loan::as_select())
        .load::<Loan>(connection);

    match results {
        Ok(data) => data,
        Err(e) => {
            println!("Error occurred: {:?}", e);
            Vec::new()
        }
    }
}

pub fn get_loan_by_id(connection: &mut PgConnection, loan_id: i32) -> Result<Loan, Error> {
    use crate::schema::loans::dsl::*;

    loans.filter(crate::schema::loans::dsl::loan_id.eq(loan_id))
        .first::<Loan>(connection)
}

pub fn add_loan(new_loan: NewLoan, connection: &mut PgConnection) -> Result<Loan, Error> {
    diesel::insert_into(crate::schema::loans::table)
        .values(&new_loan)
        .get_result::<Loan>(connection)
}

pub fn update_loan_by_id(connection: &mut PgConnection, loan_id: i32, updated_loan: Loan) -> Result<Loan, Error> {
    use crate::schema::loans::dsl::*;

    diesel::update(loans.filter(crate::schema::loans::dsl::loan_id.eq(loan_id)))
        .set(&updated_loan)
        .get_result::<Loan>(connection)
}

pub fn delete_loan_by_id(connection: &mut PgConnection, loan_id: i32) -> Result<usize, Error> {
    use crate::schema::loans::dsl::*;

    diesel::delete(loans.filter(crate::schema::loans::dsl::loan_id.eq(loan_id)))
        .execute(connection)
}
