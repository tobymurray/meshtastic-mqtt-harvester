CREATE TABLE positions (
    id SERIAL PRIMARY KEY,
    user_id VARCHAR,
    location geometry(POINT),
    timestamp TIMESTAMP
);