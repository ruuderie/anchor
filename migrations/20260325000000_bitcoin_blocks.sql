CREATE TABLE IF NOT EXISTS bitcoin_blocks (
    id TEXT PRIMARY KEY,
    height BIGINT NOT NULL UNIQUE,
    version BIGINT,
    timestamp BIGINT NOT NULL,
    tx_count INT,
    size INT,
    weight INT,
    merkle_root TEXT,
    previousblockhash TEXT,
    mediantime BIGINT,
    nonce BIGINT,
    bits BIGINT,
    difficulty DOUBLE PRECISION,
    fetched_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
