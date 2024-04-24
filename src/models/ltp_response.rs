use serde_derive::Serialize;

#[derive(Serialize, Default)]
pub struct LtpItem {
    pair: String,
    amount: String,
}

#[derive(Serialize, Default)]
pub struct LtpListResponse {
    ltp: Vec<LtpItem>,
}
