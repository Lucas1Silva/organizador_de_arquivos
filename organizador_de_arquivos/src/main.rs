mod organizer;

use std::path::PathBuf;
use anyhow::Result;
use clap::Parser;

/// Uma CLI para organizar arquivos em um diretório com base em regras de um config.toml.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// O caminho do diretório a ser organizado
    #[clap(short, long)]
    diretorio: PathBuf,

    /// Executa em modo de simulação, sem mover arquivos
    #[clap(long)]
    dry_run: bool,

    /// Exibe mais informações sobre as operações
    #[clap(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();
    
    if !args.diretorio.is_dir() {
        // Usando anyhow::bail! para retornar um erro de forma fácil
        anyhow::bail!("O caminho '{}' não é um diretório válido.", args.diretorio.display());
    }

    println!("Carregando configuração...");
    let config = organizer::load_config()?;

    if args.dry_run {
        println!("\n--- EXECUTANDO EM MODO DE SIMULAÇÃO (DRY RUN) ---\n");
    }

    organizer::run(&config, &args.diretorio, args.dry_run, args.verbose)?;

    println!("\nOrganização concluída!");
    Ok(())
}