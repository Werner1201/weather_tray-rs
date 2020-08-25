#![windows_subsystem = "windows"]
use fltk::{app::*, button::*, input::*, window::*};
use globalenv::set_var;
use std::env;

mod image_creation;
mod request_maker;

//1ed014973f3aff63e2ec5bbb95751ef4
fn main() -> Result<(), systray::Error> {
    // Setting a temporary (and local to process) environment variable to store temperature unit user's choice
    env::set_var("OPENWEATHER_UNIT", "metric");
    
    let mut app = match systray::Application::new() {
        Ok(w) => w,
        Err(_) => return Err(systray::Error::UnknownError),
    };

    // At app init : we create systray icon (generated from data fetched from API, or a pre-drawn error icon whatever the error)
    let error_icon = include_bytes!("../assets/error-5-16.ico");
    let icon = match image_creation::create_icon() {
        Ok(i) => i,
        Err(_) => error_icon.to_vec(),
    };
    app.set_tooltip(
        &env::var("OPENWEATHER_LOCATION").unwrap().to_string(),
    )?;
    app.set_icon_from_buffer(&icon[0..icon.len()], 256, 256)?;

    // City change
    app.add_menu_item("Change location", move |window| {
        location_dialog();
        let icon = match image_creation::create_icon() {
            Ok(i) => i,
            Err(_) => error_icon.to_vec(),
        };
        window.set_icon_from_buffer(&icon[0..icon.len()], 256, 256)?;
        window.set_tooltip(
            &env::var("OPENWEATHER_LOCATION").unwrap().to_string(),
        )?;
        Ok::<_, systray::Error>(())
    })?;

    app.add_menu_item("Change unit", move |window| {
        if env::var("OPENWEATHER_UNIT").unwrap() == "metric" { env::set_var("OPENWEATHER_UNIT", "imperial"); }
        else { env::set_var("OPENWEATHER_UNIT", "metric"); }
        let icon = match image_creation::create_icon() {
            Ok(i) => i,
            Err(_) => error_icon.to_vec(),
        };
        window.set_icon_from_buffer(&icon[0..icon.len()], 256, 256)?;
        Ok::<_, systray::Error>(())
    })?;

    // Refresh menu : we fetch api data and update systray icon (TODO : automatic update ?)
    //We can't do autoupdates since the API has a limit of requests per day
    app.add_menu_item("Refresh", move |window| {
        let icon = match image_creation::create_icon() {
            Ok(i) => i,
            Err(_) => error_icon.to_vec(),
        };
        window.set_icon_from_buffer(&icon[0..icon.len()], 256, 256)?;
        Ok::<_, systray::Error>(())
    })?;

    app.add_menu_separator()?;

    app.add_menu_item("Quit", |window| {
        window.quit();
        Ok::<_, systray::Error>(())
    })?;

    app.wait_for_message()?;
    Ok(())
}

// Opens the location input window
fn location_dialog() {
    let a = App::default();
    let mut w = Window::new(100, 100, 300, 80, "Please select location:");
    let city_input = Input::new(100, 30, 150, 20, "City Name:");
    let mut b = Button::new(250, 30, 30, 20, "OK");
    w.end();
    w.show();
    b.set_callback(Box::new(move || {
        // Sets updated environment variable globally and for process
        set_var("OPENWEATHER_LOCATION", &city_input.value()).unwrap();
        a.quit();
    }));
    a.run().unwrap();
}
