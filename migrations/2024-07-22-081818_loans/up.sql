-- Your SQL goes here
CREATE TABLE loans (
    loan_id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL,
    amount FLOAT8 NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    return_date DATE,
    loan_type VARCHAR(50),
    status VARCHAR(50) DEFAULT 'pending',
    interest_rate FLOAT8,
    FOREIGN KEY (user_id) REFERENCES users(id)
);
