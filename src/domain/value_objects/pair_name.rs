use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct PairName(pub String);

impl From<String> for PairName {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl Display for PairName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
