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

/// Cria o arquivo .env.test com as variáveis de ambiente necessárias para o bot
pub fn create_json_env_file() -> bool {
    let current_dir_pathbuf = env::current_dir().unwrap();
    let current_dir = current_dir_pathbuf.to_str().unwrap();
    let env_path = format!("{}/.env", current_dir);

    if Path::new(&env_path).exists() {
        return false;
    }

    let json_env = r#"TOKEN="" # Your bot token.
CLIENT_ID="" # Your bot's client ID (If this value is left blank, bots cannot be invited using /invite or /about commands.).
DEFAULT_LANGUAGE="EnglishUS" # Default language for bot
PREFIX="!" # Your prefix.
OWNER_IDS=[""] # Your discord id (You can add multiple ids.).
GUILD_ID="" # Your server ID (If you want to use the bot for a single server).
TOPGG="" # Your Top.gg API key. Obtain this from https://top.gg
KEEP_ALIVE="false" # true for keep alive in https://replit.com
LOG_CHANNEL_ID="" # If you enter this, you will be able to receive the status of Lavalink nodes and guild join/leave logs through the corresponding channel.
LOG_COMMANDS_ID="" # The channel ID where command usage logs will be sent.
BOT_STATUS="online" # Your bot status (online, dnd, idle, invisible or offline).
BOT_ACTIVITY_TYPE=0 # Activity type is a number from 0 to 5. See more here: https://discord.com/developers/docs/topics/gateway-events#activity-object-activity-types
BOT_ACTIVITY=="Lavamusic" # Your bot activity.
DATABASE_URL="" # Your database url (If you want to use sqlite, then you can leave it blank.).
AUTO_NODE="false" # true for auto node. It is given from lavainfo-api (https://lavainfo-api.deno.dev).
SEARCH_ENGINE="YouTubeMusic" # Search engine to be used when playing the song. You can use: YouTube, YouTubeMusic, SoundCloud, Spotify, Apple, Deezer, Yandex and JioSaavn
GENIUS_API="" # Sign up and get your own api at (https://genius.com/) to fetch your lyrics (CLIENT TOKEN)
NODES=[{"id":"Local Node","host":"localhost","port":2333,"authorization":"youshallnotpass","retryAmount":5,"retryDelay":60000,"secure":"false"}]
"#;

    fs::write(env_path, json_env).unwrap();

    return true;
}

