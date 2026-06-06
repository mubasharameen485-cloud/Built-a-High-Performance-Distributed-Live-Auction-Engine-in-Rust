-- Add migration script here
-- 1. Items Table
CREATE TABLE IF NOT EXISTS auction_items (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    description TEXT NOT NULL,
    current_price DOUBLE PRECISION NOT NULL,
    image_url TEXT, -- Image URL yahin add kar diya
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- 2. Bids Table
CREATE TABLE IF NOT EXISTS bids (
    id SERIAL PRIMARY KEY,
    item_id INT NOT NULL REFERENCES auction_items(id),
    user_id INT NOT NULL,
    bid_amount DOUBLE PRECISION NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- 3. Live Chats Table
CREATE TABLE IF NOT EXISTS live_chats (
    id SERIAL PRIMARY KEY,
    item_id INT NOT NULL REFERENCES auction_items(id),
    sender VARCHAR(255) NOT NULL,
    content TEXT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);