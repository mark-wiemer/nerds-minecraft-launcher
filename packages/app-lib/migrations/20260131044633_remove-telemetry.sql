-- Remove telemetry column from settings table
-- SQLite doesn't support DROP COLUMN directly on tables with constraints,
-- so we need to recreate the table without the telemetry column

-- This migration removes the telemetry field as the app no longer collects any telemetry data
ALTER TABLE settings DROP COLUMN telemetry;
