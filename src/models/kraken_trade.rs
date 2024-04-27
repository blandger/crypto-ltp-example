use rust_decimal::Decimal;
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;
use std::str::FromStr;

/// Trait for getting common error from all kraken responses
pub trait KrakenBaseError {
    /// Get error message from json if exists
    fn error(&self) -> Vec<String>;
}
/// Trade structure for one crypto pair
#[derive(Debug, Deserialize)]
pub struct KrakenTrade {
    pub error: Vec<String>,
    pub result: Option<HashMap<String, TradeResultTypes>>,
}
impl KrakenBaseError for KrakenTrade {
    fn error(&self) -> Vec<String> {
        self.error.clone()
    }
}

impl KrakenTrade {
    /// Return last trade price from the Trade object
    pub fn get_last_price(&self) -> Option<String> {
        self.result.as_ref()?;

        match self.result.as_ref().unwrap().values().next() {
            Some(trade_type) => match trade_type {
                TradeResultTypes::TradeData(trade_data) => {
                    if trade_data.last_trade_close_price.is_empty()
                        || trade_data.last_trade_close_price.len() <= 1
                    {
                        return None;
                    }
                    let price = trade_data
                        .last_trade_close_price
                        .first()
                        .map(|s| s.to_string())
                        .unwrap_or("".to_string());
                    if price.is_empty() {
                        return None;
                    }
                    let price_decimal = Decimal::from_str(price.as_str())
                        .unwrap_or_default()
                        .round_dp(2);
                    Some(price_decimal.to_string())
                }
                TradeResultTypes::String(_) => None,
            },
            None => None,
        }
    }
}
/// Type alias for using by serde deserialization
type TradeData = KrakenTradeItem;

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum TradeResultTypes {
    String(String),
    TradeData(TradeData),
}
/// One crypto pair trade item with fields
#[derive(Debug, Deserialize, Serialize)]
pub struct KrakenTradeItem {
    #[serde(rename = "a")]
    pub ask_price: Vec<String>,
    #[serde(rename = "b")]
    pub bid_price: Vec<String>,
    #[serde(rename = "c")]
    pub last_trade_close_price: Vec<String>,
    #[serde(rename = "v")]
    pub today_trade_volume: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal::Decimal;

    #[test]
    fn test_deserialize_usd() {
        let source = r#"{"error":[],"result":{"XXBTZUSD":{"a":["64612.10000","2","2.000"],"b":["64612.00000","9","9.000"],"c":["64612.10000","0.00323448"],"v":["1971.59678625","2313.55393735"],"p":["63935.29841","63952.35837"],"t":[22298,28161],"l":["62743.50000","62743.50000"],"h":["64915.90000","64915.90000"],"o":"64291.30000"}}}"#;
        let result = serde_json::from_str::<KrakenTrade>(source);
        // println!("{:?}", result);
        assert!(result.is_ok());
    }

    #[test]
    fn test_des_chf() {
        let source = r#"{"error":[],"result":{"XBTCHF":{"a":["59038.50000","1","1.000"],"b":["59005.80000","1","1.000"],"c":["59042.10000","0.00010000"],"v":["18.59019190","22.18713969"],"p":["58359.33086","58399.49802"],"t":[1420,1641],"l":["57500.00000","57500.00000"],"h":["59184.80000","59184.80000"],"o":"58778.00000"}}}"#;
        let result = serde_json::from_str::<KrakenTrade>(source);
        // println!("{:?}", result);
        assert!(result.is_ok());
    }

    #[test]
    fn test_get_last_price() {
        let source = r#"{"error":[],
            "result":{"XBTCHF":{"a":["59038.50000","1","1.000"],"b":["59005.80000","1","1.000"],
            "c":["59042.10000","0.00010000"],"v":["18.59019190","22.18713969"],
            "p":["58359.33086","58399.49802"],"t":[1420,1641],"l":["57500.00000","57500.00000"],
            "h":["59184.80000","59184.80000"],"o":"58778.00000"}}}"#;
        let result = serde_json::from_str::<KrakenTrade>(source);
        // println!("{:?}", result);
        assert!(result.is_ok());
        let price = result.unwrap().get_last_price();
        // println!("price = {:?}", price);
        assert!(price.is_some());
        assert_eq!(price.unwrap(), String::from("59042.10"));
    }

    #[test]
    fn test_get_last_price_empty_value() {
        let source = r#"{"error":[],
            "result":{"XBTCHF":{"a":["59038.50000","1","1.000"],"b":["59005.80000","1","1.000"],
            "c":["","0.00010000"],"v":["18.59019190","22.18713969"],
            "p":["58359.33086","58399.49802"],"t":[1420,1641],"l":["57500.00000","57500.00000"],
            "h":["59184.80000","59184.80000"],"o":"58778.00000"}}}"#;
        let result = serde_json::from_str::<KrakenTrade>(source);
        // println!("{:?}", result);
        assert!(result.is_ok());
        let price = result.unwrap().get_last_price();
        // println!("price = {:?}", price);
        assert!(price.is_none());
    }

    #[test]
    fn test_get_last_price_missing_value() {
        let source = r#"{"error":[],
            "result":{"XBTCHF":{"a":["59038.50000","1","1.000"],"b":["59005.80000","1","1.000"],
            "c":["59042.10000"],"v":["18.59019190","22.18713969"],
            "p":["58359.33086","58399.49802"],"t":[1420,1641],"l":["57500.00000","57500.00000"],
            "h":["59184.80000","59184.80000"],"o":"58778.00000"}}}"#;
        let result = serde_json::from_str::<KrakenTrade>(source);
        // println!("{:?}", result);
        assert!(result.is_ok());
        let price = result.unwrap().get_last_price();
        // println!("price = {:?}", price);
        assert!(price.is_none());
    }

    #[test]
    fn test_round_decimal() {
        let source = Decimal::from_str("59042.10000");
        let result = source.unwrap().round_dp(2);
        // let result = source.unwrap().trunc_with_scale(2).to_string();
        // println!("{:?}", result);
        assert_eq!("59042.10".to_string(), result.to_string());
    }
}
