#![allow(dead_code)]
#![allow(unused_variables)]

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
    #[command(description = "Register chat ID to the database, create a new user for this bot. 
    Before using the notify function, please run it first.")]
    Register,
    #[command(description = "Return specific area weather.", parse_with = "split")]
    CurrentWeather { area_name: String },
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
            let target_version_list = version_list.as_object().unwrap();

            for (key, value) in target_version_list{
                target_version_list_output = format!("{}{}\n", target_version_list_output, key);
            }
            bot.send_message(msg.chat.id, target_version_list_output).await?
        },
        Command::Area { version }=> {
            let version_list_result = area::get_version_list();
            let version_list = version_list_result.unwrap();
            let mut target_area_list_output: String = "Area of this version: \n".to_string();
            
            //Read the array from the Json "Object"
            let target_area_list = version_list.get(version).unwrap();
            let target_area_list_array = target_area_list.as_object().unwrap();

            for (key, value) in target_area_list_array{
                target_area_list_output = format!("{}{}\n", target_area_list_output, key);
            }
            bot.send_message(msg.chat.id, target_area_list_output).await?
        },
        Command::Register => {
            let chat_id = msg.chat.id.0;
            let insert_result = database::create_user(chat_id);
            match insert_result {
                Ok(result) => { 
                    bot.send_message(msg.chat.id, format!("Thank you! You are already being the user of this bot.")).await?
                },
                Err(e) => {
                    bot.send_message(msg.chat.id, format!("Sorry! Some errors have occurred. Maybe you are already registed?")).await?
                },
            }
        },
        Command::CurrentWeather { area_name } => {
            let area_list_result = area::get_area_id_list();
            let area_list = area_list_result.unwrap();
            let area_name_output = area_name.clone(); //Clone for ingore value borrow problem.
            
            //Read the array from the Json "Object"
            let target_area_list = area_list.as_object().unwrap();
            
            let mut weather_result_output = format!("Area: {}", area_name_output);
            bot.send_message(msg.chat.id, weather_result_output).await?
        }
    };
    Ok(())
}