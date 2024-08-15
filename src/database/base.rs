use postgres::{Client, Error, NoTls};

/*
    Function for connecting with a postgres SQL database.
    {host} - can be an IP or DOMAIN
    {port} - is by default 5432
    {user} - your postgres username
    {password} - your postgres user password
    {database} - the used database.
*/
pub fn connect(host: &str, port: u16, user: &str, password: &str, dbname: &str) -> Result<Client, Error> {
    let connection_str = format!("host={} port={} user={} password={} dbname={}", host, port, user, password, dbname);
    Client::connect(&connection_str, NoTls)
}

pub fn disconnect(client: &mut Result<Client, Error>) -> Result<(), Error> {
    client.close().expect("Failed to disconnect!")
}

pub fn generate_default_tables(client: &mut Result<Client, Error>) -> Result<(), Error> {
    client.batch_execute("
        CREATE TABLE IF NOT EXISTS members(
            id SERIAL PRIMARY KEY,
            name TEXT NOT NULL
        );
        CREATE TABLE IF NOT EXISTS another_table(
            id SERIAL PRIMARY KEY,
            description TEXT
        );
    ").expect("Failed to connect!")
}