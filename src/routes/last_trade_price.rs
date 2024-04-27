use std::sync::{Arc, Mutex};

use actix_web::{get, HttpResponse};
use actix_web::http::StatusCode;
use actix_web::web::{Data, Json};
use log::{debug, error};

use crate::client::rest::{fetch_pair_trade_response, RestClient};
use crate::constants::CURRENCY_PAIRS_LIST;
use crate::models::response::{LtpItem, LtpListResponse};

/// Route to get Last Time Price assets by different parameters
#[get("/ltp")]
async fn get_ltp(client: Data<Arc<RestClient>>) -> HttpResponse {
    // put results from separate spawned thread into that variable
    let fetch_result =
        Arc::new(Mutex::new(LtpListResponse::new_with_capacity(CURRENCY_PAIRS_LIST.len())));

    debug!("Before spawning loop...");

    for i in 0..CURRENCY_PAIRS_LIST.len() {
        let pair = CURRENCY_PAIRS_LIST[i]; // processed pair
        let shared_client = Arc::clone(&client);
        let shared_fetch_result = Arc::clone(&fetch_result);
        actix_rt::spawn(async move {
            match fetch_pair_trade_response(pair, &shared_client).await {
                Ok(trade) => {
                    if trade.result.is_some() {
                        let new_item = LtpItem::new(pair.to_string(), trade.get_last_price().unwrap());
                        {
                            shared_fetch_result.lock().expect("LTP Lock error").add_item(new_item);
                        }
                    }
                }
                Err(err) => {
                    error!("Not fetched pair = {:?} due to error: {}", &pair, err);
                }
            }
        }).await.unwrap();
    }
    let fetched_pairs =
        Arc::<Mutex<LtpListResponse>>::try_unwrap(fetch_result).unwrap().into_inner().unwrap();

    HttpResponse::Ok()
        .status(StatusCode::OK)
        .content_type("application/json")
        .body(
            serde_json::to_string(&Json(fetched_pairs))
                .unwrap_or_else(|e| format!("JSON serialization error: {}", e)),
        )
}