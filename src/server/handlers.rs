use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use rust_decimal::Decimal;
use chrono::{DateTime, Utc};
use super::AppState;
use crate::models::*;

#[derive(Serialize)]
pub struct HealthResponse {
    status: String,
    database: String,
}

// Health check endpoint
pub async fn health_check(
    State(state): State<Arc<AppState>>,
) -> Result<Json<HealthResponse>, StatusCode> {
    let db_status = match state.db.test_connection().await {
        Ok(_) => "connected".to_string(),
        Err(_) => "disconnected".to_string(),
    };

    Ok(Json(HealthResponse {
        status: "healthy".to_string(),
        database: db_status,
    }))
}

// User management endpoints
pub async fn create_user(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<Json<User>, StatusCode> {
    let client = state.db.get_client();
    let initial_balance = payload.initial_balance.unwrap_or_default();
    
    let row = client
        .query_one(
            "INSERT INTO users (username, cash_balance) VALUES ($1, $2) RETURNING user_id, username, cash_balance, realized_pnl, unrealized_pnl, created_at, updated_at",
            &[&payload.username, &initial_balance.to_string()],
        )
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let user = User {
        user_id: row.get(0),
        username: row.get(1),
        cash_balance: row.get::<_, String>(2).parse().unwrap_or_default(),
        realized_pnl: row.get::<_, String>(3).parse().unwrap_or_default(),
        unrealized_pnl: row.get::<_, String>(4).parse().unwrap_or_default(),
        created_at: row.get(5),
        updated_at: row.get(6),
    };

    Ok(Json(user))
}

pub async fn get_user(
    State(state): State<Arc<AppState>>,
    Path(user_id): Path<i32>,
) -> Result<Json<User>, StatusCode> {
    let client = state.db.get_client();
    
    let row = client
        .query_one(
            "SELECT user_id, username, cash_balance, realized_pnl, unrealized_pnl, created_at, updated_at FROM users WHERE user_id = $1",
            &[&user_id],
        )
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    let user = User {
        user_id: row.get(0),
        username: row.get(1),
        cash_balance: row.get::<_, String>(2).parse().unwrap_or_default(),
        realized_pnl: row.get::<_, String>(3).parse().unwrap_or_default(),
        unrealized_pnl: row.get::<_, String>(4).parse().unwrap_or_default(),
        created_at: row.get(5),
        updated_at: row.get(6),
    };

    Ok(Json(user))
}

pub async fn get_user_profile(
    State(state): State<Arc<AppState>>,
    Path(user_id): Path<i32>,
) -> Result<Json<UserProfile>, StatusCode> {
    let client = state.db.get_client();
    
    // Get user details
    let user_row = client
        .query_one(
            "SELECT user_id, username, cash_balance, realized_pnl, unrealized_pnl FROM users WHERE user_id = $1",
            &[&user_id],
        )
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    // Get user positions
    let position_rows = client
        .query(
            "SELECT position_id, user_id, symbol, quantity, avg_cost, updated_at FROM positions WHERE user_id = $1",
            &[&user_id],
        )
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let positions: Vec<Position> = position_rows
        .iter()
        .map(|row| Position {
            position_id: row.get(0),
            user_id: row.get(1),
            symbol: row.get(2),
            quantity: row.get::<_, String>(3).parse().unwrap_or_default(),
            avg_cost: row.get::<_, String>(4).parse().unwrap_or_default(),
            updated_at: row.get(5),
        })
        .collect();

    let profile = UserProfile {
        user_id: user_row.get(0),
        username: user_row.get(1),
        cash_balance: user_row.get::<_, String>(2).parse().unwrap_or_default(),
        realized_pnl: user_row.get::<_, String>(3).parse().unwrap_or_default(),
        unrealized_pnl: user_row.get::<_, String>(4).parse().unwrap_or_default(),
        positions,
    };

    Ok(Json(profile))
}

// Order management endpoints
pub async fn create_order(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateOrderRequest>,
) -> Result<Json<Order>, StatusCode> {
    let client = state.db.get_client();
    
    let time_in_force = payload.time_in_force.unwrap_or(TimeInForce::GTC);
    
    let row = client
        .query_one(
            "INSERT INTO orders (user_id, symbol, side, order_type, quantity, limit_price, remaining_quantity, time_in_force) 
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8) 
             RETURNING order_id, user_id, symbol, side, order_type, quantity, limit_price, filled_quantity, remaining_quantity, status, time_in_force, submission_time, updated_at",
            &[
                &payload.user_id, 
                &payload.symbol, 
                &payload.side.to_string(), 
                &payload.order_type.to_string(), 
                &payload.quantity.to_string(), 
                &payload.limit_price.map(|p| p.to_string()),
                &payload.quantity.to_string(),
                &time_in_force.to_string()
            ],
        )
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let order = Order {
        order_id: row.get(0),
        user_id: row.get(1),
        symbol: row.get(2),
        side: match row.get::<_, String>(3).as_str() {
            "buy" => OrderSide::Buy,
            "sell" => OrderSide::Sell,
            _ => return Err(StatusCode::INTERNAL_SERVER_ERROR),
        },
        order_type: match row.get::<_, String>(4).as_str() {
            "limit" => OrderType::Limit,
            "market" => OrderType::Market,
            "stop" => OrderType::Stop,
            _ => return Err(StatusCode::INTERNAL_SERVER_ERROR),
        },
        quantity: row.get::<_, String>(5).parse().unwrap_or_default(),
        limit_price: row.get::<_, Option<String>>(6).and_then(|p| p.parse().ok()),
        filled_quantity: row.get::<_, String>(7).parse().unwrap_or_default(),
        remaining_quantity: row.get::<_, String>(8).parse().unwrap_or_default(),
        status: match row.get::<_, String>(9).as_str() {
            "pending" => OrderStatus::Pending,
            "active" => OrderStatus::Active,
            "filled" => OrderStatus::Filled,
            "cancelled" => OrderStatus::Cancelled,
            "rejected" => OrderStatus::Rejected,
            _ => OrderStatus::Pending,
        },
        time_in_force: match row.get::<_, String>(10).as_str() {
            "GTC" => TimeInForce::GTC,
            "IOC" => TimeInForce::IOC,
            "FOK" => TimeInForce::FOK,
            "DAY" => TimeInForce::DAY,
            _ => TimeInForce::GTC,
        },
        submission_time: row.get(11),
        updated_at: row.get(12),
    };

    Ok(Json(order))
}

