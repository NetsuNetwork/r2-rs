use std::error::Error;

use aws_sdk_s3::Client;

pub async fn get_objects(
    client: Client,
    bucket_name: String,
) -> Result<Vec<String>, Box<dyn Error>> {
    let req = client
        .list_objects()
        .bucket(bucket_name)
        .send()
        .await?;
    if let Some(content) = req.contents() {
        let names = content
            .iter()
            .map(|f| {
              // We expect each file to contain a name
              f.key().unwrap().to_string()
            })
            .collect();

        return Ok(names);
    }

    Err("NO_CONTENTS".into())
}
