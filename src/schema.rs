// @generated automatically by Diesel CLI.

diesel::table! {
    loans (loan_id) {
        loan_id -> Int4,
        user_id -> Int4,
        amount -> Float8,
        created_at -> Timestamp,
        return_date -> Nullable<Date>,
        #[max_length = 50]
        loan_type -> Nullable<Varchar>,
        #[max_length = 50]
        status -> Nullable<Varchar>,
        interest_rate -> Nullable<Float8>,
    }
}

diesel::table! {
    loans_type (type_id) {
        type_id -> Int4,
        #[max_length = 50]
        name -> Nullable<Varchar>,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 100]
        fname -> Varchar,
        #[max_length = 100]
        sname -> Varchar,
        #[max_length = 100]
        lname -> Varchar,
        #[max_length = 100]
        email -> Varchar,
        dob -> Nullable<Date>,
        #[max_length = 100]
        username -> Varchar,
        #[max_length = 100]
        password -> Varchar,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::joinable!(loans -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    loans,
    loans_type,
    users,
);