pub async fn get_orders(
    State(state): State<Arc<AppState>>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<Vec<Order>>, StatusCode> {
    let client = state.db.get_client();
    let symbol = params.get("symbol");
    let user_id = params.get("user_id").and_then(|id| id.parse::<i32>().ok());

    let query_str = match (symbol, user_id) {
        (Some(_), Some(_)) => 
            "SELECT order_id, user_id, symbol, side, order_type, quantity, limit_price, filled_quantity, remaining_quantity, status, time_in_force, submission_time, updated_at FROM orders WHERE symbol = $1 AND user_id = $2 ORDER BY submission_time DESC",
        (Some(_), None) => 
            "SELECT order_id, user_id, symbol, side, order_type, quantity, limit_price, filled_quantity, remaining_quantity, status, time_in_force, submission_time, updated_at FROM orders WHERE symbol = $1 ORDER BY submission_time DESC",
        (None, Some(_)) => 
            "SELECT order_id, user_id, symbol, side, order_type, quantity, limit_price, filled_quantity, remaining_quantity, status, time_in_force, submission_time, updated_at FROM orders WHERE user_id = $1 ORDER BY submission_time DESC",
        (None, None) => 
            "SELECT order_id, user_id, symbol, side, order_type, quantity, limit_price, filled_quantity, remaining_quantity, status, time_in_force, submission_time, updated_at FROM orders ORDER BY submission_time DESC LIMIT 100",
    };

    let rows = match (symbol, user_id) {
        (Some(sym), Some(uid)) => client.query(query_str, &[&sym, &uid]).await,
        (Some(sym), None) => client.query(query_str, &[&sym]).await,
        (None, Some(uid)) => client.query(query_str, &[&uid]).await,
        (None, None) => client.query(query_str, &[]).await,
    }.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let orders: Vec<Order> = rows
        .iter()
        .map(|row| {
            Order {
                order_id: row.get(0),
                user_id: row.get(1),
                symbol: row.get(2),
                side: match row.get::<_, String>(3).as_str() {
                    "buy" => OrderSide::Buy,
                    "sell" => OrderSide::Sell,
                    _ => OrderSide::Buy,
                },
                order_type: match row.get::<_, String>(4).as_str() {
                    "limit" => OrderType::Limit,
                    "market" => OrderType::Market,
                    "stop" => OrderType::Stop,
                    _ => OrderType::Limit,
                },
                quantity: row.get::<_, String>(5).parse().unwrap_or_default(),
                limit_price: row.get::<_, Option<String>>(6).and_then(|p| p.parse().ok()),
                filled_quantity: row.get::<_, String>(7).parse().unwrap_or_default(),
                remaining_quantity: row.get::<_, String>(8).parse().unwrap_or_default(),
                status: match row.get::<_, String>(9).as_str() {
                    "pending" => OrderStatus::Pending,
                    "active" => OrderStatus::Active,
                    "filled" => OrderStatus::Filled,
                    "cancelled" => OrderStatus::Cancelled,
                    "rejected" => OrderStatus::Rejected,
                    _ => OrderStatus::Pending,
                },
                time_in_force: match row.get::<_, String>(10).as_str() {
                    "GTC" => TimeInForce::GTC,
                    "IOC" => TimeInForce::IOC,
                    "FOK" => TimeInForce::FOK,
                    "DAY" => TimeInForce::DAY,
                    _ => TimeInForce::GTC,
                },
                submission_time: row.get(11),
                updated_at: row.get(12),
            }
        })
        .collect();

    Ok(Json(orders))
}

pub async fn cancel_order(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CancelOrderRequest>,
) -> Result<Json<Order>, StatusCode> {
    let client = state.db.get_client();
    
    let row = client
        .query_one(
            "UPDATE orders SET status = 'cancelled', updated_at = CURRENT_TIMESTAMP 
             WHERE order_id = $1 AND user_id = $2 AND status IN ('pending', 'active')
             RETURNING order_id, user_id, symbol, side, order_type, quantity, limit_price, filled_quantity, remaining_quantity, status, time_in_force, submission_time, updated_at",
            &[&payload.order_id, &payload.user_id],
        )
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    let order = Order {
        order_id: row.get(0),
        user_id: row.get(1),
        symbol: row.get(2),
        side: match row.get::<_, String>(3).as_str() {
            "buy" => OrderSide::Buy,
            "sell" => OrderSide::Sell,
            _ => OrderSide::Buy,
        },
        order_type: match row.get::<_, String>(4).as_str() {
            "limit" => OrderType::Limit,
            "market" => OrderType::Market,
            "stop" => OrderType::Stop,
            _ => OrderType::Limit,
        },
        quantity: row.get::<_, String>(5).parse().unwrap_or_default(),
        limit_price: row.get::<_, Option<String>>(6).and_then(|p| p.parse().ok()),
        filled_quantity: row.get::<_, String>(7).parse().unwrap_or_default(),
        remaining_quantity: row.get::<_, String>(8).parse().unwrap_or_default(),
        status: OrderStatus::Cancelled,
        time_in_force: match row.get::<_, String>(10).as_str() {
            "GTC" => TimeInForce::GTC,
            "IOC" => TimeInForce::IOC,
            "FOK" => TimeInForce::FOK,
            "DAY" => TimeInForce::DAY,
            _ => TimeInForce::GTC,
        },
        submission_time: row.get(11),
        updated_at: row.get(12),
    };

    Ok(Json(order))
}

// Trade endpoints
pub async fn get_trades(
    State(state): State<Arc<AppState>>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<Vec<Trade>>, StatusCode> {
    let client = state.db.get_client();
    let symbol = params.get("symbol");
    let user_id = params.get("user_id").and_then(|id| id.parse::<i32>().ok());

    let query_str = match (symbol, user_id) {
        (Some(_), Some(_)) => 
            "SELECT trade_id, symbol, price, quantity, buy_order_id, sell_order_id, buyer_user_id, seller_user_id, aggressor_side, timestamp FROM trades WHERE symbol = $1 AND (buyer_user_id = $2 OR seller_user_id = $2) ORDER BY timestamp DESC",
        (Some(_), None) => 
            "SELECT trade_id, symbol, price, quantity, buy_order_id, sell_order_id, buyer_user_id, seller_user_id, aggressor_side, timestamp FROM trades WHERE symbol = $1 ORDER BY timestamp DESC",
        (None, Some(_)) => 
            "SELECT trade_id, symbol, price, quantity, buy_order_id, sell_order_id, buyer_user_id, seller_user_id, aggressor_side, timestamp FROM trades WHERE buyer_user_id = $1 OR seller_user_id = $1 ORDER BY timestamp DESC",
        (None, None) => 
            "SELECT trade_id, symbol, price, quantity, buy_order_id, sell_order_id, buyer_user_id, seller_user_id, aggressor_side, timestamp FROM trades ORDER BY timestamp DESC LIMIT 100",
    };

    let rows = match (symbol, user_id) {
        (Some(sym), Some(uid)) => client.query(query_str, &[&sym, &uid]).await,
        (Some(sym), None) => client.query(query_str, &[&sym]).await,
        (None, Some(uid)) => client.query(query_str, &[&uid]).await,
        (None, None) => client.query(query_str, &[]).await,
    }.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let trades: Vec<Trade> = rows
        .iter()
        .map(|row| {
            Trade {
                trade_id: row.get(0),
                symbol: row.get(1),
                price: row.get::<_, String>(2).parse().unwrap_or_default(),
                quantity: row.get::<_, String>(3).parse().unwrap_or_default(),
                buy_order_id: row.get(4),
                sell_order_id: row.get(5),
                buyer_user_id: row.get(6),
                seller_user_id: row.get(7),
                aggressor_side: match row.get::<_, String>(8).as_str() {
                    "buy" => OrderSide::Buy,
                    "sell" => OrderSide::Sell,
                    _ => OrderSide::Buy,
                },
                timestamp: row.get(9),
            }
        })
        .collect();

    Ok(Json(trades))
}

// Order book endpoints
pub async fn get_order_book(
    State(state): State<Arc<AppState>>,
    Path(symbol): Path<String>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<OrderBookSnapshot>, StatusCode> {
    let client = state.db.get_client();
    let depth = params.get("depth").and_then(|d| d.parse::<usize>().ok()).unwrap_or(10);

    // Get bids (buy orders)
    let bid_rows = client
        .query(
            "SELECT price, total_quantity, order_count FROM order_book_entries 
             WHERE symbol = $1 AND side = 'bid' ORDER BY price DESC LIMIT $2",
            &[&symbol, &(depth as i64)],
        )
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Get asks (sell orders)
    let ask_rows = client
        .query(
            "SELECT price, total_quantity, order_count FROM order_book_entries 
             WHERE symbol = $1 AND side = 'ask' ORDER BY price ASC LIMIT $2",
            &[&symbol, &(depth as i64)],
        )
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Get market data
    let market_row = client
        .query_opt(
            "SELECT best_bid, best_ask, mid_price, last_trade_price FROM market_data WHERE symbol = $1",
            &[&symbol],
        )
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let bids: Vec<QuoteLevel> = bid_rows
        .iter()
        .map(|row| QuoteLevel {
            price: row.get::<_, String>(0).parse().unwrap_or_default(),
            quantity: row.get::<_, String>(1).parse().unwrap_or_default(),
            order_count: row.get::<_, i32>(2) as usize,
        })
        .collect();

    let asks: Vec<QuoteLevel> = ask_rows
        .iter()
        .map(|row| QuoteLevel {
            price: row.get::<_, String>(0).parse().unwrap_or_default(),
            quantity: row.get::<_, String>(1).parse().unwrap_or_default(),
            order_count: row.get::<_, i32>(2) as usize,
        })
        .collect();

    let (best_bid, best_ask, mid_price) = if let Some(row) = market_row {
        (
            row.get::<_, Option<String>>(0).and_then(|p| p.parse().ok()),
            row.get::<_, Option<String>>(1).and_then(|p| p.parse().ok()),
            row.get::<_, Option<String>>(2).and_then(|p| p.parse().ok()),
        )
    } else {
        (None, None, None)
    };

    let spread = match (best_bid, best_ask) {
        (Some(bid), Some(ask)) => Some(ask - bid),
        _ => None,
    };

    let snapshot = OrderBookSnapshot {
        symbol,
        bids,
        asks,
        best_bid,
        best_ask,
        mid_price,
        spread,
        timestamp: Utc::now(),
    };

    Ok(Json(snapshot))
}

// Market data endpoints
pub async fn get_market_data(
    State(state): State<Arc<AppState>>,
    Path(symbol): Path<String>,
) -> Result<Json<MarketData>, StatusCode> {
    let client = state.db.get_client();
    
    let row = client
        .query_one(
            "SELECT symbol, best_bid, best_ask, mid_price, last_trade_price, last_trade_time, updated_at FROM market_data WHERE symbol = $1",
            &[&symbol],
        )
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    let market_data = MarketData {
        symbol: row.get(0),
        best_bid: row.get::<_, Option<String>>(1).and_then(|p| p.parse().ok()),
        best_ask: row.get::<_, Option<String>>(2).and_then(|p| p.parse().ok()),
        mid_price: row.get::<_, Option<String>>(3).and_then(|p| p.parse().ok()),
        last_trade_price: row.get::<_, Option<String>>(4).and_then(|p| p.parse().ok()),
        last_trade_time: row.get(5),
        updated_at: row.get(6),
    };

    Ok(Json(market_data))
}