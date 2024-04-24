use chrono::{DateTime, Local};
use serde_derive::Serialize;
use crate::constants::DATE_TIME_DEFAULT_FORMAT;

#[derive(Serialize, Default)]
pub struct LtpItem {
    pair: String,
    amount: String,
}

#[derive(Serialize)]
pub struct LtpListResponse {
    ltp: Vec<LtpItem>,
    date_time: String
}

impl LtpListResponse {
    pub fn new(ltp: Vec<LtpItem>) -> Self {
        let local: DateTime<Local> = Local::now();
        let formatted_local = local.format(DATE_TIME_DEFAULT_FORMAT);
        Self {
            ltp,
            date_time: formatted_local.to_string(),
        }
    }
}
impl Default for LtpListResponse {
    fn default() -> Self {
        let local: DateTime<Local> = Local::now();
        let formatted_local = local.format(DATE_TIME_DEFAULT_FORMAT);
        Self {
            ltp: vec![],
            date_time: formatted_local.to_string(),
        }
    }
}

