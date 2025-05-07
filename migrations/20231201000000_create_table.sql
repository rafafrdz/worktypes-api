CREATE TABLE WorkType (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    title VARCHAR(100) NOT NULL,
    description TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE WorkAttributeType (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    work_type_id UUID NOT NULL REFERENCES WorkType(id) ON DELETE CASCADE,
    name VARCHAR(100) NOT NULL,
    data_type VARCHAR(50) NOT NULL,
    is_required BOOLEAN NOT NULL DEFAULT FALSE,
    is_hidden BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS Company (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    cif_number TEXT UNIQUE,
    billing_address TEXT,
    postal_code INTEGER,
    city TEXT,
    province TEXT,
    industry TEXT,
    industry_sub_category TEXT,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL
)