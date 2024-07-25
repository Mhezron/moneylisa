use askama::Template;
use crate::models::loans::Loan;

#[derive(Template)]
#[template(path = "login.html")]
pub struct LoginTemplate {
    pub(crate) error: Option<String>,
    pub(crate) message: Option<String>,
}

#[derive(Template)]
#[template(path = "register.html")]
pub struct RegisterTemplate {
    pub(crate) error: Option<String>,
}

#[derive(Template)]
#[template(path = "home.html")]
pub struct HomeTemplate;

#[derive(Template)]
#[template(path = "dashboard.html")]
pub struct DashboardTemplate {
    pub user_name: String,
    pub email: String,
    pub member_since: String,
    pub recent_activity: String,
}

#[derive(Template)]
#[template(path = "loan_list.html")]
pub struct LoanListTemplate {
    pub loans: Vec<Loan>,
}

#[derive(Template)]
#[template(path = "loan_detail.html")]
pub struct LoanDetailTemplate {
    pub loan: Loan,
}
