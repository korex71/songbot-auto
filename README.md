# AutoBOT

## The idea is to make it easier to configure a bot. 

1. get a community-updated bot (lavamusic).
2. add your configuration.
3. run it alongside lavalink.

## Deps
`$ pip install -r requirements.txt`

## Config
* .env [see lavamusic doc](https://github.com/appujet/lavamusic)
* application.yml [see lavalink doc](https://lavalink.dev/configuration/index.html#example-environment-variables)

## Requiriments
* Python3
* Git
* Node

## Build .exe
`$ pip pyinstaller.exe --onefile --icon=icon.ico main.py`
