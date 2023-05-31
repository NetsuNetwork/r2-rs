mod client;

use std::env;

use aws_sdk_s3::{Client, Region};
use dotenvy::dotenv;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv()?;
    let endpoint = format!(
        "https://{}.r2.cloudflarestorage.com",
        env::var("R2_ACCOUNT_ID")?
    );
    let config = aws_config::from_env()
        .region(Region::new("auto"))
        .endpoint_url(endpoint)
        .load()
        .await;
    let client = Client::new(&config);

    let objects = client::get_objects(client, env::var("R2_BUCKET")?).await?;
    println!("{:?}", objects);

    Ok(())
}
