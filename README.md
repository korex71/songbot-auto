# AutoBOT

## The idea is to make it easier to configure a bot. 

1. get a community-updated bot (lavamusic).
2. add your configuration.
3. run it alongside lavalink.

## Deps
`$ cargo install`

## Config
* .env [see lavamusic doc](https://github.com/appujet/lavamusic)
* application.yml [see lavalink doc](https://lavalink.dev/configuration/index.html#example-environment-variables)

## Requiriments
* [rustup](https://www.rust-lang.org/tools/install) 
* Git
* Node

## Build .exe
`$ cargo build --release --target=x86_64-pc-windows-msvc`
