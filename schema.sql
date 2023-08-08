CREATE TABLE positions (
    id SERIAL PRIMARY KEY,
    user_id VARCHAR,
    location geometry(POINT),
    timestamp TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE,
    CONSTRAINT unique_user_timestamp_positions UNIQUE (user_id, timestamp)
);

-- Create an index to support the unique constraint on user_id and timestamp
CREATE UNIQUE INDEX idx_unique_user_timestamp_positions ON positions (user_id, timestamp);

CREATE TABLE telemetry (
    id SERIAL PRIMARY KEY,
    user_id VARCHAR,
    battery_level INTEGER,
    voltage REAL,
    timestamp TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE,
    CONSTRAINT unique_user_timestamp_telemetry UNIQUE (user_id, timestamp)
);

-- Create an index to support the unique constraint on user_id and timestamp
CREATE UNIQUE INDEX idx_unique_user_timestamp_telemetry ON telemetry (user_id, timestamp);
