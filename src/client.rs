use std::{env, error::Error};

use aws_sdk_s3::{model::Object, Client, Region};

use super::common::select_by_random;

pub struct R2Client {
    bucket: String,
    cached_keys: Vec<String>,
    client: Client,
}

impl R2Client {
    pub async fn new(bucket: String) -> Result<Self, Box<dyn Error>> {
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

        Ok(Self {
            bucket,
            client,
            cached_keys: Vec::new(),
        })
    }

    /// Get objects from the R2 Bucket
    ///
    /// This function also populates the cache which other functions will require
    pub async fn get_objects(&mut self) -> Result<Vec<String>, Box<dyn Error>> {
        let req = self
            .client
            .list_objects()
            .bucket(self.bucket.clone())
            .send()
            .await?;
        if let Some(content) = req.contents() {
            let names = content
                .iter()
                .map(|f| {
                    // We expect each file to contain a name
                    f.key().unwrap().to_string()
                })
                .collect::<Vec<String>>();

            self.cached_keys = names.clone();

            return Ok(names);
        }

        Err("NO_CONTENTS".into())
    }

    /// Grab a random name from the cache
    pub async fn grab_object(&self) -> Result<Option<String>, Box<dyn Error>> {
        if self.cached_keys.is_empty() {
            return Ok(None);
        }

        Ok(Some(select_by_random(&self.cached_keys)))
    }

    // ! Perhaps shorten this to wrap around `get_objects`
    /// Filter through objects
    /// 
    /// Similar to `get_objects`
    pub async fn filter_objects<F>(&self, filter: F) -> Result<Vec<String>, Box<dyn Error>>
    where
        F: FnMut(&&Object) -> bool,
    {
        let req = self
            .client
            .list_objects()
            .bucket(self.bucket.clone())
            .send()
            .await?;
        if let Some(content) = req.contents() {
            let names = content
                .iter()
                .filter(filter)
                .map(|f| {
                    // We expect each file to contain a name
                    f.key().unwrap().to_string()
                })
                .collect::<Vec<_>>();

            return Ok(names);
        }

        Err("NO_CONTENTS".into())
    }
}
