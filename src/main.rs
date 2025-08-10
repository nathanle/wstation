use reqwest;
use serde_json::{Value, json};
use serde::{Deserialize, Serialize};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Tempest API Token 
    #[arg(short, long)]
    token: String,
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
    timestamp: u64,
    uv: f64,
    wet_bulb_globe_temperature: f64,
    wet_bulb_temperature: f64,
    wind_avg: f64,
    wind_chill: f64,
    wind_direction: u64,
    wind_gust: f64,
    wind_lull: f64
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let url = format!("https://swd.weatherflow.com/swd/rest/observations/station/144129?token={0}", args.token);

    let response = reqwest::get(url).await?;

    if response.status().is_success() {
        let json: serde_json::Value = response.json().await?;
        let data: StationData = serde_json::from_value(json["obs"][0].clone()).unwrap();
        println!("{}", serde_json::to_string_pretty(&data).unwrap());
    } else {
        eprintln!("Request failed with status: {}", response.status());
    }

    Ok(())
}
