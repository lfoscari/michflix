use mongodb::{bson::doc, options::ClientOptions, Client};
use std::env;

async fn connect_library(address: &str, username: &str, password: &str, database: &str) -> mongodb::error::Result<mongodb::Database> {
    let uri = format!("mongodb+srv://${username}:${password}@{address}");
    let client_options = ClientOptions::parse(uri).await?;
    
    let client = Client::with_options(client_options)?;
    return Ok(client.database(database));
}

#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
    let db_address  = env::var("MONGODB_ENDPOINT").unwrap();
    let db_username = env::var("MONGODB_USERNAME").unwrap();
    let db_password = env::var("MONGODB_PASSWORD").unwrap();
    let db_database = env::var("MONGODB_DATABASE").unwrap();

    let library = connect_library(&db_address, &db_username, &db_password, &db_database).await?;

    println!("{:?}", library);

    Ok(())
}