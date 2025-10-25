/**
 * Migration: 20251005212921_abnana_migration.sql
 * Description: Create experiments, treatments, and overrides tables with initial data
 */
PRAGMA foreign_keys = ON;

CREATE TABLE
    IF NOT EXISTS experiments (
        experiment_id INTEGER PRIMARY KEY AUTOINCREMENT,
        name TEXT NOT NULL,
        created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
        updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
        is_active BOOLEAN DEFAULT 1
    );

CREATE INDEX idx_experiments_name ON experiments (name);

CREATE TABLE
    IF NOT EXISTS treatments (
        experiment_id INTEGER NOT NULL,
        user_id TEXT NOT NULL,
        treatment_id TEXT NOT NULL,
        FOREIGN KEY (experiment_id) REFERENCES experiments (experiment_id) ON DELETE CASCADE,
        PRIMARY KEY (experiment_id, user_id)
    );

CREATE INDEX idx_treatments_experiment_user ON treatments (experiment_id, user_id);

CREATE TABLE
    IF NOT EXISTS overrides (
        experiment_id INTEGER NOT NULL,
        user_id TEXT NOT NULL,
        treatment_id TEXT NOT NULL,
        FOREIGN KEY (experiment_id) REFERENCES experiments (experiment_id) ON DELETE CASCADE,
        PRIMARY KEY (experiment_id, user_id)
    );

CREATE INDEX idx_overrides_experiment_user ON overrides (experiment_id, user_id);

CREATE INDEX idx_user_overrides ON overrides (user_id);

INSERT INTO
    experiments (name, is_active)
VALUES
    ('New Feature Test', 1);

INSERT INTO
    treatments (experiment_id, user_id, treatment_id)
VALUES
    (1, 'user_123', 'T1');

INSERT INTO
    treatments (experiment_id, user_id, treatment_id)
VALUES
    (1, 'user_456', 'C');

INSERT INTO
    overrides (experiment_id, user_id, treatment_id)
VALUES
    (1, 'user_456', 'T1');