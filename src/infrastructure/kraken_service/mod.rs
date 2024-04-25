use anyhow::anyhow;

use crate::domain::models::{PairInfo, TickerInformation};

pub const KRAKEN_SERVICE_URL: &str = "https://api.kraken.com/0/public/Ticker";

#[derive(Debug, Clone)]
pub struct KrakenService {
    client: reqwest::Client,
    url: reqwest::Url,
}

impl KrakenService {
    pub fn new(url: &str) -> anyhow::Result<Self> {
        Ok(Self {
            client: reqwest::Client::builder().build()?,
            url: reqwest::Url::parse(url)?,
        })
    }

    pub async fn get_ltp_pair(&self, pair: &str) -> anyhow::Result<String> {
        let res = self
            .client
            .get(self.url.clone())
            .query(&[("pair", pair)])
            .send()
            .await?
            .json::<TickerInformation>()
            .await?;

        let pairs_info = res.result.into_values().collect::<Vec<PairInfo>>();

        let pair_info = pairs_info
            .first()
            .ok_or(anyhow!("Error getting pair info {pair}"))?;

        let ltp = pair_info
            .c
            .get(0)
            .ok_or(anyhow!("Error getting ltp pair {pair}"))?;

        Ok(ltp.to_owned())
    }
}

pub fn init_kraken_service() -> anyhow::Result<KrakenService> {
    KrakenService::new(KRAKEN_SERVICE_URL)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_init_kraken_service() {
        init_kraken_service().expect("Error initialization Kraken service");
    }

    #[tokio::test]
    async fn test_kraken_service_ltp_pair_btcusd() {
        let service = init_kraken_service().unwrap();
        service
            .get_ltp_pair("BTCUSD")
            .await
            .expect("Error getting BTC/USD");
    }

    #[tokio::test]
    async fn test_kraken_service_ltp_pair_btceur() {
        let service = init_kraken_service().unwrap();
        service
            .get_ltp_pair("BTCEUR")
            .await
            .expect("Error getting BTC/EUR");
    }

    #[tokio::test]
    async fn test_kraken_service_ltp_pair_btcchf() {
        let service = init_kraken_service().unwrap();
        service
            .get_ltp_pair("BTCCHF")
            .await
            .expect("Error getting BTC/CHF");
    }
}
