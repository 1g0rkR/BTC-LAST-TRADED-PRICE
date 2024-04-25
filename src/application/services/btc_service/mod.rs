use std::sync::Arc;

use tokio::task::JoinSet;

use crate::{
    domain::entity::pair_ltp::EntityPairLtp,
    infrastructure::{init_kraken_service, kraken_service::KrakenService, Config},
};

use anyhow::{anyhow, Result};
mod dto;

#[derive(Debug, Clone)]
pub struct BtcService {
    btc_platform: Arc<KrakenService>,
    pairs: Vec<String>,
}

impl BtcService {
    #[tracing::instrument]
    pub fn new() -> anyhow::Result<Self> {
        let pairs = Config::init()?.pairs;

        Ok(Self {
            btc_platform: Arc::new(init_kraken_service()?),
            pairs,
        })
    }
    #[tracing::instrument(skip(self))]
    pub async fn get_bitcoin_ltp(&self) -> anyhow::Result<dto::Ltp> {
        let mut res: Vec<EntityPairLtp> = vec![];
        let mut handles = Vec::new();

        let pairs = self.pairs.clone();

        for pair in pairs {
            let btc_platform = Arc::clone(&self.btc_platform);
            let pair_name = pair.clone();
            let handle: tokio::task::JoinHandle<Result<(String, String), anyhow::Error>> =
                tokio::spawn(async move {
                    tracing::info!("Start getting pair {pair} LTP.");
                    let ltp = btc_platform
                        .get_ltp_pair(pair.as_str())
                        .await
                        .map_err(|err| {
                            tracing::error!("Error getting pair {pair} LTP: {err:?}");
                            err
                        })?;
                    Ok((pair_name, ltp))
                });
            handles.push(handle);
        }

        let mut join_set = JoinSet::from_iter(handles);

        while let Some(item) = join_set.join_next().await {
            match item?? {
                Ok((pair_name, pair_amount)) => {
                    tracing::info!("Received {pair_name} LTP: {pair_amount}");
                    res.push(EntityPairLtp::new(pair_name, pair_amount)?)
                }
                Err(err) => return Err(anyhow!(err)),
            }
        }

        Ok(dto::Ltp {
            ltp: res.into_iter().map(|el| el.into()).collect(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_btc_service_init() {
        BtcService::new().expect("Error init BtcService");
    }

    #[tokio::test]
    async fn test_btc_service_get_bitcoin_ltp() {
        let service = BtcService::new().expect("Error init BtcService");

        service
            .get_bitcoin_ltp()
            .await
            .expect("Error getting bitcoin ltp");
    }
}
