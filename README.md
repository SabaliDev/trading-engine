# Trading Engine

A high-performance trading engine built with Rust, Axum, and PostgreSQL.

## Features

- **User Management**: Account creation, balance tracking, P&L calculation
- **Order Management**: Limit, market, and stop orders with various time-in-force options
- **Position Tracking**: Real-time portfolio positions with average cost basis
- **Trade Execution**: Complete trade matching and execution history
- **Order Book**: Real-time order book management and snapshots
- **Market Data**: Best bid/ask, mid-price, and last trade information
- **REST API**: Comprehensive API for all trading operations

## Quick Start

### Prerequisites

- Docker and Docker Compose
- Rust (for development)

### Setup

1. **Clone and setup environment:**
   ```bash
   cp .env.example .env
   # Edit .env with your preferred settings
   ```

2. **Start the database:**
   ```bash
   docker-compose up -d postgres
   ```

3. **Run the trading engine:**
   ```bash
   cd trading-engine
   cargo run
   ```

4. **Access the API:**
   - Server: http://localhost:3000
   - Health check: http://localhost:3000/health

## Environment Configuration

Key environment variables in `.env`:

```env
# Database
POSTGRES_DB=trading_engine
POSTGRES_USER=trader
POSTGRES_PASSWORD=password123
POSTGRES_PORT=5432

# Application
DATABASE_URL=postgresql://trader:password123@localhost:5432/trading_engine
SERVER_PORT=3000
SERVER_HOST=0.0.0.0
```

## API Endpoints

### Health
- `GET /health` - System health check

### Users
- `POST /users` - Create user
- `GET /users/{user_id}` - Get user details
- `GET /users/{user_id}/profile` - Get user profile with positions

### Orders
- `POST /orders` - Create order
- `GET /orders?user_id=1&symbol=BTC` - Get orders (with filters)
- `POST /orders/cancel` - Cancel order

### Trades
- `GET /trades?user_id=1&symbol=BTC` - Get trades (with filters)

### Market Data
- `GET /orderbook/{symbol}?depth=10` - Get order book snapshot
- `GET /market/{symbol}` - Get market data

## Example API Usage

### Create a user:
```bash
curl -X POST http://localhost:3000/users \
  -H "Content-Type: application/json" \
  -d '{"username": "alice", "initial_balance": "10000.00"}'
```

### Create an order:
```bash
curl -X POST http://localhost:3000/orders \
  -H "Content-Type: application/json" \
  -d '{
    "user_id": 1,
    "symbol": "BTC",
    "side": "buy",
    "order_type": "limit",
    "quantity": "0.1",
    "limit_price": "50000.00"
  }'
```

### Get order book:
```bash
curl http://localhost:3000/orderbook/BTC?depth=5
```

## Development

### Database Management

**Reset database:**
```bash
docker-compose down -v
docker-compose up -d postgres
```

**View logs:**
```bash
docker-compose logs postgres
```

### Testing

```bash
cd trading-engine
cargo test
```

## Architecture

- **Backend**: Rust with Axum web framework
- **Database**: PostgreSQL with proper indexing
- **API**: RESTful JSON API
- **Data Models**: Comprehensive trading entities
- **Containerization**: Docker Compose for easy deployment

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests
5. Submit a pull request