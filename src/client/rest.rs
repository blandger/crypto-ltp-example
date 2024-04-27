use std::sync::Arc;

use log::debug;
use serde::Deserialize;

use crate::constants::{GET_TICKER_PATH, KRAKEN_REST_API_HOST};
use crate::errors::{ClientResult, Error};
use crate::models::kraken_trade::{KrakenBaseError, KrakenTrade};

/// Component for making rest api calls and convert kraken response into struct
#[derive(Debug)]
pub struct RestClient;

impl Default for RestClient {
    fn default() -> Self {
        Self::new()
    }
}

impl RestClient {
    /// create new client for accessing remote rest api
    pub fn new() -> Self {
        Self {}
    }

    /// Make a get call to remote rest api.
    /// The target type the answer should be parsed into should be specified
    pub async fn get<T: KrakenBaseError + for<'a> Deserialize<'a>>(
        &self,
        url: &str,
    ) -> ClientResult<T> {
        debug!("connecting.... '{}'", url);
        let body = reqwest::get(url)
            .await
            .map_err(|err| Error::Connect(err.to_string()))?
            .json::<T>()
            .await
            .map_err(|err| Error::FetchFailed(err.to_string()))?;
        if !body.error().is_empty() {
            let error = format!("{}: {:?}", "Response with error:", &body.error());
            return Err(Error::IncorrectResponse(error));
        }
        Ok(body)
    }
}

/// Get a response from the remote host and deserialize it into target struct.
pub async fn fetch_pair_trade_response(
    pair_name: &str,
    client: &Arc<RestClient>,
) -> ClientResult<KrakenTrade> {
    debug!("Fetching a Pair = {:?}", pair_name);
    let url = format!(
        "{}{}?pair={}",
        KRAKEN_REST_API_HOST,
        GET_TICKER_PATH,
        pair_name.replace('/', "")
    );
    let response = client.get::<KrakenTrade>(&url).await?;
    debug!("Success, fetched data for Pair = {:?}", pair_name);
    Ok(response)
}

#[cfg(test)]
mod tests {
    use crate::constants::{GET_TICKER_PATH, KRAKEN_REST_API_HOST};

    use super::*;

    #[actix_rt::test]
    async fn test_invalid_host() {
        let url = format!(
            "{}{}?pair={}",
            "KRAKEN_REST_API_HOST",
            GET_TICKER_PATH,
            "BTC/USD".replace("/", "")
        );
        // println!("{}", &url);
        let client = RestClient::new();
        let result = client.get::<KrakenTrade>(&url).await;
        // println!("{:?}", result);
        assert!(result.is_err());
    }

    #[actix_rt::test]
    async fn test_invalid_path() {
        let url = format!(
            "{}{}?pair={}",
            KRAKEN_REST_API_HOST,
            "GET_TICKER_PATH",
            "BTC/USD".replace("/", "")
        );
        // println!("{}", &url);
        let client = RestClient::new();
        let result = client.get::<KrakenTrade>(&url).await;
        // println!("{:?}", result);
        assert!(result.is_err());
    }

    #[actix_rt::test]
    async fn test_invalid_param() {
        let url = format!(
            "{}{}?pair={}",
            KRAKEN_REST_API_HOST,
            GET_TICKER_PATH,
            "QQ/USD".replace("/", "")
        );
        // println!("{}", &url);
        let client = RestClient::new();
        let result = client.get::<KrakenTrade>(&url).await;
        // println!("{:?}", result);
        assert!(result.is_err());
    }

    #[actix_rt::test]
    async fn test_request_usd() {
        let url = format!(
            "{}{}?pair={}",
            KRAKEN_REST_API_HOST,
            GET_TICKER_PATH,
            "BTC/USD".replace("/", "")
        );
        // println!("{}", &url);
        let client = RestClient::new();
        let result = client.get::<KrakenTrade>(&url).await;
        // println!("{:?}", result);
        assert!(result.is_ok());
    }

    #[actix_rt::test]
    async fn test_request_chf() {
        let url = format!(
            "{}{}?pair={}",
            KRAKEN_REST_API_HOST,
            GET_TICKER_PATH,
            "BTC/CHF".replace("/", "")
        );
        // println!("{}", &url);
        let client = RestClient::new();
        let result = client.get::<KrakenTrade>(&url).await;
        // println!("{:?}", result);
        assert!(result.is_ok());
    }
}
