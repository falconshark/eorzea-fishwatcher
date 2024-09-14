use chrono::{DateTime, Local, TimeZone};
use std::{collections::HashMap, fmt::format};
use std::fs::File;
use std::io::BufReader;

/**
 *
 * Weather API come from Eorzea Weather
 * https://eorzea-weather.info
 *
 */

 pub async fn get_current_weather(area: String){
    let target_url = format!("https://eorzea-weather.info/api/zones/{}/forecast?locale=ja", area);
 }
