-- Remove telemetry column from settings table
-- This migration removes the telemetry field as the app no longer collects any telemetry data
-- Using CREATE-COPY-DROP pattern for broader SQLite compatibility

-- Create new table without telemetry column
CREATE TABLE settings_new (
    id INTEGER NOT NULL CHECK (id = 0),

    max_concurrent_downloads INTEGER NOT NULL DEFAULT 10,
    max_concurrent_writes INTEGER NOT NULL DEFAULT 10,

    theme TEXT NOT NULL DEFAULT 'dark',
    default_page TEXT NOT NULL DEFAULT 'home',
    collapsed_navigation INTEGER NOT NULL DEFAULT TRUE,
    advanced_rendering INTEGER NOT NULL DEFAULT TRUE,
    native_decorations INTEGER NOT NULL DEFAULT FALSE,

    discord_rpc INTEGER NOT NULL DEFAULT TRUE,
    developer_mode INTEGER NOT NULL DEFAULT FALSE,

    onboarded INTEGER NOT NULL DEFAULT FALSE,

    extra_launch_args JSONB NOT NULL,
    custom_env_vars JSONB NOT NULL,
    mc_memory_max INTEGER NOT NULL DEFAULT 2048,
    mc_force_fullscreen INTEGER NOT NULL DEFAULT FALSE,
    mc_game_resolution_x INTEGER NOT NULL DEFAULT 854,
    mc_game_resolution_y INTEGER NOT NULL DEFAULT 480,

    hide_on_process_start INTEGER NOT NULL DEFAULT FALSE,

    hook_pre_launch TEXT NULL,
    hook_wrapper TEXT NULL,
    hook_post_exit TEXT NULL,

    custom_dir TEXT NULL,
    prev_custom_dir TEXT NULL,
    migrated INTEGER NOT NULL DEFAULT FALSE,

    personalized_ads INTEGER NOT NULL DEFAULT TRUE,
    toggle_sidebar INTEGER NOT NULL DEFAULT FALSE,
    feature_flags JSONB NOT NULL DEFAULT '{}',

    PRIMARY KEY (id)
);

-- Copy data from old table to new table (excluding telemetry)
INSERT INTO settings_new (
    id,
    max_concurrent_downloads,
    max_concurrent_writes,
    theme,
    default_page,
    collapsed_navigation,
    advanced_rendering,
    native_decorations,
    discord_rpc,
    developer_mode,
    onboarded,
    extra_launch_args,
    custom_env_vars,
    mc_memory_max,
    mc_force_fullscreen,
    mc_game_resolution_x,
    mc_game_resolution_y,
    hide_on_process_start,
    hook_pre_launch,
    hook_wrapper,
    hook_post_exit,
    custom_dir,
    prev_custom_dir,
    migrated,
    personalized_ads,
    toggle_sidebar,
    feature_flags
)
SELECT
    id,
    max_concurrent_downloads,
    max_concurrent_writes,
    theme,
    default_page,
    collapsed_navigation,
    advanced_rendering,
    native_decorations,
    discord_rpc,
    developer_mode,
    onboarded,
    extra_launch_args,
    custom_env_vars,
    mc_memory_max,
    mc_force_fullscreen,
    mc_game_resolution_x,
    mc_game_resolution_y,
    hide_on_process_start,
    hook_pre_launch,
    hook_wrapper,
    hook_post_exit,
    custom_dir,
    prev_custom_dir,
    migrated,
    personalized_ads,
    toggle_sidebar,
    feature_flags
FROM settings;

-- Drop old table
DROP TABLE settings;

-- Rename new table to original name
ALTER TABLE settings_new RENAME TO settings;
