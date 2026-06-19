-- Add up migration script here
CREATE TABLE
    users (
        id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
        email VARCHAR(255) NOT NULL UNIQUE,
        first_name VARCHAR(255),
        last_name VARCHAR(255),
        photo TEXT,
        nick_name VARCHAR(255) UNIQUE,
        permit_id UUID REFERENCES permits (id) ON DELETE SET NULL,
        created_at TIMESTAMPTZ NOT NULL DEFAULT NOW (),
        updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW (),
        deleted_at TIMESTAMPTZ
    );

CREATE INDEX idx_users_email ON users (email);

CREATE INDEX idx_users_nick_name ON users (nick_name);

CREATE INDEX idx_users_deleted_at ON users (deleted_at);