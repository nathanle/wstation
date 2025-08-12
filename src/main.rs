use reqwest;
use serde_json;
use serde::{Serialize};
use clap::Parser;
use rust_decimal::prelude::*;
use chrono::{NaiveDateTime, DateTime, Utc, Local, TimeZone};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Tempest API Token 
    #[arg(short, long)]
    token: String,
    /// Station ID 
    #[arg(short, long)]
    station: String,
}

#[derive(serde::Deserialize, Serialize, Debug)]
struct StationData {

    air_density: f64,
    air_temperature: f64,
    barometric_pressure: f64,
    brightness: u64,
    delta_t: f64,
    dew_point: f64,
    feels_like: f64,
    heat_index: f64,
    lightning_strike_count: u64,
    lightning_strike_count_last_1hr: u64,
    lightning_strike_count_last_3hr: u64,
    lightning_strike_last_distance: u64,
    lightning_strike_last_epoch: u64,
    precip: f64,
    precip_accum_last_1hr: f64,
    precip_accum_local_day: f64,
    precip_accum_local_day_final: f64,
    precip_accum_local_yesterday: f64,
    precip_accum_local_yesterday_final: f64,
    precip_analysis_type_yesterday: u64,
    precip_minutes_local_day: u64,
    precip_minutes_local_yesterday: u64,
    precip_minutes_local_yesterday_final: u64,
    pressure_trend: String,
    relative_humidity: u64,
    sea_level_pressure: f64,
    solar_radiation: u64,
    station_pressure: f64,
    timestamp: i64,
    uv: f64,
    wet_bulb_globe_temperature: f64,
    wet_bulb_temperature: f64,
    wind_avg: f64,
    wind_chill: f64,
    wind_direction: u64,
    wind_gust: f64,
    wind_lull: f64
}

fn c_to_f(c: &f64) -> f64 {
    let s: f64 = *c as f64;
    let r = Decimal::from_f64(s * 1.8 + 32.0)
        .unwrap()
        .round_dp(2)
        .to_f64()
        .unwrap();

    r
}

fn round(c: &f64) -> f64 {
    let r = Decimal::from_f64(*c as f64)
        .unwrap()
        .round_dp(2)
        .to_f64()
        .unwrap();

    r
}
fn epoch_to_dt(e: &String) -> String {
    //let epoch: i64 = e.clone();
    let timestamp = e.parse::<i64>().unwrap();
    let naive = NaiveDateTime::from_timestamp(timestamp, 0);
    let datetime: DateTime<Utc> = DateTime::from_utc(naive, Utc);
    let newdate = datetime.format("%Y-%m-%d %H:%M:%S");

    newdate.to_string()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let url = format!("https://swd.weatherflow.com/swd/rest/observations/station/{}?token={}", args.station, args.token);

    let response = reqwest::get(url).await?;

    if response.status().is_success() {
        let json: serde_json::Value = response.json().await?;
        let data: StationData = serde_json::from_value(json["obs"][0].clone()).unwrap();
        let data2 = StationData {
            feels_like: c_to_f(&data.feels_like),
            heat_index: c_to_f(&data.heat_index),
            air_temperature: c_to_f(&data.air_temperature),
            precip: round(&data.precip),
            precip_accum_last_1hr: round(&data.precip_accum_last_1hr),
            //timestamp: epoch_to_dt(&data.timestamp),
            ..data
        };
        println!("{}", serde_json::to_string_pretty(&data2).unwrap());
    } else {
        eprintln!("Request failed with status: {}", response.status());
    }

    Ok(())
}
