use std::sync::Arc;

use teloxide::{prelude::*, types::Recipient};
use tokio::sync::Mutex;
use tracing::{debug, error};

use crate::Config;
use async_trait::async_trait;

#[async_trait]
pub trait SendMessage {
    async fn send_simple_message(&self, _chat_id: i64, _text: String) {}
}

async fn answer(bot: Bot, path: String, msg: Message) -> ResponseResult<()> {
    let contens = tokio::fs::read_to_string(&path).await.unwrap();
    let mut conf = serde_json::from_str::<Config>(&contens).unwrap();
    let b = !conf.subscribers.contains(&msg.chat.id.0);
    debug!("{}", path);
    if b {
        conf.subscribers.push(msg.chat.id.0);
        if let Err(err) =
            tokio::fs::write(&path, serde_json::to_string(&conf).unwrap().as_bytes()).await
        {
            error!("Error writing to file: {}", err);
        }
    }
    bot.send_dice(msg.chat.id).await?;
    Ok(())
}

pub async fn bot_task(bot: Arc<Mutex<Bot>>, path: String) {
    let _bot = Arc::clone(&bot).lock().await.clone();

    Dispatcher::builder(_bot, Update::filter_message().endpoint(answer))
        .dependencies(dptree::deps![path])
        .build()
        .dispatch()
        .await;
}

#[async_trait]
impl SendMessage for Bot {
    async fn send_simple_message(&self, _chat_id: i64, _text: String) {
        let _ = &self
            .send_message(Recipient::Id(ChatId(_chat_id)), _text)
            .await;
    }
}
