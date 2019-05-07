use actix::prelude::*;
use actix_web::{App, http::Method};
use super::handlers::db::{DbExecutor};
use super::handlers::{import_records, reports};

pub struct AppState {
    pub db: Addr<DbExecutor>,
}

// creates and returns the app after mounting all routes/resources
pub fn create_app(db: Addr<DbExecutor>) -> App<AppState> {
    App::with_state(AppState { db })
        .resource("/import", |r|  r.method(Method::POST).f(import_records))
        .resource("/reporting", |r|  r.method(Method::GET).f(reports))
}
