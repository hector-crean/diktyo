use std::path::Path;

use aws_sdk_s3::{
    client::Client, error::SdkError, operation::put_object::PutObjectError, primitives::ByteStream,
};

// use aws_smith_http::error;

use tokio::io::AsyncReadExt;

#[derive(Clone)]
pub struct S3Bucket {
    client: Client,
    region: String,
    bucket_name: String,
}

#[derive(thiserror::Error, Debug)]
pub enum S3Error {
    #[error("File not found")]
    FileNotFound(#[from] std::io::Error),

    #[error("File size conversion error")]
    FileSizeConversion(#[from] std::num::TryFromIntError),

    #[error(transparent)]
    S3Error(#[from] aws_sdk_s3::Error),

    #[error(transparent)]
    PutObjectError(#[from] SdkError<PutObjectError>),

    #[error("Environment variable error")]
    EnvVarError(#[from] std::env::VarError),
}

impl S3Bucket {
    pub fn new(config: aws_sdk_s3::Config, region: &str, bucket_name: &str) -> Self {
        Self {
            client: aws_sdk_s3::Client::from_conf(config),
            region: region.to_string(),
            bucket_name: bucket_name.to_string(),
        }
    }

    pub fn url(&self, key: &str) -> Result<String, S3Error> {
        Ok(format!(
            "https://{}.s3.{}.amazonaws.com/{key}",
            self.bucket_name, self.region,
        ))
    }

    pub async fn upload_object<P: AsRef<Path>>(
        &self,
        file_path: P,
        key: &str,
    ) -> Result<String, S3Error> {
        let mut file = tokio::fs::File::open(file_path).await?;

        let size_estimate: usize = file
            .metadata()
            .await
            .map(|md| md.len())
            .unwrap_or(1024)
            .try_into()?;

        let mut contents = Vec::with_capacity(size_estimate);

        file.read_to_end(&mut contents).await?;

        let _put_object_output = self
            .client
            .put_object()
            .bucket(&self.bucket_name)
            .key(key)
            .body(ByteStream::from(contents))
            .send()
            .await?;

        let url = self.url(key)?;

        Ok(url)
    }

    pub async fn delete_file(&self, key: &str) -> Result<bool, S3Error> {
        Ok(self
            .client
            .delete_object()
            .bucket(&self.bucket_name)
            .key(key)
            .send()
            .await
            .is_ok())
    }
}

#[cfg(test)]
pub mod tests {
    use aws_sdk_s3::config::{Credentials, Region};
    use rand::{distributions::Alphanumeric, Rng};

    use super::*;

    // for `call`
    // for `oneshot` and `ready`

    async fn bucket_singleton() -> Result<S3Bucket, S3Error> {
        use dotenv::dotenv;

        dotenv().ok();

        let aws_key = std::env::var("AWS_ACCESS_KEY_ID")?;
        let aws_key_secret = std::env::var("AWS_SECRET_ACCESS_KEY")?;
        let s3_region = std::env::var("AWS_REGION")?;
        let aws_bucket = std::env::var("S3_BUCKET_NAME")?;

        let aws_config = aws_sdk_s3::config::Builder::new()
            .region(Region::new(s3_region.clone()))
            .credentials_provider(Credentials::new(
                aws_key,
                aws_key_secret,
                None,
                None,
                "loaded-from-custom-env",
            ))
            .build();

        let bucket = S3Bucket::new(aws_config, &s3_region, &aws_bucket);

        Ok(bucket)
    }
}
