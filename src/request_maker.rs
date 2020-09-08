use globalenv::set_var;
use serde_json::Value;
use std::{env, error::Error};

// Fetching data from Open weather API
fn make_request() -> Result<Value, Box<dyn std::error::Error>> {
    let location = env::var("OPENWEATHER_LATLONG").unwrap();
    // $env:OPENWEATHER_API_KEY="<paste key>"; cargo run
    let app_id = env::var("OPENWEATHER_API_KEY")?;
    let _ = get_geocode();
    let url = format!(
        "https://atlas.microsoft.com/weather/currentConditions/json?subscription-key={}&api-version=1.0&query={}&unit={}",
        app_id,
        location,
        env::var("OPENWEATHER_UNIT").unwrap()
    );
    let resp: Value = reqwest::blocking::get(&url)?.json()?;
    println!("{:#?}", resp["results"][0]["temperature"]["value"]);
    Ok(resp)
}

// Transforming API result to i64 temperature (° Celsius)
pub fn get_temp() -> Result<String, Box<dyn Error>> {
    let temp = make_request()?["results"][0]["temperature"]["value"]
        .as_f64()
        .ok_or("Cannot convert temperature")?;
    Ok((temp as i64).to_string())
}

fn get_geocode() -> Result<(), Box<dyn std::error::Error>> {
    let location = env::var("OPENWEATHER_LOCATION").unwrap_or(String::from("Duque de Caxias"));
    // $env:OPENWEATHER_API_KEY="<paste key>"; cargo run
    let app_id: String = env::var("OPENWEATHER_API_KEY").unwrap();
    let url = format!(
        "https://atlas.microsoft.com/search/address/json?subscription-key={}&api-version=1.0&query={}",
        app_id.to_owned(),
        location.to_owned(), 
        //env::var("OPENWEATHER_UNIT").unwrap()
    );
    let resp: Value = reqwest::blocking::get(&url)?.json()?;
    set_var(
        "OPENWEATHER_LATLONG",
        &format!(
            "{}, {}",
            resp["results"][0]["position"]["lat"].as_f64().unwrap(),
            resp["results"][0]["position"]["lon"].as_f64().unwrap()
        ),
    ).unwrap();

    Ok(())
}
