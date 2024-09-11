extern crate dotenv;
use dotenv::dotenv;
use teloxide::{prelude::*, utils::command::BotCommands};

mod database;
mod weather;
mod area;

#[tokio::main]
async fn main() {
    dotenv().ok(); // Using dot env to load environment value from file.
    pretty_env_logger::init();
    log::info!("Starting Eorzea weather watching bot...");

    //Ensure all the required table is in the database.
    let _ = database::init_database();

    let bot = Bot::from_env();
    Command::repl(bot, answer).await;
}

// Command List
#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "These commands are supported:")]
enum Command {
    #[command(description = "Display this text.")]
    Help,

    #[command(description = "Display version list of FFXIV.")]
    Version,
    #[command(description = "Display area list of specific version.", parse_with = "split")]
    Area { version: String },
    #[command(description = "Register chat ID to the database, create a new user for this bot.")]
    Register,
    #[command(description = "Return specific area weather.", parse_with = "split")]
    CurrentWeather { area: String },
}

async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Help => bot.send_message(msg.chat.id, Command::descriptions().to_string()).await?,
        Command::Version => {
            //Get version list from the Json file.
            let version_list_result = area::get_version_list();
            let version_list = version_list_result.unwrap();

            let mut target_version_list_output: String = "List of currently available versions: \n".to_string();

            //Read the array from the Json "Object"
            let target_version_list = version_list.get("area").unwrap();
            let target_version_list_array = target_version_list.as_array().unwrap();

            for version in target_version_list_array{
                target_version_list_output = format!("{}{}\n", target_version_list_output, version.as_str().expect("Value is a str"));
            } 
            bot.send_message(msg.chat.id, target_version_list_output).await?
        },
        Command::Area { version }=> {
            bot.send_message(msg.chat.id, format!("Thank you! You are already being the user of this bot.")).await?
        },
        Command::Register => {
            let chat_id = msg.chat.id.0;
            let _ = database::create_user(chat_id);
            bot.send_message(msg.chat.id, format!("Thank you! You are already being the user of this bot.")).await?
        },
        Command::CurrentWeather { area } => {
            bot.send_message(msg.chat.id, format!("Area: {area} ")).await?
        }
    };
    Ok(())
}