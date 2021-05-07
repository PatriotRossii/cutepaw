use std::future::Future;

use futures::TryFutureExt;
use reqwest::{Client, Response};
use serde::{Deserialize, Serialize};

use crate::CutepawError;

pub const API_VERSION: &str = "5.130";
pub const API_TIMEOUT_MS: u64 = 400;

pub struct ApiManager {
    token: String,
    version: String,
    client: Client,
}

impl ApiManager {
    const API_SERVER: &'static str = "https://api.vk.com/method";

    pub fn new<T1, T2>(token: T1, version: T2) -> Self
    where
        T1: Into<String>,
        T2: Into<String>,
    {
        Self {
            token: token.into(),
            version: version.into(),
            client: Client::new(),
        }
    }

    pub fn request<T: Serialize + ?Sized>(
        &self,
        method: &str,
        params: &T,
    ) -> impl Future<Output = Result<Response, reqwest::Error>> {
        let request = self
            .client
            .get(format!("{}/{}", ApiManager::API_SERVER, method));

        let request = request.query(params);
        let request = request.query(&[("access_token", &self.token), ("v", &self.version)]);

        request.send()
    }

    pub async fn request_json<'a, T: Serialize + ?Sized, Y>(
        &self,
        method: &str,
        params: &T,
    ) -> impl Future<Output = Result<Y, CutepawError>>
    where
        Y: for<'de> Deserialize<'de>,
    {
        self.request(method, params)
            .and_then(|x| x.json::<Y>())
            .map_err(CutepawError::ReqwestError)
    }
}
