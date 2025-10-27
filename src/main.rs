mod matching_engine;
use matching_engine::engine::MatchingEngine;
use matching_engine::orderbook::{Order, BidOrAsk,OrderBook};

use crate::matching_engine::engine::TradingPair;
use rust_decimal_macros::dec;

fn main() {
    
    let buy_order_alice = Order::new(BidOrAsk::Bid,5.5);
    let buy_order_bob = Order::new(BidOrAsk::Bid,2.45);

    let mut order_book = OrderBook::new();
    order_book.add_limit_order(dec!(4.4),buy_order_alice);
    order_book.add_limit_order(dec!(4.4),buy_order_bob);

    let sell_order = Order::new(BidOrAsk::Ask, 6.5);
    order_book.add_limit_order(dec!(20.0),sell_order);

    //println!("{:#?}", order_book);

    let mut engine = MatchingEngine::new();
    let pair  = TradingPair::new("BTC".to_string(),"USD".to_string());
    engine.add_new_market(pair.clone());

    let buy_order = Order::new(BidOrAsk::Bid,6.5);
    engine.place_limit_order(pair,dec!(10.000),buy_order).unwrap()

    
}
