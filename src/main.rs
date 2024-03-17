use teloxide::{prelude::*, repl, utils::command::BotCommands};
use tokio::time::{sleep, Duration};

mod db;
mod settings;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting command bot...");

    let bot = Bot::new(settings::BOT_ID);

    repl(bot.clone(), |bot: Bot, msg: Message| async move {
        let mut redis = db::Redis::new();
        redis.set_words();

        let txt = match msg.text() {
            Some(s) => s,
            None => "Error",
        };

        let admin = UserId(settings::ADMIN_ID);

        if msg.from().take().unwrap().id == admin {
            if txt.starts_with("/") {
                let command = Command::parse(txt, "").unwrap_or_else(|_| Command::default());
                match command {
                    Command::Set(word) => {
                        redis.set(word.as_str(), 1);
                        redis.write_word(&word);
                        let _ = bot.send_message(msg.chat.id, "Слово успешно добавленно").await;
                    },
                    Command::Help => {
                        let _ = bot.send_message(msg.chat.id, Command::descriptions().to_string()).await;
                    },
                    Command::Del(word) => {
                        match redis.keys() {
                            Ok(mut keys) => {
                                keys.retain(|s| s != &word);
                                let _ = redis.del(word.as_str());
                                redis.write_words(keys);

                                let _ = bot.send_message(msg.chat.id, "Словарь обнавлен").await;
                            },
                            Err(_) => {},
                        }
                    }
                    Command::Show => {
                        match redis.keys() {
                            Ok(keys) => {
                                let mut format_str = String::new();
                                for word in keys {
                                    if word != "" {
                                        format_str += &format!("{} \n", word).to_string();
                                    } else {
                                        continue;
                                    }
                                }
                                let _ = bot.send_message(msg.chat.id, format_str.as_str()).await;
                            },
                            Err(_) => {},
                        }
                    }
                    Command::Undeclorated => {
                        let _ = bot.send_message(msg.chat.id, "Данная команда не найдена").await;
                    }
                }
            } else {
                for word in txt.split_whitespace().rev() {
                    if redis.get(word) == Ok(1) {
                        let _ = bot.delete_message(msg.chat.id, msg.id).await;
                    }
                }
            };

        } else {
            let _ = bot.send_message(msg.chat.id, "Не тот пользователь").await;
        }

        Ok(())
    })
    .await;
}

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "Все поддерживаемые команды:")]
enum Command {
    #[command(description = "Все доступные команды")]
    Help,
    #[command(description = "Добавить слово {после пробела пишеться слово}")]
    Set(String),
    #[command(description = "Удалить слово {после пробела пишеться слово}")]
    Del(String),
    #[command(description = "Показать все слова в словаре")]
    Show,
    Undeclorated
}

impl Default for Command {
    fn default() -> Self {
        Command::Undeclorated
    }
}