-- Initialize trading engine database

-- Users table
CREATE TABLE IF NOT EXISTS users (
    user_id SERIAL PRIMARY KEY,
    username VARCHAR(50) UNIQUE NOT NULL,
    cash_balance DECIMAL(18, 8) DEFAULT 0,
    realized_pnl DECIMAL(18, 8) DEFAULT 0,
    unrealized_pnl DECIMAL(18, 8) DEFAULT 0,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Positions table
CREATE TABLE IF NOT EXISTS positions (
    position_id SERIAL PRIMARY KEY,
    user_id INTEGER REFERENCES users(user_id) ON DELETE CASCADE,
    symbol VARCHAR(20) NOT NULL,
    quantity DECIMAL(18, 8) NOT NULL,
    avg_cost DECIMAL(18, 8) NOT NULL,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(user_id, symbol)
);

-- Orders table (enhanced)
CREATE TABLE IF NOT EXISTS orders (
    order_id SERIAL PRIMARY KEY,
    user_id INTEGER REFERENCES users(user_id) ON DELETE CASCADE,
    symbol VARCHAR(20) NOT NULL,
    side VARCHAR(4) CHECK (side IN ('buy', 'sell')) NOT NULL,
    order_type VARCHAR(10) CHECK (order_type IN ('limit', 'market', 'stop')) NOT NULL,
    quantity DECIMAL(18, 8) NOT NULL,
    limit_price DECIMAL(18, 8),
    filled_quantity DECIMAL(18, 8) DEFAULT 0,
    remaining_quantity DECIMAL(18, 8) NOT NULL,
    status VARCHAR(10) CHECK (status IN ('pending', 'active', 'filled', 'cancelled', 'rejected')) DEFAULT 'pending',
    time_in_force VARCHAR(10) CHECK (time_in_force IN ('GTC', 'IOC', 'FOK', 'DAY')) DEFAULT 'GTC',
    submission_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Trades table (enhanced)
CREATE TABLE IF NOT EXISTS trades (
    trade_id SERIAL PRIMARY KEY,
    symbol VARCHAR(20) NOT NULL,
    price DECIMAL(18, 8) NOT NULL,
    quantity DECIMAL(18, 8) NOT NULL,
    buy_order_id INTEGER REFERENCES orders(order_id),
    sell_order_id INTEGER REFERENCES orders(order_id),
    buyer_user_id INTEGER REFERENCES users(user_id),
    seller_user_id INTEGER REFERENCES users(user_id),
    aggressor_side VARCHAR(4) CHECK (aggressor_side IN ('buy', 'sell')) NOT NULL,
    timestamp TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Order book entries table (for maintaining order book state)
CREATE TABLE IF NOT EXISTS order_book_entries (
    id SERIAL PRIMARY KEY,
    symbol VARCHAR(20) NOT NULL,
    side VARCHAR(4) CHECK (side IN ('bid', 'ask')) NOT NULL,
    price DECIMAL(18, 8) NOT NULL,
    total_quantity DECIMAL(18, 8) NOT NULL,
    order_count INTEGER NOT NULL,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(symbol, side, price)
);

-- Market data table for storing best bid/ask and last trade
CREATE TABLE IF NOT EXISTS market_data (
    symbol VARCHAR(20) PRIMARY KEY,
    best_bid DECIMAL(18, 8),
    best_ask DECIMAL(18, 8),
    mid_price DECIMAL(18, 8),
    last_trade_price DECIMAL(18, 8),
    last_trade_time TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Indexes for performance
CREATE INDEX IF NOT EXISTS idx_orders_user_id ON orders(user_id);
CREATE INDEX IF NOT EXISTS idx_orders_symbol ON orders(symbol);
CREATE INDEX IF NOT EXISTS idx_orders_status ON orders(status);
CREATE INDEX IF NOT EXISTS idx_trades_symbol ON trades(symbol);
CREATE INDEX IF NOT EXISTS idx_trades_timestamp ON trades(timestamp);
CREATE INDEX IF NOT EXISTS idx_positions_user_id ON positions(user_id);
CREATE INDEX IF NOT EXISTS idx_order_book_symbol_side ON order_book_entries(symbol, side);

-- Insert sample users
INSERT INTO users (username, cash_balance) VALUES ('trader1', 100000.00) ON CONFLICT DO NOTHING;
INSERT INTO users (username, cash_balance) VALUES ('trader2', 50000.00) ON CONFLICT DO NOTHING;

-- Insert sample market data
INSERT INTO market_data (symbol) VALUES ('BTC') ON CONFLICT DO NOTHING;
INSERT INTO market_data (symbol) VALUES ('ETH') ON CONFLICT DO NOTHING;
INSERT INTO market_data (symbol) VALUES ('AAPL') ON CONFLICT DO NOTHING;