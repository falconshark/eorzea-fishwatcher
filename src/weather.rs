use chrono::{DateTime, Local, TimeZone};
use serde::{Deserialize, Serialize};
use reqwest::Error;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ApiRreust {
    real_time: String,
    eorzea_time: String,
    weather_info: WeatherInfo,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct WeatherInfo {
    current_weather: String,
    previous_weather: String,
    next_weather: String,
}

/**
 *
 * Weather API Github Repo:
 * https://github.com/falconshark/ffxiv-weather-api
 *
 */

 pub async fn get_current_weather(area: String) -> Result<(), Error>{
    let request_url = format!("https://weather-ffxiv.sardo.work?area={}", area);
    let response = reqwest::get(&request_url).await?;
    let weather_info_json: String = response.text().await?;
    let weather_info: Result<ApiRreust, serde_json::Error> = serde_json::from_str(weather_info_json.as_str());
    println!("{:?}", weather_info);
    Ok(())
 }
