use serde::Deserialize;
use std::path::Path;

// ---------------------------------------------------------------------------
// Default value helpers (required by #[serde(default = "...")])
// ---------------------------------------------------------------------------

fn default_host() -> String {
    "127.0.0.1".to_string()
}

fn default_port() -> u16 {
    7823
}

fn default_poll_interval_seconds() -> u64 {
    2
}

fn default_idle_threshold_minutes() -> u64 {
    5
}

fn default_database_path() -> String {
    "tokiwa.db".to_string()
}

// ---------------------------------------------------------------------------
// Config structs
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    #[serde(default = "default_host")]
    pub host: String,

    #[serde(default = "default_port")]
    pub port: u16,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            host: default_host(),
            port: default_port(),
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct TrackingConfig {
    #[serde(default = "default_poll_interval_seconds")]
    pub poll_interval_seconds: u64,

    #[serde(default = "default_idle_threshold_minutes")]
    pub idle_threshold_minutes: u64,
}

impl Default for TrackingConfig {
    fn default() -> Self {
        Self {
            poll_interval_seconds: default_poll_interval_seconds(),
            idle_threshold_minutes: default_idle_threshold_minutes(),
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseConfig {
    #[serde(default = "default_database_path")]
    pub path: String,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            path: default_database_path(),
        }
    }
}

#[derive(Debug, Deserialize, Clone, Default)]
pub struct Config {
    #[serde(default)]
    pub server: ServerConfig,

    #[serde(default)]
    pub tracking: TrackingConfig,

    #[serde(default)]
    pub database: DatabaseConfig,
}

// ---------------------------------------------------------------------------
// Loader
// ---------------------------------------------------------------------------

impl Config {
    /// Load configuration from `config/config.toml` if it exists and is valid.
    ///
    /// Falls back to built-in defaults if:
    ///   - the file does not exist, or
    ///   - the file cannot be read, or
    ///   - the file fails to parse.
    ///
    /// Partial config files are also supported: any missing key falls back to
    /// its `#[serde(default)]` value, so callers get a fully-populated Config
    /// regardless of which keys are present in the file.
    pub fn load() -> Self {
        let config_path = Path::new("config/config.toml");

        if !config_path.exists() {
            tracing::warn!(
                path = %config_path.display(),
                "Config file not found — using built-in defaults"
            );
            return Config::default();
        }

        match std::fs::read_to_string(config_path) {
            Err(err) => {
                tracing::warn!(
                    path = %config_path.display(),
                    error = %err,
                    "Failed to read config file — using built-in defaults"
                );
                Config::default()
            }
            Ok(raw) => match toml::from_str::<Config>(&raw) {
                Ok(cfg) => {
                    tracing::info!(
                        path = %config_path.display(),
                        "Loaded configuration from file"
                    );
                    cfg
                }
                Err(err) => {
                    tracing::warn!(
                        path = %config_path.display(),
                        error = %err,
                        "Failed to parse config file — using built-in defaults"
                    );
                    Config::default()
                }
            },
        }
    }
}
