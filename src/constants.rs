use dirs;

/// Default configuration file.
pub const DEFAULT_CONFIG_FILE: &str = format!("{}{}", home_dir(), "config.toml");
