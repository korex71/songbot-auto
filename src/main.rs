use clap::{Parser, Subcommand};
use dialoguer::Select;
mod lavalink;
mod utils;
mod bot;

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

async fn start_services(bot_folder: &str, lavalink_folder: &str) {
    if let Err(e) = lavalink::start_lavalink(lavalink_folder) {
        panic!("Erro ao iniciar Lavalink: {}", e);
    }

    if !lavalink::check_lavalink_uptime().await {
        panic!("Erro ao verificar se o Lavalink está online.");
    }

    if let Err(e) = bot::start_bot(bot_folder) {
        panic!("Erro ao iniciar Bot: {}", e);
    }
}

async fn full_setup() {
    let (current_dir, bot_folder, lavalink_folder) = utils::get_paths();

    bot::clone_bot(&bot_folder);
    lavalink::get_lavalink(&lavalink_folder).await.unwrap();
    utils::copy_config_files(&current_dir, &bot_folder, &lavalink_folder);
    start_services(&bot_folder, &lavalink_folder).await;
}

#[tokio::main]
async fn main() {

    let cli = Cli::parse();
    let (current_dir, bot_folder, lavalink_folder) = utils::get_paths();

    match cli.command {
        Some(Commands::Setup) => {
            full_setup().await;
        }
        Some(Commands::Start) => {
            if !utils::bot_already_configured() {
                println!("[!] Erro: Bot não encontrado.");
                return;
            }

            if !utils::lavalink_already_downloaded() {
                println!("[!] Erro: Lavalink não encontrado.");
                return;
            }

            lavalink::check_lavalink_youtube_plugin_version(&lavalink_folder)
                .await
                .unwrap();
            utils::copy_config_files(&current_dir, &bot_folder, &lavalink_folder);
            start_services(&bot_folder, &lavalink_folder).await;
        }
        None => {
            let choice = Select::new()
                .with_prompt("Atualizar bot e Lavalink ou apenas iniciar?")
                .items(&["Atualizar", "Apenas iniciar"])
                .default(0)
                .interact()
                .unwrap();

            match choice {
                0 => full_setup().await,
                _ => {
                    if !utils::bot_already_configured() {
                        println!("[!] Erro: Bot não encontrado.");
                        return;
                    }

                    if !utils::lavalink_already_downloaded() {
                        println!("[!] Erro: Lavalink não encontrado.");
                        return;
                    }

                    lavalink::check_lavalink_youtube_plugin_version(&lavalink_folder)
                        .await
                        .unwrap();
                    utils::copy_config_files(&current_dir, &bot_folder, &lavalink_folder);
                    start_services(&bot_folder, &lavalink_folder).await;
                }
            }
        }
    }
}
