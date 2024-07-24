use crate::models::loans::{Loan, NewLoan};
use diesel::prelude::*;

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

pub fn add_loans(new_loan: NewLoan, connection: &mut PgConnection) -> Result<bool, String> {
    let res = diesel::insert_into(crate::schema::loans::table)
        .values(&new_loan)
        .get_result::<Loan>(connection);

    match res {
        Ok(_) => Ok(true),
        Err(e) => {
            let f = format!("Error occurred: {:?}", e);
            println!("{}", f);
            Err(f)
        }
    }
}
