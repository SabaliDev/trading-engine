use super::orderbook::OrderBook;
use std::collections::HashMap;


pub struct TradingPair{
    base: String,
    quote: String
}

impl TradingPair {
    pub fn new(base: String, quote:String) ->Self{
        TradingPair{
            base, quote
        }
    }
}

pub struct MatchingEngine {
    orderbooks: HashMap<TradingPair,OrderBook>
}

impl MatchingEngine{
 pub fn new() -> Self {
    MatchingEngine{
        orderbooks:HashMap::new(),
    }
 }   
}