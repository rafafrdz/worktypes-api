-- migrations/0001_create_company.sql
-- Crear tipo ENUM si no existe
DO $ $ BEGIN CREATE TYPE document_storage_type AS ENUM ('LOCAL', 'REMOTE');

EXCEPTION
WHEN duplicate_object THEN NULL;

END $ $;

-- Tabla Company
CREATE TABLE IF NOT EXISTS company (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    cif_number TEXT UNIQUE NOT NULL,
    billing_address TEXT NOT NULL,
    postal_code INTEGER NOT NULL,
    city TEXT NOT NULL,
    province TEXT NOT NULL,
    industry TEXT NOT NULL,
    industry_sub_category TEXT NOT NULL DEFAULT ''
);

-- Tabla CompanyConfig
CREATE TABLE IF NOT EXISTS company_config (
    id TEXT PRIMARY KEY,
    company_id TEXT NOT NULL REFERENCES company(id) ON DELETE CASCADE,
    key TEXT NOT NULL,
    label TEXT,
    visible BOOLEAN NOT NULL DEFAULT TRUE,
    UNIQUE(company_id, key)
);

-- Tabla Employee
CREATE TABLE IF NOT EXISTS employee (
    id TEXT PRIMARY KEY,
    email TEXT NOT NULL UNIQUE,
    role TEXT NOT NULL,
    company_id TEXT NOT NULL REFERENCES company(id) ON DELETE CASCADE,
    office_id TEXT
);

-- Tabla CertificationTemplate
CREATE TABLE IF NOT EXISTS certification_template (
    id SERIAL PRIMARY KEY,
    company_id TEXT NOT NULL REFERENCES company(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    description TEXT,
    logo TEXT NOT NULL
);

-- Tabla Report
CREATE TABLE IF NOT EXISTS report (
    id TEXT PRIMARY KEY,
    company_id TEXT NOT NULL REFERENCES company(id) ON DELETE CASCADE,
    office_id TEXT,
    date TIMESTAMPTZ NOT NULL DEFAULT now(),
    document_id TEXT NOT NULL,
    format TEXT NOT NULL,
    data_type TEXT NOT NULL,
    selected_tags TEXT []
);

-- Tabla Document
CREATE TABLE IF NOT EXISTS document (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL,
    company_id TEXT NOT NULL REFERENCES company(id) ON DELETE CASCADE,
    date TIMESTAMPTZ NOT NULL DEFAULT now(),
    url TEXT NOT NULL,
    original_filename TEXT NOT NULL,
    type document_storage_type NOT NULL DEFAULT 'LOCAL'
);

-- Tabla Objective
CREATE TABLE IF NOT EXISTS objective (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    utility_type TEXT NOT NULL,
    target FLOAT NOT NULL,
    target_date TEXT NOT NULL DEFAULT '2024-01-01',
    company_id TEXT NOT NULL REFERENCES company(id) ON DELETE CASCADE
);