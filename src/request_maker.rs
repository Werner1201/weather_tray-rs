use std::{collections::HashMap, env, error::Error};

// Fetching data from Open weather API
fn make_request() -> Result<HashMap<String, serde_json::Value>, Box<dyn std::error::Error>> {
    let location = env::var("OPENWEATHER_LOCATION").unwrap_or(String::from("Duque de Caxias"));
    // $env:OPENWEATHER_API_KEY="<paste key>"; cargo run
    let app_id = env::var("OPENWEATHER_API_KEY")?;
    let url = format!(
        "http://api.openweathermap.org/data/2.5/weather?q={}&APPID={}&units={}",
        location,
        app_id,
        env::var("OPENWEATHER_UNIT").unwrap()
    );
    let resp = reqwest::blocking::get(&url)?.json::<HashMap<String, serde_json::Value>>()?;
    println!("{:#?}", resp["main"]["temp"]);
    Ok(resp)
}

// Transforming API result to i64 temperature (° Celsius)
pub fn get_temp() -> Result<String, Box<dyn Error>> {
    let temp = make_request()?["main"]["temp"]
        .as_f64()
        .ok_or("Cannot convert temperature")?;
    Ok((temp as i64).to_string())
}
