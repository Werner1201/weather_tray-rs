use std::{collections::HashMap, error::Error, env};

// Fetching data from Open weather API
fn make_request() -> Result<HashMap<String, serde_json::Value>, Box<dyn std::error::Error>> {
    let location = "Isbergues, FR";
    // $env:OPENWEATHER_API_KEY="<paste key>"; cargo run
    let app_id = env::var("OPENWEATHER_API_KEY")?;
    let url = format!(
        "http://api.openweathermap.org/data/2.5/weather?q={}&APPID={}&units=metric",
        location, app_id
    );
    let resp = reqwest::blocking::get(&url)?
        .json::<HashMap<String, serde_json::Value>>()?;
    println!("{:#?}", resp["main"]["temp"]);
    Ok(resp)
}

// Transforming API result to i64 temperature (° Celsius)
pub fn get_temp() -> Result<String, Box<dyn Error>> {
    let cop_temp_str = &make_request()?["main"]["temp"].as_f64().ok_or("Cannot convert temperature")?;
    Ok((*cop_temp_str as i64).to_string())
}
