use chrono::{DateTime, FixedOffset, Local, TimeZone, Utc};
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
fn calculateWeatherValue(timestamp: u64) {
    let EORZEA_HOUR = 175 * 1000; // number of real life milliseconds in an Eorzean hour
    let EORZEA_8_HOUR = 8 * 175 * 1000; // number of real life milliseconds in 8 Eorzean hours
    let EORZEA_DAY = 24 * 175 * 1000; // number of real life milliseconds in an Eorzean day
    let eorzeanHoursFromEpoch = timestamp / EORZEA_HOUR;
    let eorzeanDaysFromEpoch = timestamp / EORZEA_DAY;

    let increment = getIncrement(eorzeanHoursFromEpoch);
    let step1 = (eorzeanDaysFromEpoch << 32) >> 0;
    let step2 = step1 * 100 + increment;
    let step3 = ((step2 << 11) ^ step2) >> 0;
    let step4 = ((step3 >> 8) ^ step3) >> 0;
    return step4 % 100;
}

// 00:00~07:59 is 8
// 08:00~15:59 is 16
// 16:00~23:59 is 0
fn getIncrement(timestamp: u64) {
    return (timestamp + 8 - (timestamp % 8)) % 24;
}

/**
 * Returns the date corresponding to the nearest 00:00, 08:00 or 16:00 Eorzea Time
 *
 */
fn convertToNearestRealIntervalStart(timestamp: u64) {
    let result = time - time % EORZEA_8_HOUR;
    let dateTime: DateTime<Local> = Local.timestamp(result, 0);
    return dateTime;
}

/**
 * Calculates the nearest eorzea time interval start corresponding to a unix timestamp
 *
 * Returns one of 00:00, 08:00, and 16:00
 *
 * @param {number} time A unix timestamp in milliseconds
 */
fn convertToNearestEorzeanIntervalStart(timestamp: u64) {
    let EORZEA_HOUR = 175 * 1000;
    let eorzeanHoursFromEpoch = time / EORZEA_HOUR;
    let eorzeaTimeHour = (eorzeanHoursFromEpoch - (eorzeanHoursFromEpoch % 8)) % 24;
    return format!("{}:00", eorzeaTimeHour);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2));
    }
}
