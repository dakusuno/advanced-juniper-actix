#[macro_use]
extern crate diesel;
pub mod graphql;
pub mod schema;
pub mod db;
pub mod handlers;
use std::{env, io};
use crate::db::get_pool;
use actix_web::{HttpServer,App,middleware,web};
pub use crate::handlers::register;
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    logging_setup();
    dotenv::dotenv().ok();
    std::env::set_var("RUST_LOG","actix_web=info,info");
    let pool = get_pool();
    HttpServer::new(move||
        App::new()
            .data(pool.clone())
            .wrap(middleware::Logger::default())
            .configure(register)
            .default_service(web::to(||async{"404"}))
    )
        .bind("127.0.0.1:8999")?
        .run()
        .await
}
fn logging_setup() {
    env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
}