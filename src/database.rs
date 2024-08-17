#![allow(unused)]

use std::fmt::format;
use sqlx::{Connection, Executor, PgPool};
use tokio::fs::read_to_string;
use crate::resources::DATABASE_URL;

/// Test the postgres connection from the resources' module. (DATABASE_URL)
pub async fn connect_test() {
    let test = sqlx::postgres::PgConnection::connect(DATABASE_URL)
        .await
        .expect("[TEST] Connection failed!");

    test.close();
}

pub async fn generate_pool() -> PgPool {
    PgPool::connect(DATABASE_URL).await.unwrap()
}

/// Create a table in the database from an .sql file.
pub async fn create_table(table_path: &str) {
    let connection = PgPool::connect(DATABASE_URL)
        .await
        .unwrap();

    let sql = read_to_string(table_path)
        .await
        .unwrap();

    connection.execute(&*sql)
        .await
        .expect(format!("Problem was detected by creating table [ {} ]", table_path).as_str());

    connection.close().await;
    println!("Create table [ {} ]", table_path);
}

/// Refresh an existing table (first drop then its call create_table() fn!).
pub async fn drop_create_table(dropped_name: &str, table_path: &str) {
    let connection = PgPool::connect(DATABASE_URL)
        .await
        .unwrap();

    connection.execute(format!("drop table {}", dropped_name).as_str())
        .await
        .unwrap();

    connection.close().await;
    create_table(table_path);
}