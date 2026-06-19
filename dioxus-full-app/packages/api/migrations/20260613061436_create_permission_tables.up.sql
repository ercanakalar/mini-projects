-- Add migration script here
CREATE TABLE
    permits (
        id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
        name TEXT NOT NULL UNIQUE,
        description TEXT,
        created_at TIMESTAMPTZ NOT NULL DEFAULT NOW (),
        updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW (),
        deleted_at TIMESTAMPTZ
    );

CREATE INDEX idx_permits_deleted_at ON permits (deleted_at);

CREATE TABLE
    permissions (
        id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
        name TEXT NOT NULL UNIQUE,
        created_at TIMESTAMPTZ NOT NULL DEFAULT NOW (),
        updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW (),
        deleted_at TIMESTAMPTZ
    );

CREATE INDEX idx_permissions_deleted_at ON permissions (deleted_at);

-- Junction tables for many-to-many
CREATE TABLE
    permit_permissions (
        permit_id UUID REFERENCES permits (id) ON DELETE CASCADE,
        permission_id UUID REFERENCES permissions (id) ON DELETE CASCADE,
        PRIMARY KEY (permit_id, permission_id)
    );
