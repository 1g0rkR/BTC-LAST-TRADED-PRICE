use serde::Serialize;

use crate::domain::entity::pair_ltp::EntityPairLtp;

#[derive(Serialize)]
pub struct Ltp {
    pub ltp: Vec<PairAmount>,
}

#[derive(Serialize)]
pub struct PairAmount {
    pub pair: String,
    pub amount: String,
}

impl From<EntityPairLtp> for PairAmount {
    fn from(entity: EntityPairLtp) -> Self {
        Self {
            pair: entity.pair.to_string(),
            amount: entity.amount.to_string(),
        }
    }
}
