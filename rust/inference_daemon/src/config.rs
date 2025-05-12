use std::path::PathBuf;
use serde::Deserialize;
use thiserror::Error;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub model_path: PathBuf,
    pub context_window: usize,
    pub beam_width: usize,
    pub quantization: bool,
    pub socket_path: PathBuf,
    pub gpu_device: Option<usize>,
    pub log_level: String,
}

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Failed to read config file: {0}")]
    Io(#[from] std::io::Error),
    #[error("Failed to parse config: {0}")]
    Parse(#[from] toml::de::Error),
    #[error("Invalid config: {0}")]
    Validation(String),
}

impl Config {
    pub fn load_config() -> Result<Self, ConfigError> {
        let config_paths = [
            PathBuf::from("$XDG_CONFIG_HOME/ai_autocorrect/config.toml"),
            PathBuf::from("~/.config/ai_autocorrect/config.toml"),
            PathBuf::from("config.toml"),
        ];

        for path in config_paths {
            if path.exists() {
                let config_str = std::fs::read_to_string(&path)?;
                let config: Config = toml::from_str(&config_str)?;
                config.validate()?;
                return Ok(config);
            }
        }

        Err(ConfigError::Validation("No config file found".to_string()))
    }

    fn validate(&self) -> Result<(), ConfigError> {
        if !self.model_path.exists() {
            return Err(ConfigError::Validation(
                format!("Model path does not exist: {}", self.model_path.display())
            ));
        }

        if self.context_window == 0 {
            return Err(ConfigError::Validation(
                "Context window must be greater than 0".to_string()
            ));
        }

        Ok(())
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            model_path: PathBuf::from("model.onnx"),
            context_window: 128,
            beam_width: 1,
            quantization: false,
            socket_path: PathBuf::from("/tmp/ai_autocorrect.sock"),
            gpu_device: None,
            log_level: "info".to_string(),
        }
    }
}