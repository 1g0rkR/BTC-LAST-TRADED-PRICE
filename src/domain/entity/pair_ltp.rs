use crate::domain::value_objects::{pair_amount::PairAmount, pair_name::PairName};

#[derive(Debug, Clone)]
pub struct EntityPairLtp {
    pub pair: PairName,
    pub amount: PairAmount,
}

impl EntityPairLtp {
    pub fn new(pair_name: String, pair_amount: String) -> anyhow::Result<Self> {
        Ok(Self {
            pair: pair_name.into(),
            amount: pair_amount.try_into()?,
        })
    }
}
