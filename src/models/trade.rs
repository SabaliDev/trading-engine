use serde::{Deserialize, Serialize};
use rust_decimal::Decimal;
use chrono::{DateTime, Utc};
use super::OrderSide;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trade {
    pub trade_id: i32,
    pub symbol: String,
    pub price: Decimal,
    pub quantity: Decimal,
    pub buy_order_id: i32,
    pub sell_order_id: i32,
    pub buyer_user_id: i32,
    pub seller_user_id: i32,
    pub aggressor_side: OrderSide,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeHistory {
    pub trades: Vec<Trade>,
    pub total_volume: Decimal,
    pub total_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserTradeHistory {
    pub user_id: i32,
    pub trades: Vec<UserTrade>,
    pub total_bought: Decimal,
    pub total_sold: Decimal,
    pub total_fees: Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserTrade {
    pub trade_id: i32,
    pub symbol: String,
    pub side: OrderSide,
    pub price: Decimal,
    pub quantity: Decimal,
    pub order_id: i32,
    pub timestamp: DateTime<Utc>,
    pub fees: Decimal,
}