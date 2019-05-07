#[macro_use]
extern crate failure;
extern crate actix_web;
#[macro_use]
extern crate diesel;
extern crate serde;
#[macro_use]
extern crate serde_derive;
use actix::prelude::*;

use diesel::r2d2::{ConnectionManager};
use dotenv;
use diesel::{
    Connection,
    ConnectionError,
    PgConnection,
};

use actix_web::{server};
use  handlers::db::{DbExecutor};

pub fn connect () -> Result<PgConnection,ConnectionError> {
    PgConnection::establish("")
}

mod handlers;
mod app;
pub mod schema;
pub mod models;
fn main() {
    let sys = actix::System::new("reporting_api");
    //panic if this param is not provided since database connection will fail
    let manager = ConnectionManager::<PgConnection>::new(dotenv::var( "DATABASE_URL").unwrap());
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    let addr: Addr<DbExecutor> = SyncArbiter::start(4, move || DbExecutor(pool.clone()));
    let ip = dotenv::var( "ADDRESS").unwrap();
    let port = dotenv::var( "PORT").unwrap();
    let bind_to = format!("{}:{}", ip, port);
    server::new(move || app::create_app(addr.clone()))
        .bind(&bind_to)
        .expect(&format!("{} {}", "Cannot bind to ",&bind_to))
        .start();
        sys.run();
}
