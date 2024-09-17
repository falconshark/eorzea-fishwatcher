use chrono::{DateTime, Local, TimeZone};
use serde::Deserialize;
use reqwest::Error;


/**
 *
 * Weather API come from Eorzea Weather
 * https://eorzea-weather.info
 *
 */

 pub async fn get_current_weather(area: String) -> Result<(), Error>{
    let request_url = format!("https://eorzea-weather.info/api/zones/{}/forecast?locale=ja", area);
    let response = reqwest::get(&request_url).await?;
    let json_result = response.json().await?;
    println!("{:?}", json_result);
    Ok(())
 }
