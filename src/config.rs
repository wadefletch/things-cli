use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Config {
    pub auth_token: Option<String>,
}

fn config_path() -> Result<PathBuf> {
    let config_dir = dirs::config_dir()
        .context("Could not determine config directory")?
        .join("things-cli");
    Ok(config_dir.join("config.toml"))
}

impl Config {
    pub fn load() -> Result<Self> {
        let path = config_path()?;
        if !path.exists() {
            return Ok(Self::default());
        }
        let contents = fs::read_to_string(&path)
            .with_context(|| format!("Failed to read config at {}", path.display()))?;
        let config: Self = toml::from_str(&contents)?;
        Ok(config)
    }

    pub fn save(&self) -> Result<()> {
        let path = config_path()?;
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let contents = toml::to_string_pretty(self)?;
        fs::write(&path, &contents)?;

        // Set file permissions to 0600
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let perms = fs::Permissions::from_mode(0o600);
            fs::set_permissions(&path, perms)?;
        }

        Ok(())
    }

    pub fn set_token(&mut self, token: String) -> Result<()> {
        self.auth_token = Some(token);
        self.save()
    }

    pub fn clear_token(&mut self) -> Result<()> {
        self.auth_token = None;
        self.save()
    }

    pub fn masked_token(&self) -> Option<String> {
        self.auth_token.as_ref().map(|t| {
            if t.len() <= 4 {
                "****".to_owned()
            } else {
                format!("{}****", &t[..4])
            }
        })
    }
}
