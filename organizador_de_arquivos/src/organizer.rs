use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use anyhow::{Context, Result};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    #[serde(rename = "regras")]
    pub rules: HashMap<String, Vec<String>>,
}

pub fn load_config() -> Result<Config> {
    let config_str = fs::read_to_string("config.toml")
        .context("Não foi possível ler o arquivo 'config.toml'")?;
    let config: Config = toml::from_str(&config_str)
        .context("Formato inválido para o arquivo 'config.toml'")?;
    Ok(config)
}

pub fn run(config: &Config, directory: &Path, dry_run: bool, verbose: bool) -> Result<()> {
    let extension_map = build_extension_map(&config.rules);
    if verbose {
        println!("Regras carregadas: {} categorias.", config.rules.len());
    }

    let entries = fs::read_dir(directory)
        .with_context(|| format!("Falha ao ler o diretório '{}'", directory.display()))?;

    for entry in entries {
        let entry = entry.with_context(|| "Falha ao ler uma entrada do diretório")?;
        let path = entry.path();

        if path.is_file() {
            if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
                if let Some(target_folder_name) = extension_map.get(ext) {
                    process_file(&path, target_folder_name, directory, dry_run, verbose)?;
                }
            }
        }
    }
    Ok(())
}

fn build_extension_map(rules: &HashMap<String, Vec<String>>) -> HashMap<String, String> {
    let mut map = HashMap::new();
    for (folder, extensions) in rules {
        for ext in extensions {
            map.insert(ext.clone(), folder.clone());
        }
    }
    map
}

fn process_file(
    path: &PathBuf,
    target_folder_name: &str,
    base_dir: &Path,
    dry_run: bool,
    verbose: bool,
) -> Result<()> {
    let target_dir = base_dir.join(target_folder_name);

    if !target_dir.exists() {
        if dry_run {
            println!("[SIMULAÇÃO] Criaria o diretório: {}", target_dir.display());
        } else {
            if verbose {
                println!("Criando diretório: {}", target_dir.display());
            }
            fs::create_dir(&target_dir)
                .with_context(|| format!("Falha ao criar o diretório '{}'", target_dir.display()))?;
        }
    }
    
    let file_name = path.file_name().unwrap_or_default();
    let new_path = target_dir.join(file_name);

    if dry_run {
        println!("[SIMULAÇÃO] Moveria '{}' -> '{}'", path.display(), new_path.display());
    } else {
        println!("Movendo '{}' -> '{}'", path.display(), new_path.display());
        fs::rename(path, &new_path)
            .with_context(|| format!("Falha ao mover o arquivo '{}'", path.display()))?;
    }
    
    Ok(())
}