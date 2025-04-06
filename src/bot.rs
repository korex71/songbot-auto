use clap::{Parser, Subcommand};
use std::{
    path::Path,
    process::{Command, Stdio},
};

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Setup,
    Start,
}

const BOT_REPO: &str = "https://github.com/appujet/lavamusic";

pub fn clone_bot(bot_folder: &str) {
    if !Path::new(&bot_folder).exists()
        && !Path::new(format!("{}/.env", &bot_folder).as_str()).exists()
    {
        println!("[X] Clonando repositório do bot...");
        Command::new("git")
            .args(["clone", BOT_REPO])
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()
            .unwrap()
            .wait()
            .unwrap();

        println!("[X] Instalando dependências...");
        Command::new("cmd")
            .current_dir(bot_folder)
            .args(["/C", "npm install"])
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()
            .unwrap()
            .wait()
            .unwrap();

        println!("[X] Rodando migrações...");
        Command::new("cmd")
            .args(["/C", "npm", "run", "db:migrate"])
            .current_dir(bot_folder)
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()
            .unwrap()
            .wait()
            .unwrap();
    }
}

pub fn start_bot(bot_folder: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("[X] Iniciando Bot...");

    let comando = format!("cd /D {} && npm run build && npm run start", bot_folder);
    Command::new("cmd")
        .args(["/C", "start", "cmd", "/K", &comando])
        .spawn()?; // apenas dispara

    Ok(())
}
