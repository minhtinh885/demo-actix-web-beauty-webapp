extern crate actix;
extern crate actix_web;
extern crate chrono;
extern crate bcrypt;
extern crate jsonwebtoken;
extern crate r2d2;
extern crate futures;
extern crate env_logger;
extern crate dotenv;
extern crate num_cpus;
extern crate mysql;
extern crate r2d2_mysql;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate failure;
#[macro_use]
extern crate serde_derive;
extern crate rand;
extern crate regex;

use actix::prelude::*;
use actix_web::{server};
use r2d2_mysql::{MysqlConnectionManager};
use db::DbExecutor;

mod db;
mod errors;
mod app;
mod middlewares;
mod api;
mod handler;
mod share;
mod utils;

fn main() {
    dotenv::dotenv().ok();
    ::std::env::set_var("RUST_LOG", "actix_web=info");
    ::std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    let database_url = ::std::env::var("DATABASE_URL").expect("DATABASE_URL phải được cài đặt");

    let sys = System::new("my-pham");
    // Create db connection pool
    let manager = MysqlConnectionManager::new(mysql::OptsBuilder::from_opts(mysql::Opts::from_url(database_url.clone().as_str()).unwrap()));
    let pool = r2d2::Pool::new(manager).expect("Không thể tạo pool");

    let addr: Addr<DbExecutor> = SyncArbiter::start(num_cpus::get(), move || DbExecutor(pool.clone()));

    share::init_user(database_url.clone());


    server::new(move || app::create_app(addr.clone()))
        .bind("127.0.0.1:3001")
        .expect("Không thể liên kết tới: 127.0.0.1:3001")
        .start();

    println!("Đã khởi động server: 127.0.0.1:3001");
    let _ = sys.run();

}
