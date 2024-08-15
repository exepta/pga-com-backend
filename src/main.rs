mod api;
mod database;

use api::user_rest::{
    get_user
};

use actix_web::{HttpServer, App, web::Data, middleware::Logger};
use crate::database::base::{connect, disconnect, generate_default_tables};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    let mut client = connect("localhost", 5432, "postgres", "postgres", "postgres");
    generate_default_tables(&mut client);
    disconnect(&mut client);

    HttpServer::new(move || {
        let logger = Logger::default();
        App::new()
            .wrap(logger)
            .service(get_user)
    }).bind(("127.0.0.1", 80))
        ?.run()
        .await


}
