use serde::Deserialize;
use std::fs;
use crate::commands::add::AddError;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub default_filetype: Option<String>,
}

impl Config {
    pub fn load() -> Result<Self, AddError> {
        let config_path = ".boj/config.toml";
        let content = fs::read_to_string(config_path)
            .map_err(|e| AddError::ConfigError(format!("Failed to read config file: {}", e)))?;
        let config: Config = toml::from_str(&content)
            .map_err(|e| AddError::ConfigError(format!("Failed to parse config file: {}", e)))?;
        Ok(config)
    }

    pub fn default_extension(&self) -> &str {
        self.default_filetype.as_deref().unwrap_or("py")
    }
}

#[cfg(test)]
mod tests {
    use super::Config;

    #[test]
    fn default_extension_when_none() {
        let config = Config { default_filetype: None };
        assert_eq!(config.default_extension(), "py");
    }

    #[test]
    fn default_extension_custom() {
        let config = Config { default_filetype: Some("rs".to_string()) };
        assert_eq!(config.default_extension(), "rs");
    }
}