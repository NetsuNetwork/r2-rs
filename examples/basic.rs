use std::env;
use std::error::Error;

use dotenvy::dotenv;
use r2_rs::client::R2Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv()?;
    let mut client = R2Client::new(env::var("R2_BUCKET")?).await?;
    let objects = client.get_objects().await?;
    let one = client.grab_object().await?;
    let filtered = client.filter_objects(|o| {
      o.key().unwrap().starts_with("Fq")
    }).await?;

    println!("{:?}", objects);
    println!("{:?}", one);
    println!("{:?}", filtered);

    Ok(())
}
