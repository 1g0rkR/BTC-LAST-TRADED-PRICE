use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct PairAmount(pub f64);

impl TryFrom<String> for PairAmount {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(Self(value.parse::<f64>()?))
    }
}

impl Display for PairAmount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
