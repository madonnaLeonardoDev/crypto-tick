use std::{str::FromStr};

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_json::{self, Value};

#[derive(Serialize, Deserialize)]
pub struct RawPayload {
    stream: String,
    data: Value
}


#[derive(Deserialize,Serialize, Debug)]
pub struct AggTrade{
    pub symbol: String,
    pub trade_time: u64,
    pub price: Decimal,
    pub quantity: Decimal,
    pub is_buy: bool
}

#[derive(Deserialize, Serialize)]
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

#[derive(Serialize, Deserialize)]
pub struct BookTicker{
    pub symbol: String,
    pub transaction_time: u64,
    pub bid_price: Decimal,
    pub bid_quantity: Decimal,
    pub ask_price: Decimal,
    pub ask_quantity: Decimal
}

#[derive(Serialize, Deserialize)]
pub struct RawBookTickerEvent {
    s: String,
    #[allow(non_snake_case)]
    T:u64,
    b: String,
    #[allow(non_snake_case)]
    B: String,
    a: String,
    #[allow(non_snake_case)]
    A: String
}

impl From<RawBookTickerEvent> for BookTicker {
    fn from(value: RawBookTickerEvent) -> Self {
        Self {
            symbol: (value.s),
            transaction_time: (value.T),
            bid_price: (Decimal::from_str(&value.b).unwrap()),
            bid_quantity: (Decimal::from_str(&value.B).unwrap()),
            ask_price: (Decimal::from_str(&value.a).unwrap()),
            ask_quantity: (Decimal::from_str(&value.A).unwrap())
        }
    }
}

pub fn handle_payload(payload_str: &str) -> Result<(), String> {

    let raw_payload: RawPayload = serde_json::from_str(payload_str)
        .map_err(|e| e.to_string())?;

    let (_symbol, stream_type) = raw_payload.stream.split_once("@").unwrap();

    match stream_type {
        "aggTrade" => {
            let raw_data: RawAggTradeEvent = serde_json::from_value(raw_payload.data)
                .map_err(|e| e.to_string())?; 
            let data: AggTrade = raw_data.into();
        },
        s if s.starts_with("depth") => {

        },
        "forceOrder" => {

        },
        _=> {

        }
    };

    /* let raw_payload: RawAggTradeEvent = serde_json::from_str(payload_str)
        .map_err(|e| e.to_string())?;

    let payload: AggTrade = raw_payload.into();
    
    //WHAT TO DO WITH THE PAYLOAD

    println!("{:?}", payload);

    */

    println!("{}", payload_str);

    Ok(())



}