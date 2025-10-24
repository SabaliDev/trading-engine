mod matching_engine;
use matching_engine::engine::MatchingEngine;
use matching_engine::orderbook::{Order, BidOrAsk,OrderBook};

fn main() {
    
    let buy_order_alice = Order::new(BidOrAsk::Bid,5.5);
    let buy_order_bob = Order::new(BidOrAsk::Bid,2.45);

    let mut order_book = OrderBook::new();
    order_book.add_order(4.4,buy_order_alice);
    order_book.add_order(4.4,buy_order_bob);

    let sell_order = Order::new(BidOrAsk::Ask, 6.5);
    order_book.add_order(20.0,sell_order);

    println!("{:#?}", order_book);

    let engine = MatchingEngine::new();
}
