use std::env;
use dotenv::dotenv;
use lazy_static::lazy_static;

lazy_static!{
    pub static ref DATABASE_URL: String = set_database_url();
    pub static ref JWT_TOKKEN: String = set_jwt_token();
    pub static ref JWT_LIFE_SPAN: i64 = set_jwt_life_span();
}

fn set_database_url() -> String {
    dotenv().ok();
    env::var("DATABASE_URL").unwrap()
}

fn set_jwt_token() -> String {
    dotenv().ok();
    env::var("JWT_TOKEN").unwrap()
}

fn set_jwt_life_span() -> i64 {
    dotenv().ok();
    env::var("JWT_LIFE_SPAN").unwrap().parse::<i64>().unwrap()
}