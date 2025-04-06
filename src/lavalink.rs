use std::{
    fs,
    process::Command,
    time::Duration,
};
use futures_util::StreamExt;
use indicatif::ProgressBar;
use reqwest::Client;
use serde::Deserialize;
use regex::Regex;

#[derive(Deserialize)]
struct GithubRelease {
    assets: Vec<Asset>,
    tag_name: String,   
}

#[derive(Deserialize)]
struct Asset {
    browser_download_url: String,
}

const REPO_OWNER: &str = "lavalink-devs";
const LAVALINK_REPO_NAME: &str = "Lavalink";
const LAVALINK_PLUGIN_REPO_NAME: &str = "youtube-source";

pub async fn get_lavalink(lavalink_folder: &str) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!("https://api.github.com/repos/{}/{}/releases/latest", REPO_OWNER, LAVALINK_REPO_NAME);
    let client = Client::new();
    let res = client.get(&url)
        .header("User-Agent", "bot-setup")
        .send()
        .await?;

    let release: GithubRelease = res.json().await?;
    let download_url = &release.assets[0].browser_download_url;

    let filename = download_url.split('/').last().unwrap();
    let filepath = format!("{}/{}", lavalink_folder, filename);

    println!("[X] Baixando Lavalink...");
    let mut file = fs::File::create(&filepath)?;
    let mut resp = client.get(download_url).send().await?.bytes_stream();

    let pb = ProgressBar::new_spinner();

    pb.set_message("Baixando...");

    pb.enable_steady_tick(Duration::from_millis(100));

    while let Some(chunk) = resp.next().await {
        use std::io::Write;
        file.write_all(&chunk?)?;
    }
    pb.finish_with_message("Download completo!");

    println!("[X] Download completo.");
    Ok(())
}

/// Verifica se o plugin do Youtube está na versão mais recente
/// e atualiza o arquivo application.yml com a nova versão
pub async fn check_lavalink_youtube_plugin_version(lavalink_folder: &str) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!("https://api.github.com/repos/{}/{}/releases/latest", REPO_OWNER, LAVALINK_PLUGIN_REPO_NAME);
    let client = Client::new();
    let res = client.get(&url)
        .header("User-Agent", "bot-setup")
        .send()
        .await?;

    let release: GithubRelease = res.json().await?;
    let tag_name = &release.tag_name;

    let application_yml_path = format!("{}/application.yml", lavalink_folder);
    let content = fs::read_to_string(&application_yml_path)?;

    // Regex to find the line with the dependency
    // and replace the version number with the new one
    let re = Regex::new(r#"(?m)(- dependency: "dev\.lavalink\.youtube:youtube-plugin:)([\d\.]+)(")"#).unwrap();
    
    let resultado = re.replace_all(&content, |caps: &regex::Captures| {
        format!("{}{}{}", &caps[1], tag_name, &caps[3])
    }).to_string();

    fs::write(&application_yml_path, &resultado)?;
    println!("[X] Atualizando versão do plugin Youtube...");
    println!("[X] Versão do plugin Youtube atualizada para: {}", tag_name);
    Ok(())
}

pub async fn check_lavalink_uptime() -> bool {
    let client = reqwest::Client::new();
    for i in 1..=10 {
        println!("[{}/10] Verificando se o Lavalink está online...", i);
        let res = client
            .get("http://localhost:2333/version")
            .header("Authorization", "youshallnotpass")
            .send()
            .await;

        if let Ok(r) = res {
            if r.status().is_success() {
                println!("[X] Lavalink online!");
                return true;
            }
        }

        tokio::time::sleep(Duration::from_secs(5)).await;
    }

    println!("[!] Erro: Lavalink não iniciou a tempo.");
    false
}

pub fn start_lavalink(lavalink_folder: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("[X] Iniciando Lavalink...");

    let comando = format!("cd /D {} && java -jar Lavalink.jar", lavalink_folder);
    Command::new("cmd")
        .args(["/C", "start", "cmd", "/K", &comando])
        .spawn()?; // apenas dispara

    Ok(())
}