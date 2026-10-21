use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Debug)]
pub struct AggTrade{
    pub symbol: String,
    pub trade_time: u64,
    pub price: Decimal,
    pub quantity: Decimal,
    pub is_buy: bool
}

#[derive(Deserialize)]
struct RawAggTradeEvent{
    s: String,
    #[allow(non_snake_case)]
    T: u64,
    p: String,
    q: String,
    m: bool
}

impl From<RawAggTradeEvent> for AggTrade {
    fn from(value: RawAggTradeEvent) -> Self {
        Self {
            symbol: (value.s),
            trade_time: (value.T),
            price: (value.p.parse().unwrap()),
            quantity: (value.q.parse().unwrap()),
            is_buy: (!value.m) 
        }
    }
}
pub fn handle_payload(payload_str: &str) -> Result<(), String> {

    let raw_payload: RawAggTradeEvent = serde_json::from_str(payload_str)
        .map_err(|e| e.to_string())?;

    let payload: AggTrade = raw_payload.into();
    
    //WHAT TO DO WITH THE PAYLOAD

    println!("{:?}", payload);


    Ok(())



}