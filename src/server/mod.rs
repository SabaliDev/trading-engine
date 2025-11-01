pub mod routes;
pub mod handlers;

use axum::{Router, serve};
use tower_http::cors::CorsLayer;
use tokio::net::TcpListener;
use std::sync::Arc;
use crate::database::DatabaseConnection;

pub struct AppState {
    pub db: Arc<DatabaseConnection>,
}

pub async fn create_app(db: DatabaseConnection) -> Router {
    let state = Arc::new(AppState {
        db: Arc::new(db),
    });

    Router::new()
        .merge(routes::create_routes())
        .layer(CorsLayer::permissive())
        .with_state(state)
}

pub async fn start_server(app: Router) -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("0.0.0.0:3000").await?;
    println!("Server running on http://localhost:3000");
    
    serve(listener, app).await?;
    Ok(())
}