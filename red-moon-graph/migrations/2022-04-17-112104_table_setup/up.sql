-- Your SQL goes here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE campaigns (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4()
);

CREATE TABLE "campaign_nodes" (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    campaign_id UUID  NOT NULL REFERENCES campaigns ON DELETE CASCADE ,
    title TEXT NOT NULL,
    description TEXT,
    body TEXT
)