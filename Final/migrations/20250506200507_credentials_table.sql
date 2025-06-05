CREATE TABLE credentials (
    id UUID PRIMARY KEY,
    username VARCHAR NOT NULL,
    password VARCHAR NOT NULL,
    victim_ip VARCHAR,
    user_agent TEXT, 
    captured_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

--DROP TABLE credentials;
