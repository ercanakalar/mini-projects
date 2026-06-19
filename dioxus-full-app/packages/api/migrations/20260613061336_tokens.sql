-- Add migration script here
CREATE TABLE
    tokens (
        id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
        user_id UUID NOT NULL UNIQUE,
        refresh_token TEXT,
        access_token TEXT,
        reset_token TEXT UNIQUE,
        password_reset_token_expiry TIMESTAMPTZ,
        created_at TIMESTAMPTZ NOT NULL DEFAULT NOW (),
        updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW (),
        deleted_at TIMESTAMPTZ
    );

CREATE INDEX idx_tokens_reset_token ON tokens (reset_token);

CREATE INDEX idx_tokens_deleted_at ON tokens (deleted_at);