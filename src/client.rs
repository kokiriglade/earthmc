//! # Client
//!
//! The main client module for the EarthMC API.
//!
//! Contains the [`Client`] struct and its methods.

use parking_lot::Mutex;
use reqwest::{Client as ReqwestClient, Url};
use serde::{Serialize, de::DeserializeOwned};
use std::{
    fmt::Debug,
    sync::{Arc, LazyLock},
    time::Duration,
};

use derive_builder::Builder;

use crate::{
    errors::{Error, snippet_around},
    named_id::NamedId,
    nation::Nation,
    player::Player,
    query::{Query, SimpleQuery},
    retry_strategy::{JitteredBackoff, RetryStrategy},
    town::Town,
    world::World,
};

pub const DEFAULT_BASE_URL: &str = "https://api.earthmc.net/v3/";

static DEFAULT_HTTP_CLIENT: LazyLock<ReqwestClient> = LazyLock::new(|| {
    reqwest::ClientBuilder::new()
        .connect_timeout(Duration::from_secs(10))
        .timeout(Duration::from_secs(10))
        .user_agent(format!(
            "https://github.com/kokiriglade/earthmc {}",
            env!("CARGO_PKG_VERSION")
        ))
        .build()
        .expect("Failed to initialize HTTP client")
});

#[derive(Builder, Clone)]
pub struct Client {
    #[builder(default = DEFAULT_HTTP_CLIENT.clone())]
    reqwest_client: ReqwestClient,
    #[builder(default = DEFAULT_BASE_URL.parse().unwrap())]
    base_url: Url,
    #[builder(default = Arc::new(Mutex::new(JitteredBackoff::default())))]
    retry_strategy: Arc<Mutex<dyn RetryStrategy>>,
    #[builder(default = World::Aurora)]
    world: World,
}

impl Debug for Client {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Client")
            .field("reqwest_client", &self.reqwest_client)
            .field("base_url", &self.base_url)
            .finish()
    }
}

impl Client {
    /// Perform a GET request and deserialize into `T`.
    async fn get<T>(&self, path: &str) -> Result<T, Error>
    where
        T: DeserializeOwned,
    {
        // build the full URL
        let combined = format!("{}/{}", self.world.as_string(), path);
        let url = self
            .base_url
            .join(&combined)
            .expect("Failed to construct request URL");

        let mut num_retries = 0;
        loop {
            let attempt = self.reqwest_client.get(url.clone()).send().await;

            match attempt {
                Ok(response) => {
                    match response.error_for_status() {
                        Ok(success) => {
                            // deserialize into T
                            let parsed = success.json::<T>().await?;
                            return Ok(parsed);
                        }
                        Err(e) => {
                            let err: Error = e.into();
                            let mut strat = self.retry_strategy.lock();

                            if let Some(delay) =
                                strat.should_retry_after(num_retries)
                            {
                                tokio::time::sleep(delay).await;
                                num_retries += 1;
                                continue;
                            } else {
                                return Err(err);
                            }
                        }
                    }
                }
                Err(e) => {
                    let err: Error = e.into();
                    let mut strat = self.retry_strategy.lock();

                    if let Some(delay) = strat.should_retry_after(num_retries) {
                        tokio::time::sleep(delay).await;
                        num_retries += 1;
                        continue;
                    } else {
                        return Err(err);
                    }
                }
            }
        }
    }

    /// Perform a POST request with JSON body `B` and deserializes the response into
    /// `T`.
    pub async fn post<T, B>(&self, path: &str, body: B) -> Result<T, Error>
    where
        T: DeserializeOwned,
        B: Serialize + Sized,
    {
        // build full URL
        let combined = format!("{}/{}", self.world.as_string(), path);
        let url = self
            .base_url
            .join(&combined)
            .expect("Failed to construct request URL");

        let mut num_retries = 0;
        loop {
            let attempt = self
                .reqwest_client
                .post(url.clone())
                .json(&body)
                .send()
                .await;

            match attempt {
                Ok(response) => match response.error_for_status() {
                    Ok(ok_response) => {
                        let text = ok_response.text().await?;
                        match serde_json::from_str::<T>(&text) {
                            Ok(parsed) => return Ok(parsed),
                            Err(de_err) => {
                                let snippet = snippet_around(
                                    &text,
                                    de_err.line(),
                                    de_err.column(),
                                    100,
                                );
                                return Err(
                                    Error::DeserializationWithSnippet {
                                        source: de_err,
                                        snippet,
                                    },
                                );
                            }
                        }
                    }
                    Err(e) => {
                        let err: Error = e.into();
                        let mut strat = self.retry_strategy.lock();
                        if let Some(delay) =
                            strat.should_retry_after(num_retries)
                        {
                            tokio::time::sleep(delay).await;
                            num_retries += 1;
                            continue;
                        }
                        return Err(err);
                    }
                },
                Err(e) => {
                    let err: Error = e.into();
                    let mut strat = self.retry_strategy.lock();
                    if let Some(delay) = strat.should_retry_after(num_retries) {
                        tokio::time::sleep(delay).await;
                        num_retries += 1;
                        continue;
                    }
                    return Err(err);
                }
            }
        }
    }

    const TOWNS_PATH: &str = "towns";
    const NATIONS_PATH: &str = "nations";
    const PLAYERS_PATH: &str = "players";

    /// Fetches all currently registered Towny towns.
    pub async fn get_all_towns(&self) -> Result<Vec<NamedId>, Error> {
        self.get::<Vec<NamedId>>(Self::TOWNS_PATH).await
    }

    /// Queries detailed information on specific towns.
    pub async fn query_towns(
        &self,
        query: SimpleQuery,
    ) -> Result<Vec<Town>, Error> {
        self.post::<Vec<Town>, Query<SimpleQuery>>(
            Self::TOWNS_PATH,
            Query::from(query),
        )
        .await
    }

    /// Fetches all currently registered Towny nations.
    pub async fn get_all_nations(&self) -> Result<Vec<NamedId>, Error> {
        self.get::<Vec<NamedId>>(Self::NATIONS_PATH).await
    }

    /// Queries detailed information on specific nations.
    pub async fn query_nations(
        &self,
        query: SimpleQuery,
    ) -> Result<Vec<Nation>, Error> {
        self.post::<Vec<Nation>, Query<SimpleQuery>>(
            Self::NATIONS_PATH,
            Query::from(query),
        )
        .await
    }

    /// Fetches all currently registered Towny residents.
    pub async fn get_all_players(&self) -> Result<Vec<NamedId>, Error> {
        self.get::<Vec<NamedId>>(Self::PLAYERS_PATH).await
    }

    /// Queries detailed information on specific players.
    pub async fn query_players(
        &self,
        query: SimpleQuery,
    ) -> Result<Vec<Player>, Error> {
        self.post::<Vec<Player>, Query<SimpleQuery>>(
            Self::PLAYERS_PATH,
            Query::from(query),
        )
        .await
    }
}
