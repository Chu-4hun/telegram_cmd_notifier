pub mod bot;

use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::{
    fs::OpenOptions,
    io::{self, Read, Write},
    sync::Arc,
    vec,
};

use tokio::sync::Mutex;
use tracing::{debug, error, Level};
use tracing_subscriber::FmtSubscriber;

use crate::bot::{bot_task, SendMessage};

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
struct Config {
    bot_token: String,
    subscribers: Vec<i64>,
}

#[tokio::main]
async fn main() -> io::Result<()> {
    setup_logging().expect("logging failed");

    let dir = setup_project_dirs().expect("dirs setup failed");
    let config = init_config(&dir).expect("config failed");
    let bot = Arc::new(Mutex::new(teloxide::Bot::new(
        &config.bot_token.to_string(),
    )));
    let new_thread_bot = Arc::clone(&bot);
    let path = dir.as_os_str().to_str().unwrap_or_default().to_owned();

    println!("your config file is located here: {}", dir.display());

    tokio::task::spawn(async move {
        bot_task(new_thread_bot, path.clone()).await;
    });

    let mut input = String::new();
    let mut repeat: bool = true;
    while repeat {
        match io::stdin().read_line(&mut input) {
            Ok(n) => {
                if n > 0 {
                    debug!("{n} bytes read \n{input}\n");
                    let contens = tokio::fs::read_to_string(
                        &dir.as_os_str().to_str().unwrap_or_default().to_owned(),
                    )
                    .await
                    .unwrap();
                    let conf = serde_json::from_str::<Config>(&contens).unwrap();

                    for sub in conf.subscribers {
                        Arc::clone(&bot)
                            .lock()
                            .await
                            .clone()
                            .send_simple_message(sub, input.to_string())
                            .await;
                    }
                    input = "".into();
                } else {
                    repeat = false;
                }
            }

            Err(error) => error!("error: {error}"),
        }
    }
    Ok(())
}

fn setup_project_dirs() -> Result<std::path::PathBuf, Box<dyn std::error::Error>> {
    let proj_dirs = ProjectDirs::from("com", "Telegram_notifier", "TelegramNotifierApp")
        .ok_or("File system err")?;
    let mut dir = proj_dirs.preference_dir().to_path_buf();
    std::fs::create_dir_all(&dir)?;
    dir.push("config.json");
    Ok(dir)
}

fn setup_logging() -> Result<(), Box<dyn std::error::Error>> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::WARN)
        .finish();
    tracing::subscriber::set_global_default(subscriber).map_err(|e| e.into())
}

fn init_config(dir: &std::path::PathBuf) -> Option<Config> {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(&dir)
        .unwrap_or_else(|e| {
            error!("{:#?}", e);
            OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(&dir)
                .expect("Unable to open or create config file")
        });

    let mut contents = String::new();
    if let Ok(_) = file.read_to_string(&mut contents) {
        if let Ok(res) = serde_json::from_str::<Config>(&contents) {
            return Some(res);
        }
    } else {
        error!("Unable to read file");
    }

    if contents.is_empty() {
        let mut key = request_key();
        while key.is_empty() {
            print!("Provided string is empty");
            key = request_key();
        }

        let config = Config {
            bot_token: key.trim_end().to_owned(),
            subscribers: vec![],
        };

        let str_config = serde_json::to_string(&config).unwrap();
        if let Err(e) = file.write_all(str_config.as_bytes()) {
            error!("{:#}", e);
        }

        return Some(config);
    }

    None
}

fn request_key() -> String {
    let mut key_buffer = String::new();

    print!("enter your Telegram bot key:\n");
    io::stdin()
        .read_line(&mut key_buffer)
        .expect("unable to read input");

    return key_buffer;
}
