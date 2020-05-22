extern crate failure;

async fn test_redis() -> Result<(), failure::Error> {
    println!("running test_redis:");

    use redis::AsyncCommands;
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let mut con = client.get_async_connection().await?;
    con.rpush("theKey", 1i32).await?;
    con.rpush("theKey", "hello").await?;
    println!("{:?}", con.lpop::<_, i32>("theKey").await?);
    println!("{:?}", con.lpop::<_, String>("theKey").await?);

    Ok(())
}

async fn test_mongo() -> Result<(), failure::Error> {
    println!("running test_mongo:");

    use mongodb::{Client, options::ClientOptions};
    let mut client_options = ClientOptions::parse("mongodb://localhost:27017").await?;
    client_options.app_name = Some("My App".to_string());
    let client = Client::with_options(client_options)?;

    for db_name in client.list_database_names(None, None).await? {
        println!("db_name: {}", db_name);
    }

    Ok(())
}

async fn process() -> Result<(), failure::Error> {
    test_redis().await?;
    test_mongo().await?;

    println!("DONE");

    Ok(())
}

fn main() -> Result<(), failure::Error> {
    let mut rt = tokio::runtime::Runtime::new()?;
    tokio::task::LocalSet::new().block_on(&mut rt, process())?;

    Ok(())
}
