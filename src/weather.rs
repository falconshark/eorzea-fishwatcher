use chrono::{DateTime, Local, TimeZone};
/**
 *
 * Weather calculating algorithm borrowed from
 * https://github.com/Rogueadyn/SaintCoinach/blob/master/SaintCoinach/Xiv/WeatherRate.cs
 *
 * and code is just rewrited from
 * https://github.com/Kikuflare/FFXIV-Weather-Forecast/blob/master/scripts/weather.js
 */

/**
 * Calulates the weather value given a timestamp
 */
fn calculate_weather_value(timestamp: u64) -> u64 {
    let eorzea_hour = 175 * 1000; // number of real life milliseconds in an Eorzean hour
    let eorzea_8_hour = 8 * 175 * 1000; // number of real life milliseconds in 8 Eorzean hours
    let eorzean_hours_from_epoch = timestamp / eorzea_hour;
    let eorzean_days_from_epoch = timestamp / eorzea_8_hour;

    let increment = get_increment(eorzean_hours_from_epoch);
    let step1 = (eorzean_days_from_epoch << 32) >> 0;
    let step2 = step1 * 100 + increment;
    let step3 = ((step2 << 11) ^ step2) >> 0;
    let step4 = ((step3 >> 8) ^ step3) >> 0;
    return step4 % 100;
}

// 00:00~07:59 is 8
// 08:00~15:59 is 16
// 16:00~23:59 is 0
fn get_increment(timestamp: u64) -> u64 {
    return (timestamp + 8 - (timestamp % 8)) % 24;
}

/**
 * Returns the date corresponding to the nearest 00:00, 08:00 or 16:00 Eorzea Time
 *
 */
fn convert_to_nearest_real_interval_start(time: u64) -> DateTime<Local> {
    let eorzea_8_hour = 8 * 175 * 1000;
    let result = time - time % eorzea_8_hour;
    let result_i64 = result as i64;
    let date_time: DateTime<Local> = Local.timestamp(result_i64, 0);
    return date_time;
}

/**
 * Calculates the nearest eorzea time interval start corresponding to a unix timestamp
 *
 * Returns one of 00:00, 08:00, and 16:00
 *
 * @param {number} time A unix timestamp in milliseconds
 */
fn convert_to_nearest_eorzean_interval_start(timestamp: u64) -> String{
    let eorzea_hour = 175 * 1000;
    let eorzean_hours_from_epoch = timestamp / eorzea_hour;
    let eorzea_time_hour = (eorzean_hours_from_epoch - (eorzean_hours_from_epoch % 8)) % 24;
    return format!("{}:00", eorzea_time_hour);
}
