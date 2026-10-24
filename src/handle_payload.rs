use std::{str::FromStr};
use std::time::{SystemTime, UNIX_EPOCH};
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
            symbol: value.s.to_ascii_lowercase(),
            trade_time: value.T,
            price: value.p.parse().unwrap(),
            quantity: value.q.parse().unwrap(),
            is_buy: !value.m 
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BookTicker{
    pub symbol: String,
    pub local_time: u64,
    pub bid_price: Decimal,
    pub bid_quantity: Decimal,
    pub ask_price: Decimal,
    pub ask_quantity: Decimal
}

#[derive(Serialize, Deserialize)]
pub struct RawBookTickerEvent {
    s: String,
    #[allow(non_snake_case)]
    b: String,
    #[allow(non_snake_case)]
    B: String,
    a: String,
    #[allow(non_snake_case)]
    A: String
}

impl From<RawBookTickerEvent> for BookTicker {
    fn from(value: RawBookTickerEvent) -> Self {
        let local_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();
        Self {
            symbol: value.s.to_ascii_lowercase(),
            local_time: local_time as u64,
            bid_price: Decimal::from_str(&value.b).unwrap(),
            bid_quantity: Decimal::from_str(&value.B).unwrap(),
            ask_price: Decimal::from_str(&value.a).unwrap(),
            ask_quantity: Decimal::from_str(&value.A).unwrap()
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BidAsk{
    quantity: Decimal,
    price: Decimal
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DepthLevels {
    symbol: String,
    local_time: u64,
    bids: Vec<BidAsk>,
    asks: Vec<BidAsk>
}

#[derive(Serialize, Deserialize)]
pub struct RawDepthLevelsEvent {
    bids: Vec<(String, String)>,
    asks: Vec<(String, String)>
}

impl RawDepthLevelsEvent {
    fn into_symbol(self, s: &str) -> DepthLevels {
        let mut bids: Vec<BidAsk> = Vec::with_capacity(self.bids.len());
        let mut asks: Vec<BidAsk> = Vec::with_capacity(self.asks.len());

        let local_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();

        for e in self.bids {
            bids.push(BidAsk { quantity: Decimal::from_str(&e.1).unwrap() , price: Decimal::from_str(&e.0).unwrap() });
        }
        for e in self.asks {
            asks.push(BidAsk { quantity: Decimal::from_str(&e.1).unwrap() , price: Decimal::from_str(&e.0).unwrap() });
        }
        
        DepthLevels {
            symbol: s.to_string().to_ascii_lowercase(),
            local_time: local_time as u64,
            bids: bids,
            asks: asks 
        }
    }
}


pub fn handle_payload(payload_str: &str) -> Result<(), String> {

    let raw_payload: RawPayload = serde_json::from_str(payload_str)
        .map_err(|e| e.to_string())?;

    let (symbol, stream_type) = raw_payload.stream.split_once("@").unwrap();

    match stream_type {
        "aggTrade" => {
            let raw_data: RawAggTradeEvent = serde_json::from_value(raw_payload.data)
                .map_err(|e| e.to_string())?; 
            let data: AggTrade = raw_data.into();

            println!("{:?}", data);
        },
        s if s.starts_with("depth") => {
            println!("{}", raw_payload.data);
            
            let raw_data: RawDepthLevelsEvent = serde_json::from_value(raw_payload.data)
                .map_err(|e| e.to_string())?;
            let data: DepthLevels = raw_data.into_symbol(symbol);
            println!("{:?}", data);
        },
        "bookTicker" => {
            let raw_data: RawBookTickerEvent = serde_json::from_value(raw_payload.data)
                .map_err(|e| e.to_string())?;
            let data: BookTicker = raw_data.into();
            println!("{:?}", data);
        },
        _=> {
            return Err("Unkown Stream Type value".to_string())
        },

    };

    Ok(())



}