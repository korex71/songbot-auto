use std::{
    env,
    path::Path,
    fs
};
use fs_extra::file::copy;

pub fn bot_already_configured() -> bool {
    let current_dir = env::current_dir().unwrap();
    let bot_folder = current_dir.join("lavamusic");
    let lavalink_folder = bot_folder.join("lavalink");


    if Path::new(&bot_folder).exists() && Path::new(&lavalink_folder).exists() {
        return true;
    }
    false
}

pub fn lavalink_already_downloaded() -> bool {
    let current_dir = env::current_dir().unwrap();
    let bot_folder = current_dir.join("lavamusic");
    let lavalink_folder = bot_folder.join("lavalink");

    if Path::new(&lavalink_folder).exists() && Path::new(&lavalink_folder.join("Lavalink.jar")).exists() {
        return true;
    }
    false
}

pub fn copy_config_files(current_dir: &str, bot_folder: &str, lavalink_folder: &str) {
    fs::create_dir_all(lavalink_folder).unwrap();

    let _ = copy(format!("{}/.env", current_dir), format!("{}/.env", bot_folder), &fs_extra::file::CopyOptions {
        overwrite: true,
        ..Default::default()
    });
    
    // overwrite the application.yml file
    let _ = copy(
        format!("{}/application.yml", current_dir),
        format!("{}/application.yml", lavalink_folder),
        &fs_extra::file::CopyOptions {
            overwrite: true,
            ..Default::default()
        },
    );

    println!("[X] Arquivos de configuração copiados.");
}

pub fn get_paths() -> (String, String, String) {
    let current_dir = env::current_dir().unwrap();
    let bot_folder = current_dir.join("lavamusic");
    let lavalink_folder = bot_folder.join("lavalink");

    (
        current_dir.to_string_lossy().to_string(),
        bot_folder.to_string_lossy().to_string(),
        lavalink_folder.to_string_lossy().to_string(),
    )
}
