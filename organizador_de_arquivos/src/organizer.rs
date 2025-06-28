use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use anyhow::{Context, Result};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(rename = "regras")]
    pub rules: HashMap<String, Vec<String>>,
}

pub fn load_config() -> Result<Config> {
    let config_str = fs::read_to_string("config.json")
        .context("Não foi possível ler o arquivo de configuração")?;
    let config: Config = toml::from_str(&config_str)
        .context("Erro ao deserializar o arquivo de configuração")?;
    Ok(config)
}