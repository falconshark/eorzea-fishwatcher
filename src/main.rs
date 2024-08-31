extern crate dotenv;
mod database;
use dotenv::dotenv;
use teloxide::{prelude::*, utils::command::BotCommands};

#[tokio::main]
async fn main() {
    dotenv().ok(); // Using dot env to load environment value from file.
    pretty_env_logger::init();
    log::info!("Starting eorzea weather watching bot...");

    //Ensure all of the required table is in the database.
    let _ = database::init_database();

    let bot = Bot::from_env();
    Command::repl(bot, answer).await;
}

// Command List
#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "These commands are supported:")]
enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "Register chat ID to the database, create a new user for this bot.")]
    Register,
    #[command(description = "handle a username and an age.", parse_with = "split")]
    CurrentWeather { area: String },
}

async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Help => bot.send_message(msg.chat.id, Command::descriptions().to_string()).await?,
        Command::Register => {
            let chat_id = msg.chat.id.0;
            let _ = database::create_user(chat_id);
            bot.send_message(msg.chat.id, format!("Thank you! You are already being the user of this bot.")).await?
        },
        Command::CurrentWeather { area } => {
            bot.send_message(msg.chat.id, format!("Target area: @{area} ")).await?
        }
    };
    Ok(())
}