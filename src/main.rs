#![windows_subsystem = "windows"]
use fltk::{app::*, button::*, input::*, window::*};
use globalenv::set_var;
use std::env;

mod image_creation;
mod request_maker;

//1ed014973f3aff63e2ec5bbb95751ef4
fn main() -> Result<(), systray::Error> {
    let mut app = match systray::Application::new() {
        Ok(w) => w,
        Err(_) => return Err(systray::Error::UnknownError),
    };

    // At app init : we create systray icon (generated from data fetched from API, or a pre-drawn error icon whatever the error)
    let error_icon = include_bytes!("../assets/error-5-16.ico");
    let icon = match image_creation::create_icon(None) {
        Ok(i) => i,
        Err(_) => error_icon.to_vec(),
    };
    app.set_tooltip("Temperature (Celcius)")?;
    app.set_icon_from_buffer(&icon[0..icon.len()], 256, 256)?;

    // City change
    app.add_menu_item("Change location", move |window| {
        let gui_window = App::default();
        let mut wind = Window::new(100, 100, 300, 80, "Please select location:");
        let city_input = Input::new(100, 30, 150, 20, "City Name:");
        let mut but = Button::new(250, 30, 30, 20, "OK");
        wind.end();
        wind.show();
        but.set_callback(Box::new(move || {
            // Sets updated environment variable for global and process
            set_var("OPENWEATHER_LOCATION", &city_input.value()).unwrap();
            gui_window.quit();
        }));
        gui_window.run().unwrap();

        let icon = match image_creation::create_icon(env::var("OPENWEATHER_LOCATION").ok()) {
            Ok(i) => i,
            Err(_) => error_icon.to_vec(),
        };
        window.set_icon_from_buffer(&icon[0..icon.len()], 256, 256)?;
        Ok::<_, systray::Error>(())
    })?;

    // Refresh menu : we fetch api data and update systray icon (TODO : automatic update ?)
    app.add_menu_item("Refresh", move |window| {
        let icon = match image_creation::create_icon(None) {
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
