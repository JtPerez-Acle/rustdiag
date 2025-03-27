use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

/// Configuration for RustDiag
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    /// Default interface to use if not specified
    pub default_interface: Option<String>,
    
    /// Default port range for scanning
    pub default_port_range: String,
    
    /// Whether to enable geolocation by default
    pub enable_geolocation: bool,
    
    /// Path to store results
    pub results_path: Option<PathBuf>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            default_interface: None,
            default_port_range: String::from("1-1000"),
            enable_geolocation: false,
            results_path: None,
        }
    }
}

impl Config {
    /// Load configuration from a file
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        let config_str = fs::read_to_string(&path)
            .with_context(|| format!("Failed to read config file: {:?}", path.as_ref()))?;
        
        let config: Config = serde_json::from_str(&config_str)
            .with_context(|| "Failed to parse config file")?;
        
        Ok(config)
    }
    
    /// Save configuration to a file
    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let config_str = serde_json::to_string_pretty(self)
            .with_context(|| "Failed to serialize config")?;
        
        fs::write(&path, config_str)
            .with_context(|| format!("Failed to write config file: {:?}", path.as_ref()))?;
        
        Ok(())
    }
    
    /// Get default config path
    pub fn default_path() -> PathBuf {
        let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
        home.join(".config").join("rustdiag").join("config.json")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;
    
    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.default_port_range, "1-1000");
        assert_eq!(config.enable_geolocation, false);
    }
    
    #[test]
    fn test_load_save_config() {
        // Create a temporary file
        let mut temp_file = NamedTempFile::new().unwrap();
        
        // Create a config
        let config = Config {
            default_interface: Some("eth0".to_string()),
            default_port_range: "1-100".to_string(),
            enable_geolocation: true,
            results_path: Some(PathBuf::from("/tmp/rustdiag")),
        };
        
        // Serialize to JSON and write to the temp file
        let config_json = serde_json::to_string_pretty(&config).unwrap();
        write!(temp_file, "{}", config_json).unwrap();
        
        // Load the config from the temp file
        let loaded_config = Config::load(temp_file.path()).unwrap();
        
        // Verify the loaded config matches the original
        assert_eq!(loaded_config.default_interface, Some("eth0".to_string()));
        assert_eq!(loaded_config.default_port_range, "1-100");
        assert_eq!(loaded_config.enable_geolocation, true);
        assert_eq!(loaded_config.results_path, Some(PathBuf::from("/tmp/rustdiag")));
    }
}
