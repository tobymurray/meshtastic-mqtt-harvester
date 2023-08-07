CREATE TABLE positions (
    id SERIAL PRIMARY KEY,
    user_id VARCHAR,
    location geometry(POINT),
    timestamp TIMESTAMP WITH TIME ZONE
);

CREATE TABLE telemetry (
    id SERIAL PRIMARY KEY,
    user_id VARCHAR,
    battery_level INTEGER,
    voltage REAL,
    timestamp TIMESTAMP WITH TIME ZONE
);
