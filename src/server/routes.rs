use axum::{
    routing::{get, post, put, delete},
    Router,
};
use std::sync::Arc;
use super::{handlers, AppState};

pub fn create_routes() -> Router<Arc<AppState>> {
    Router::new()
        // Health check
        .route("/health", get(handlers::health_check))
        
        // User management
        .route("/users", post(handlers::create_user))
        .route("/users/:user_id", get(handlers::get_user))
        .route("/users/:user_id/profile", get(handlers::get_user_profile))
        
        // Order management
        .route("/orders", post(handlers::create_order))
        .route("/orders", get(handlers::get_orders))
        .route("/orders/cancel", post(handlers::cancel_order))
        
        // Trade data
        .route("/trades", get(handlers::get_trades))
        
        // Order book and market data
        .route("/orderbook/:symbol", get(handlers::get_order_book))
        .route("/market/:symbol", get(handlers::get_market_data))
}