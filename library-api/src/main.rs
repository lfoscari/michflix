extern crate redis;

use redis::Commands;
use std::env;

fn connect_library(address: String, username: String, password: String) -> redis::RedisResult<redis::Connection> {
    let connection_url = format!("redis://{username}:{password}@{address}");

    let client = redis::Client::open(connection_url)?;
    let connection = client.get_connection()?;

    Ok(connection)
}

fn main() -> redis::RedisResult<()> {
    let db_address  = env::var("REDIS_ENDPOINT").unwrap();
    let db_username = env::var("REDIS_USERNAME").unwrap();
    let db_password = env::var("REDIS_PASSWORD").unwrap();

    let mut library = connect_library(db_address, db_username, db_password)?;

    let _: () = library.set("MICH", "FLIX")?;
    let flix: String = library.get("MICH")?;

    println!("MICH{}", flix);

    Ok(())
}