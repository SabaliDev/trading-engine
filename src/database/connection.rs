use tokio_postgres::{Client, NoTls, Error};
use std::env;

pub struct DatabaseConnection {
    client: Client,
}

impl DatabaseConnection {
    pub async fn new() -> Result<Self, Error> {
        let database_url = env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgresql://trader:password123@localhost:5432/trading_engine".to_string());

        let (client, connection) = tokio_postgres::connect(&database_url, NoTls).await?;

        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("connection error: {}", e);
            }
        });

        Ok(DatabaseConnection { client })
    }

    pub fn get_client(&self) -> &Client {
        &self.client
    }

    pub async fn test_connection(&self) -> Result<(), Error> {
        self.client.simple_query("SELECT 1").await?;
        Ok(())
    }
}