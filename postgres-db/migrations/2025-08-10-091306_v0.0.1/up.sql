-- Your SQL goes here
-- Enable uuid-ossp extension for UUID generation (Postgres)
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- 1. Users table
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    username VARCHAR(50) UNIQUE NOT NULL,
    email VARCHAR(100) UNIQUE NOT NULL,
    password_hash TEXT NOT NULL,
    wallet_pubkey VARCHAR(44) NOT NULL,
    wallet_privkey_enc BYTEA NOT NULL,  -- encrypted private key
    is_active BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    last_login_at TIMESTAMPTZ
);

-- 2. User balances (per token)
CREATE TABLE user_balances (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    token_mint VARCHAR(44) NOT NULL, -- Solana token mint address or 'SOL' for native SOL
    available_balance NUMERIC(30,10) DEFAULT 0 NOT NULL,
    locked_balance NUMERIC(30,10) DEFAULT 0 NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(user_id, token_mint)
);

CREATE INDEX idx_user_balances_user_id ON user_balances(user_id);


-- 3. Orders
CREATE TABLE orders (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    symbol VARCHAR(20) NOT NULL, -- e.g. 'SOL/USDC'
    side VARCHAR(4) NOT NULL CHECK (side IN ('bid', 'ask')),
    price NUMERIC(30,10) NOT NULL CHECK (price >= 0),
    quantity NUMERIC(30,10) NOT NULL CHECK (quantity > 0),
    filled_quantity NUMERIC(30,10) DEFAULT 0 NOT NULL CHECK (filled_quantity >= 0),
    status VARCHAR(20) NOT NULL CHECK (status IN ('open', 'partially_filled', 'filled', 'cancelled')),
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_orders_user_id ON orders(user_id);
CREATE INDEX idx_orders_status ON orders(status);