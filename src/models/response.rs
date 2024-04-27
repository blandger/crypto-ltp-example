use chrono::{DateTime, Local};
use serde_derive::Serialize;

/// One item in final response from API
#[derive(Serialize, Debug, Default)]
pub struct LtpItem {
    /// Crypto pair name
    pair: String,
    /// Last price value
    amount: String,
}

impl LtpItem {
    pub fn new(pair: String, amount: String) -> Self {
        Self { pair, amount }
    }
}

/// Response list from API
#[derive(Serialize, Debug)]
pub struct LtpListResponse {
    ltp: Vec<LtpItem>,
    /// Optional for testing purpose
    date_time: String,
}

impl LtpListResponse {
    /// Create a new response from the list
    pub fn new(ltp: Vec<LtpItem>) -> Self {
        let local: DateTime<Local> = Local::now();
        // let formatted_local = local.format(DATE_TIME_DEFAULT_FORMAT);
        let formatted_local = local.to_rfc3339_opts(chrono::SecondsFormat::Secs, true);
        Self {
            ltp,
            date_time: formatted_local.to_string(),
        }
    }
    /// Create a new response using capacity value
    pub fn new_with_capacity(capacity: usize) -> Self {
        let local: DateTime<Local> = Local::now();
        let formatted_local = local.to_rfc3339_opts(chrono::SecondsFormat::Secs, true);
        Self {
            ltp: Vec::with_capacity(capacity),
            date_time: formatted_local.to_string(),
        }
    }
    /// Add one more item into a list
    pub fn add_item(&mut self, item: LtpItem) {
        self.ltp.push(item);
    }
}

impl Default for LtpListResponse {
    fn default() -> Self {
        let local: DateTime<Local> = Local::now();
        let formatted_local = local.to_rfc3339_opts(chrono::SecondsFormat::Secs, true);
        Self {
            ltp: vec![],
            date_time: formatted_local.to_string(),
        }
    }
}
