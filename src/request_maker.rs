use serde_json;
use std::{collections::HashMap, error::Error};

//Lembrar de dar unwrap no retorno do outro lado
fn make_request() -> Result<HashMap<String, serde_json::Value>, Box<dyn std::error::Error>> {
    let location = "Duque+de+Caxias";
    let app_id = "1ed014973f3aff63e2ec5bbb95751ef4";
    let url = format!(
        "http://api.openweathermap.org/data/2.5/weather?q={}&APPID={}&units=metric",
        location, app_id
    );
    let resp = reqwest::blocking::get(&url)?
        .json::<HashMap<String, serde_json::Value>>()?;
    println!("{:#?}", resp["main"]["temp"]);
    Ok(resp)
}

pub fn get_temp() -> Result<String, Box<dyn Error>> {
    let cop_temp_str = &make_request()?["main"]["temp"].as_f64().unwrap_or(0.0);
    Ok((*cop_temp_str as i64).to_string())
}
