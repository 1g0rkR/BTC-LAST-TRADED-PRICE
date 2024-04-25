use std::collections::HashMap;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct TickerInformation {
    pub error: Vec<String>,
    pub result: HashMap<String, PairInfo>,
}

#[derive(Debug, Deserialize)]
pub struct PairInfo {
    pub a: Vec<String>,
    pub b: Vec<String>,
    pub c: Vec<String>,
    pub v: Vec<String>,
    pub p: Vec<String>,
    pub t: Vec<i32>,
    pub l: Vec<String>,
    pub h: Vec<String>,
    pub o: String,
}
