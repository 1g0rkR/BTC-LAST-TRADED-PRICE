const BTC_PAIRS_ENV_VAR: &str = "BTC_PAIRS";

pub struct Config {
    pub pairs: Vec<String>,
}

impl Config {
    pub fn init() -> anyhow::Result<Self> {
        let env_pairs = std::env::var(BTC_PAIRS_ENV_VAR)?;

        Ok(Self {
            pairs: env_pairs
                .trim()
                .split(",")
                .map(|el| el.to_owned())
                .collect(),
        })
    }
}
