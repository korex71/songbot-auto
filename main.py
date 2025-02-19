import json
import os
import shutil
import subprocess
import time

import requests

# Diretórios e comandos
CURRENT_DIR = os.getcwd()
BOT_REPO = "https://github.com/appujet/lavamusic"
BOT_FOLDER = os.path.join(CURRENT_DIR, "lavamusic")
LAVALINK_FOLDER = os.path.join(BOT_FOLDER, "lavalink")

# Comandos
CLONE_BOT_CMD = f"git clone {BOT_REPO}"
NPM_INSTALL_CMD = "npm install"
PRISMA_MIGRATE_CMD = "npm run db:migrate"

# Configuração
APPCONFIG_PATH = os.path.join(CURRENT_DIR, 'appconfig.json')

# GitHub API
REPO_OWNER = "lavalink-devs"
REPO_NAME = "Lavalink"
API_URL = f"https://api.github.com/repos/{REPO_OWNER}/{REPO_NAME}/releases/latest"


def get_lavalink(folder):
    try:
        response = requests.get(API_URL)
        response.raise_for_status()
        release_data = response.json()
        download_url = release_data["assets"][0]["browser_download_url"]
        local_filename = os.path.join(folder, download_url.split("/")[-1])

        print(f"# Lavalink::Downloading {local_filename}...")
        with requests.get(download_url, stream=True) as r, open(local_filename, "wb") as f:
            shutil.copyfileobj(r.raw, f)
        print("# Lavalink::Download complete.")
    except requests.exceptions.RequestException as e:
        print(f"# Lavalink::Error: {e}")

def clone_github_bot():
    if not os.path.isdir(BOT_FOLDER):
        print("[X] Cloning bot repository...")
        subprocess.run(CLONE_BOT_CMD, shell=True, check=True)
        print("[X] Installing dependencies...")
        subprocess.run(NPM_INSTALL_CMD, shell=True, cwd=BOT_FOLDER, check=True)
        os.makedirs(LAVALINK_FOLDER, exist_ok=True)
        print("[X] Bot setup complete.")


def copy_configuration_files():
    try:
        shutil.copy(os.path.join(CURRENT_DIR, ".env"), os.path.join(BOT_FOLDER, ".env"))
        shutil.copy(os.path.join(CURRENT_DIR, "application.yml"), os.path.join(LAVALINK_FOLDER, "application.yml"))
        print("[X] Configuration files copied.")
    except Exception as e:
        print(f"[Error] {e}")


def prisma_db_migration():
    print("[X] Running database migrations...")
    subprocess.run(PRISMA_MIGRATE_CMD, shell=True, cwd=BOT_FOLDER, check=True)


def check_lavalink_uptime(retries=10):
    url = "http://localhost:2333/version"
    headers = {"Authorization": "youshallnotpass"}
    for attempt in range(1, retries + 1):
        try:
            print(f"[{attempt}/{retries}] Checking Lavalink status...")
            if requests.get(url, headers=headers).status_code == 200:
                print("[X] Lavalink is online.")
                return True
        except requests.exceptions.RequestException:
            pass
        time.sleep(5)
    print("[X] Error: Lavalink did not start in time.")
    return False

def start_bot():
    print("[X] Starting Lavalink...")
    subprocess.run(f'start cmd /k "cd /d {LAVALINK_FOLDER} && java -jar Lavalink.jar"', shell=True)
    if not check_lavalink_uptime():
        raise RuntimeError("Lavalink did not start correctly.")

    print("[X] Starting bot...")
    subprocess.run(f'start cmd /k "cd /d {BOT_FOLDER} && npm run start"', shell=True)


def setup_dependencies():
    clone_github_bot()
    prisma_db_migration()
    get_lavalink(LAVALINK_FOLDER)
    copy_configuration_files()

def ask_initialize():
    choice = input("# Update bot and Lavalink or just start? [U(Update)/S(Start)]: ").strip().lower()
    if choice == "u":
        setup_dependencies()
        start_bot()
    else:
        copy_configuration_files()
        start_bot()


def main():
    if not os.path.isdir(BOT_FOLDER):
        setup_dependencies()
        start_bot()
    else:
        ask_initialize()


if __name__ == "__main__":
    try:
        main()
    except Exception as e:
        print(f"[Error] {e}")
        input("Press Enter to exit...")