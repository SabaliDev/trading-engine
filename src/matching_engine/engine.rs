#![allow(dead_code)]
use super::orderbook::{OrderBook,Order};
use std::{collections::HashMap};
use rust_decimal::prelude::*;

#[derive(PartialEq, Eq, Hash, Clone,Debug)]
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
    pub fn to_string(&self) ->String{
        format!("{}/{}",self.base,self.quote)
    }
}
#[derive(Debug)]
pub struct MatchingEngine {
    orderbooks: HashMap<TradingPair,OrderBook>
}

impl MatchingEngine{
 pub fn new() -> Self {
    MatchingEngine{
        orderbooks:HashMap::new(),
    }
 }
 pub fn add_new_market(&mut self, pair: TradingPair){
    self.orderbooks.insert(pair.clone(), OrderBook::new());
    print!("Opening new orderbook for market {:?}", pair.to_string());
 }
 pub fn place_limit_order(&mut self, pair: TradingPair, price:Decimal, order:Order) -> Result<(),String>{
    match self.orderbooks.get_mut(&pair){
        Some(orderbook) => {
            orderbook.add_limit_order(price,order);

            println!("Placed limit order @ price {}", price);
            Ok(())
        }
        None => {
            Err(format!("The order book for the given trading pair ({})does not exist",pair.to_string()))
        }
    }

   
 }
}