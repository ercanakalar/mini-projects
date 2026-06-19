-- Add migration script here
CREATE TABLE
    manuel_auth (
        id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
        email VARCHAR(255) NOT NULL UNIQUE,
        password VARCHAR(255) NOT NULL,
        user_id UUID NOT NULL UNIQUE,
        token_id UUID UNIQUE REFERENCES tokens (id) ON DELETE SET NULL,
        created_at TIMESTAMPTZ NOT NULL DEFAULT NOW (),
        updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW (),
        deleted_at TIMESTAMPTZ
    );

CREATE INDEX idx_manuel_auth_deleted_at ON manuel_auth (deleted_at);

CREATE TABLE
    google_auth (
        id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
        email VARCHAR(255) NOT NULL UNIQUE,
        user_id UUID NOT NULL UNIQUE,
        token_id UUID UNIQUE REFERENCES tokens (id) ON DELETE SET NULL,
        created_at TIMESTAMPTZ NOT NULL DEFAULT NOW (),
        updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW (),
        deleted_at TIMESTAMPTZ
    );

CREATE INDEX idx_google_auth_deleted_at ON google_auth (deleted_at);