pub fn create_application_yml_file() -> bool {
    let current_dir_pathbuf = env::current_dir().unwrap();
    let current_dir = current_dir_pathbuf.to_str().unwrap();
    let application_yml_path = format!("{}/application.yml", current_dir);

    if Path::new(&application_yml_path).exists() {
        return false;
    }

    let json_env = r#"server: # REST and WS server
  port: 2333 # The port that the server listens on
  address: 0.0.0.0
  http2:
    enabled: false # Whether to enable HTTP/2 support
plugins:
  dunctebot:
    ttsLanguage: 'en-US' # language of the TTS engine
    sources:
      # true = source enabled, false = source disabled
      getyarn: true # www.getyarn.io
      clypit: true # www.clyp.it
      tts: true # speak:Words to speak
      pornhub: true # should be self-explanatory
      reddit: true # should be self-explanatory
      ocremix: true # www.ocremix.org
      tiktok: true # tiktok.com
      mixcloud: true # mixcloud.com
      soundgasm: true # soundgasm.net
      pixeldrain: true # pixeldrain.com
  jiosaavn:
    apiURL: "https://jiosaavn-plugin-api.deno.dev" # JioSaavn API URL
    playlistTrackLimit: 50 # The maximum number of tracks to return from given playlist (default 50 tracks)
    recommendationsTrackLimit: 10 # The maximum number of track to return from recommendations (default 10 tracks)
  lavasrc:
    providers: # Custom providers for track loading. This is the default
      # - "dzisrc:%ISRC%" # Deezer ISRC provider
      # - "dzsearch:%QUERY%" # Deezer search provider
      - "ytsearch:\"%ISRC%\"" # Will be ignored if track does not have an ISRC. See https://en.wikipedia.org/wiki/International_Standard_Recording_Code
      - "ytsearch:%QUERY%" # Will be used if track has no ISRC or no track could be found for the ISRC
      - "ytmsearch:%QUERY%" # Will be used if track has no ISRC or no track could be found for the ISRC
      #  you can add multiple other fallback sources here
    sources:
      spotify: false # Enable Spotify source
      applemusic: false # Enable Apple Music source
      deezer: false # Enable Deezer source
      yandexmusic: false # Enable Yandex Music source
      flowerytts: false # Enable Flowery TTS source
      youtube: true # Enable YouTube search source (https://github.com/topi314/LavaSearch)
      vkmusic: false # Enable Vk Music source
    lyrics-sources:
      spotify: false # Enable Spotify lyrics source
      deezer: false # Enable Deezer lyrics source
      youtube: false # Enable YouTube lyrics source
      yandexmusic: false # Enable Yandex Music lyrics source
      vkmusic: false # Enable Vk Music lyrics source
    spotify:
      clientId: ""
      clientSecret: ""
      # spDc: "your sp dc cookie" # the sp dc cookie used for accessing the spotify lyrics api
      countryCode: "US" # the country code you want to use for filtering the artists top tracks. See https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2
      playlistLoadLimit: 6 # The number of pages at 100 tracks each
      albumLoadLimit: 6 # The number of pages at 50 tracks each
      resolveArtistsInSearch: true # Whether to resolve artists in track search results (can be slow)
      localFiles: false # Enable local files support with Spotify playlists. Please note `uri` & `isrc` will be `null` & `identifier` will be `"local"`
    applemusic:
      countryCode: "US" # the country code you want to use for filtering the artists top tracks and language. See https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2
      mediaAPIToken: "your apple music api token" # apple music api token
      # or specify an apple music key
      keyID: "your key id"
      teamID: "your team id"
      musicKitKey: |
        -----BEGIN PRIVATE KEY-----
        your key
        -----END PRIVATE KEY-----      
      playlistLoadLimit: 6 # The number of pages at 300 tracks each
      albumLoadLimit: 6 # The number of pages at 300 tracks each
    deezer:
      masterDecryptionKey: "your master decryption key" # the master key used for decrypting the deezer tracks. (yes this is not here you need to get it from somewhere else)
      # arl: "your deezer arl" # the arl cookie used for accessing the deezer api this is optional but required for formats above MP3_128
      formats: [ "FLAC", "MP3_320", "MP3_256", "MP3_128", "MP3_64", "AAC_64" ] # the formats you want to use for the deezer tracks. "FLAC", "MP3_320", "MP3_256" & "AAC_64" are only available for premium users and require a valid arl
    yandexmusic:
      accessToken: "your access token" # the token used for accessing the yandex music api. See https://github.com/TopiSenpai/LavaSrc#yandex-music
      playlistLoadLimit: 1 # The number of pages at 100 tracks each
      albumLoadLimit: 1 # The number of pages at 50 tracks each
      artistLoadLimit: 1 # The number of pages at 10 tracks each
    flowerytts:
      voice: "default voice" # (case-sensitive) get default voice from here https://api.flowery.pw/v1/tts/voices
      translate: false # whether to translate the text to the native language of voice
      silence: 0 # the silence parameter is in milliseconds. Range is 0 to 10000. The default is 0.
      speed: 1.0 # the speed parameter is a float between 0.5 and 10. The default is 1.0. (0.5 is half speed, 2.0 is double speed, etc.)
      audioFormat: "mp3" # supported formats are: mp3, ogg_opus, ogg_vorbis, aac, wav, and flac. Default format is mp3
    youtube:
      countryCode: "US" # the country code you want to use for searching lyrics via ISRC. See https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2
    vkmusic:
      userToken: "your user token" # This token is needed for authorization in the api. Guide: https://github.com/topi314/LavaSrc#vk-music
      playlistLoadLimit: 1 # The number of pages at 50 tracks each
      artistLoadLimit: 1 # The number of pages at 10 tracks each
      recommendationsLoadLimit: 10 # Number of tracks
  youtube:
    enabled: true # Whether this source can be used.
    allowSearch: true # Whether "ytsearch:" and "ytmsearch:" can be used.
    allowDirectVideoIds: true # Whether just video IDs can match. If false, only complete URLs will be loaded.
    allowDirectPlaylistIds: true # Whether just playlist IDs can match. If false, only complete URLs will be loaded.
    # The clients to use for track loading. See below for a list of valid clients.
    # Clients are queried in the order they are given (so the first client is queried first and so on...)
    clients:
      - ANDROID_MUSIC
      - MUSIC
      - WEB
      - WEBEMBEDDED
      - TVHTML5EMBEDDED
      # This secure your lavalink is working and playing music if you used this two things below.
    #oauth:
     # enabled: true # IF YOU RUN YOUR LAVALINK CHECK YOUR CONSOLE AND CLICK THE GOOGLE.COM/DEVICES LINK AND THERE IS A CODE THAT YOU NEED TO PUT IN TO THE GOOGLE AND USE A BURNER ACCOUNT OR ALT ACCOUNT THERE IS A POSSIBLE CHANGE YOU CAN GET BANNED FROM GOOGLE OR YOUTUBE SO JUST USE A BURNER ACCOUNT IN CASE.
    #pot:
   #   token: "" # THIS REQUIRE INSTALLING https://github.com/iv-org/youtube-trusted-session-generator THIS IS A PYTHON FILE INSTALL THE REQUIREMENTS AND RUN PYTON INDEX FILE AND AFTER RUNNING YOU WILL RECIVE AND TOKEN AND VISTOR DATA AND PASTE IT BELOW.
   #   visitorData: ""
lavalink:
  plugins:
    - dependency: "com.github.appujet:jiosaavn-plugin:1.0.2"
      repository: "https://jitpack.io"
    - dependency: "com.dunctebot:skybot-lavalink-plugin:1.7.0"
      snapshot: false # set to true if you want to use snapshot builds
    - dependency: "com.github.topi314.lavasearch:lavasearch-plugin:1.0.0"
      snapshot: false # set to true if you want to use snapshot builds
    - dependency: "com.github.topi314.lavasrc:lavasrc-plugin:4.3.0"
      snapshot: false # set to true if you want to use snapshot builds
    - dependency: "com.github.topi314.sponsorblock:sponsorblock-plugin:3.0.1"
      snapshot: false # set to true if you want to use snapshot builds
    - dependency: "dev.lavalink.youtube:youtube-plugin:1.12.0"
      snapshot: false # set to true if you want to use snapshot builds
  pluginsDir: './plugins'
  server:
    password: "youshallnotpass"
    sources:
      # The default Youtube source is now deprecated and won't receive further updates. Please use https://github.com/lavalink-devs/youtube-source#plugin instead.
      youtube: false
      bandcamp: true
      soundcloud: true
      twitch: true
      vimeo: true
      mixer: true
      nico: true
      http: true # warning: keeping HTTP enabled without a proxy configured could expose your server's IP address.
      local: false
    filters: # All filters are enabled by default
      volume: true
      equalizer: true
      karaoke: true
      timescale: true
      tremolo: true
      vibrato: true
      distortion: true
      rotation: true
      channelMix: true
      lowPass: true
    bufferDurationMs: 400 # The duration of the NAS buffer. Higher values fare better against longer GC pauses. Duration <= 0 to disable JDA-NAS. Minimum of 40ms, lower values may introduce pauses.
    frameBufferDurationMs: 5000 # How many milliseconds of audio to keep buffered
    opusEncodingQuality: 10 # Opus encoder quality. Valid values range from 0 to 10, where 10 is best quality but is the most expensive on the CPU.
    resamplingQuality: MEDIUM # Quality of resampling operations. Valid values are LOW, MEDIUM and HIGH, where HIGH uses the most CPU.
    trackStuckThresholdMs: 10000 # The threshold for how long a track can be stuck. A track is stuck if does not return any audio data.
    useSeekGhosting: true # Seek ghosting is the effect where whilst a seek is in progress, the audio buffer is read from until empty, or until seek is ready.
    youtubePlaylistLoadLimit: 6 # Number of pages at 100 each
    playerUpdateInterval: 5 # How frequently to send player updates to clients, in seconds
    youtubeSearchEnabled: true
    soundcloudSearchEnabled: true
    gc-warnings: true
    #ratelimit:
      #ipBlocks: ["1.0.0.0/8", "..."] # list of ip blocks
      #excludedIps: ["...", "..."] # ips which should be explicit excluded from usage by lavalink
      #strategy: "RotateOnBan" # RotateOnBan | LoadBalance | NanoSwitch | RotatingNanoSwitch
      #searchTriggersFail: true # Whether a search 429 should trigger marking the ip as failing
      #retryLimit: -1 # -1 = use default lavaplayer value | 0 = infinity | >0 = retry will happen this numbers times
    #youtubeConfig: # Required for avoiding all age restrictions by YouTube, some restricted videos still can be played without.
      #email: "your account mail" # Email of Google account
      #password: "your account password" # Password of Google account
    #httpConfig: # Useful for blocking bad-actors from ip-grabbing your music node and attacking it, this way only the http proxy will be attacked
      #proxyHost: "localhost" # Hostname of the proxy, (ip or domain)
      #proxyPort: 3128 # Proxy port, 3128 is the default for squidProxy
      #proxyUser: "" # Optional user for basic authentication fields, leave blank if you don't use basic auth
      #proxyPassword: "" # Password for basic authentication

metrics:
  prometheus:
    enabled: false
    endpoint: /metrics

sentry:
  dsn: ""
  environment: ""
#  tags:
#    some_key: some_value
#    another_key: another_value

logging:
  file:
    path: ./logs/

  level:
    # Set this to DEBUG to enable more detailed logging. Please note that this will likely spam your console.
    root: INFO
    # Set this to DEBUG to enable more detailed logging from Lavalink.
    lavalink: INFO

  request:
    enabled: true
    includeClientInfo: true
    includeHeaders: false
    includeQueryString: true
    includePayload: true
    maxPayloadLength: 10000

  logback:
    rollingpolicy:
      max-file-size: 1GB
      max-history: 30
"#;

    fs::write(application_yml_path, json_env).unwrap();

    return true;
}