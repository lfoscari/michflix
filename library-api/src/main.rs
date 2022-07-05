extern crate redis;
use redis::Commands;

fn main() -> redis::RedisResult<()> {
    let db_address  = env!("REDIS_ENDPOINT");
    let db_username = env!("REDIS_USERNAME");
    let db_password = env!("REDIS_PASSWORD");

    let connection_url = format!("redis://{db_username}:{db_password}@{db_address}");

    let client = redis::Client::open(connection_url)?;
    let mut connection = client.get_connection()?;

    let _: () = connection.set("MICH", "FLIX")?;
    let flix: String = connection.get("MICH")?;

    println!("MICH{}", flix);

    Ok(())
}