use crate::commands::add::AddError;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct General {
    pub selenium_browser: String,
    pub default_filetype: String,
    pub editor_command: String,
}

#[derive(Debug, Deserialize)]
pub struct Workspace {
    pub ongoing_dir: String,
    pub archive_dir: String,
    pub archive: bool,
}

#[derive(Debug, Deserialize)]
pub struct FileType {
    pub language: String,
    pub main: String,
    pub compile: Option<String>,
    pub run: String,
    pub after: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub general: General,
    pub workspace: Workspace,
    #[serde(rename = "filetype")]
    pub filetype: HashMap<String, FileType>,
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
        &self.general.default_filetype
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::config::{General, Workspace};

    use super::Config;

    #[test]
    fn default_extension_when_none() {
        let config = Config {
            general: General {
                selenium_browser: String::new(),
                default_filetype: String::new(),
                editor_command: String::new(),
            },
            workspace: Workspace {
                ongoing_dir: String::new(),
                archive_dir: String::new(),
                archive: false,
            },
            filetype: HashMap::new(),
        };
        assert_eq!(config.default_extension(), "");
    }

    #[test]
    fn default_extension_custom() {
        let config = Config {
            general: General {
                selenium_browser: String::new(),
                default_filetype: "rs".to_string(),
                editor_command: String::new(),
            },
            workspace: Workspace {
                ongoing_dir: String::new(),
                archive_dir: String::new(),
                archive: false,
            },
            filetype: HashMap::new(),
        };
        assert_eq!(config.default_extension(), "rs");
    }
}
