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
    let mapa = make_request()?;
    let temp_num = &mapa["main"]["temp"];
    let cop_temp_str = temp_num.to_string();
    let test_float: f64 = cop_temp_str.parse()?;
    let test_round = test_float.round() as i64;
    Ok(test_round.to_string())
}
