use chrono::{DateTime, Local, TimeZone};
use serde::Deserialize;
use reqwest::Error;

/**
 *
 * Weather API come from Eorzea Weather
 * https://eorzea-weather.info
 *
 */

 pub async fn get_current_weather(area: String){
    let target_url = format!("https://eorzea-weather.info/api/zones/{}/forecast?locale=ja", area);
 }
