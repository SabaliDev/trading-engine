use serde::{Deserialize, Serialize};
use rust_decimal::Decimal;
use chrono::{DateTime, Utc};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderBook {
    pub symbol: String,
    pub bids: BTreeMap<Decimal, PriceLevel>,
    pub asks: BTreeMap<Decimal, PriceLevel>,
    pub best_bid: Option<Decimal>,
    pub best_ask: Option<Decimal>,
    pub mid_price: Option<Decimal>,
    pub last_trade_price: Option<Decimal>,
    pub last_updated: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceLevel {
    pub price: Decimal,
    pub orders: Vec<OrderBookEntry>,
    pub total_quantity: Decimal,
    pub order_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderBookEntry {
    pub order_id: i32,
    pub quantity: Decimal,
    pub time: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderBookSnapshot {
    pub symbol: String,
    pub bids: Vec<QuoteLevel>,
    pub asks: Vec<QuoteLevel>,
    pub best_bid: Option<Decimal>,
    pub best_ask: Option<Decimal>,
    pub mid_price: Option<Decimal>,
    pub spread: Option<Decimal>,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuoteLevel {
    pub price: Decimal,
    pub quantity: Decimal,
    pub order_count: usize,
}

impl OrderBook {
    pub fn new(symbol: String) -> Self {
        Self {
            symbol,
            bids: BTreeMap::new(),
            asks: BTreeMap::new(),
            best_bid: None,
            best_ask: None,
            mid_price: None,
            last_trade_price: None,
            last_updated: Utc::now(),
        }
    }

    pub fn update_best_prices(&mut self) {
        self.best_bid = self.bids.keys().next_back().copied();
        self.best_ask = self.asks.keys().next().copied();
        
        if let (Some(bid), Some(ask)) = (self.best_bid, self.best_ask) {
            self.mid_price = Some((bid + ask) / Decimal::from(2));
        }
        
        self.last_updated = Utc::now();
    }

    pub fn get_spread(&self) -> Option<Decimal> {
        match (self.best_bid, self.best_ask) {
            (Some(bid), Some(ask)) => Some(ask - bid),
            _ => None,
        }
    }

    pub fn to_snapshot(&self, depth: Option<usize>) -> OrderBookSnapshot {
        let depth = depth.unwrap_or(10);
        
        let bids: Vec<QuoteLevel> = self.bids
            .iter()
            .rev()
            .take(depth)
            .map(|(price, level)| QuoteLevel {
                price: *price,
                quantity: level.total_quantity,
                order_count: level.order_count,
            })
            .collect();

        let asks: Vec<QuoteLevel> = self.asks
            .iter()
            .take(depth)
            .map(|(price, level)| QuoteLevel {
                price: *price,
                quantity: level.total_quantity,
                order_count: level.order_count,
            })
            .collect();

        OrderBookSnapshot {
            symbol: self.symbol.clone(),
            bids,
            asks,
            best_bid: self.best_bid,
            best_ask: self.best_ask,
            mid_price: self.mid_price,
            spread: self.get_spread(),
            timestamp: self.last_updated,
        }
    }
}