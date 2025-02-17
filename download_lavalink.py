import os

import requests


def get_latest_lavalink_release():
    """Fetches the latest Lavalink release information."""
    repo_owner = "lavalink-devs"
    repo_name = "Lavalink"
    api_url = f"https://api.github.com/repos/{repo_owner}/{repo_name}/releases/latest"
    
    response = requests.get(api_url)
    response.raise_for_status()
    release_data = response.json()
    
    return release_data["tag_name"], release_data["assets"][0]["browser_download_url"]

def download_file(url, destination):
    """Downloads a file from a given URL to a specified destination."""
    with requests.get(url, stream=True) as response:
        response.raise_for_status()
        with open(destination, "wb") as file:
            for chunk in response.iter_content(chunk_size=8192):
                file.write(chunk)

def get_lavalink(folder):
    """Downloads the latest version of Lavalink to the specified folder."""
    try:
        latest_version, download_url = get_latest_lavalink_release()
        print(f"# Lavalink song server::Latest Version: {latest_version}")
        print(f"# Lavalink song server::Download URL: {download_url}")
        
        local_filename = os.path.join(folder, os.path.basename(download_url))
        download_file(download_url, local_filename)
        print("# Lavalink song server::Downloaded")
    except requests.exceptions.RequestException as e:
        print(f"# Lavalink song server::Error: {e}")
