use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use strum_macros::{AsRefStr, EnumString};
use thiserror::Error;

pub mod logging;
pub mod mock;

const RECAPTCHA_VERIFY_URL: &str = "https://google.com/recaptcha/api/siteverify";

#[async_trait]
pub trait Service {
    async fn verify(&self, token: String, agent_ip_address: Option<String>) -> Result<(), Error>;
}

struct ServiceImpl {
    secret_key: String,
}

pub fn new_service(secret_key: String) -> impl Service {
    ServiceImpl { secret_key }
}

#[async_trait]
impl Service for ServiceImpl {
    async fn verify(&self, token: String, agent_ip_address: Option<String>) -> Result<(), Error> {
        let mut body = format!("secret={}&response={}", self.secret_key, token);
        if let Some(remote_ip) = agent_ip_address {
            body.push_str(format!("&remoteip={}", remote_ip).as_str())
        };
        let client = reqwest::Client::new();
        let response = match client
            .post(RECAPTCHA_VERIFY_URL)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .header("Accept", "*/*")
            .body(body)
            .send()
            .await
        {
            Ok(resp) => resp,
            Err(e) => return Err(Error::Unknown { cause: e.to_string() }),
        };
        let response_body = response.text().await.unwrap();
        // println!("{}", response_body.as_str());
        // {
        //   "success": true,
        //   "challenge_ts": "2020-10-04T18:11:50Z",
        //   "hostname": "testkey.google.com"
        // }
        let body: VerifyResponse = match serde_json::from_str(response_body.as_str()) {
            Ok(body) => body,
            Err(e) => return Err(Error::Unknown { cause: e.to_string() }),
        };
        if !body.success {
            return Err(Error::VerificationFailed {
                cause: "recaptcha token verification failed".to_string(),
            });
        }
        Ok(())
    }
}

#[derive(Error, Clone, Debug, EnumString, AsRefStr)]
pub enum Error {
    #[error("verification of recaptcha token failed: {cause}")]
    VerificationFailed { cause: String },

    #[error("an unknown error occurred: {cause}")]
    Unknown { cause: String },
}

/// See the definition of this response here:
/// https://developers.google.com/recaptcha/docs/verify
#[derive(Serialize, Deserialize)]
struct VerifyResponse {
    success: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Fixture {
        service: Box<dyn Service>,
    }

    impl Fixture {
        const PUBLIC_TEST_SECRET_KEY: &'static str = "6LeIxAcTAAAAAGG-vFI1TnRWxMZNFuojJ4WifJWe";

        pub fn new() -> Self {
            Self {
                service: Box::new(new_service(Fixture::PUBLIC_TEST_SECRET_KEY.to_string())),
            }
        }
    }

    #[tokio::test]
    async fn should_verify_with_recaptcha_test_keys() {
        let f = Fixture::new();
        if let Err(e) = f.service.verify("".to_string(), None).await {
            panic!("failed to verify using recaptcha test token: {}", e)
        }
    }
}

