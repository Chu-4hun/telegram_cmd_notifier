# Telegram cmd notifier

Motivation: I wanted to get notified in telegram when my server logs out errors

## How to install
### Using cargo
This requires [cargo to be installed](https://www.rust-lang.org/tools/install) on your system and added to the PATH

This clones this repo and installs it
```bash
cargo install --git https://github.com/Chu-4hun/telegram_cmd_notifier
```

### Using releases
#### Win
* get the latest [release]( https://github.com/Chu-4hun/telegram_cmd_notifier/releases/latest/download/telegram_cmd_notifier_x86_64-pc-windows-msvc.exe)
* rename file to the `telegram_cmd_notifier.exe`
* put it in folder included in PATH
#### Linux
* get the latest [release]( https://github.com/Chu-4hun/telegram_cmd_notifier/releases/latest/download/telegram_cmd_notifier_x86_64-unknown-linux-gnu)
* rename file to the `telegram_cmd_notifier`
* put it in folder included in PATH


## How to use

At first run you will need to enter your Telegram bot api key ([how to get one](https://core.telegram.org/bots/features#creating-a-new-bot))
```bash
>telegram_cmd_notifier
enter your Telegram bot key:
```
And after that program will save config.json in
| Linux                                         	| Win                                                                                          	| Mac                                                                                                 	|
|-----------------------------------------------	|---------------------------------------------------------------------------------------------  |-----------------------------------------------------------------------------------------------------	|
| $HOME/.config/TelegramNotifierApp/config.json 	| C:\Users\{UserName}\AppData\Roaming\Telegram_notifier\TelegramNotifierApp\config\config.json 	| /Users/{UserName}/Library/Application Support/com.Telegram_notifier.TelegramNotifierApp/config.json 	|

config.json will look like
```json
{"bot_token":"{YOUR_TOKEN}","subscribers":[]}
```
To subscribe to a bot, simply send something to the bot in Telegram

After config app will scan `stdin` and send it to all subscribers

### Example usage
```bash
docker logs rust_backend | grep ERROR | grep -vi 'TRACE'| grep -vi 'DEBUG' | docker-telegram-notifier
```
