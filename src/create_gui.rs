#[path = "open_browser.rs"]
mod open_browser;
use fltk::{app::*, button::*, frame::*, input::*, window::*};
use std::env;

pub fn construct_gui() {
    let gui_window = App::default();
    let mut wind = Window::new(100, 100, 400, 300, "Hello from rust");
    let _get_key_label = Frame::new(150, 30, 100, 20, "To Obtain your key acess:");
    let mut link = Button::new(
        150,
        50,
        100,
        20,
        "https://home.openweathermap.org/users/sign_up",
    );
    link.set_frame(FrameType::FlatBox);
    link.clear_visible_focus();
    link.set_label_color(Color::Blue);
    // https://home.openweathermap.org/users/sign_up
    //let mut link_label = Frame::new(150,50,100,20,"https://home.openweathermap.org/users/sign_up",);
    let city_input = Input::new(150, 130, 150, 20, "City Name:");
    let key_input = Input::new(200, 170, 100, 20, "Your Personal Key:");
    let mut but = Button::new(100, 230, 200, 40, "Save");
    wind.end();
    wind.show();
    link.set_callback(Box::new(move || open_browser::open_browser()));
    but.set_callback(Box::new(move || {
        save_all(&city_input, &key_input, &gui_window)
    }));
    gui_window.run().unwrap();
}

fn set_key(key: String) {
    let app_id = key;
    println!("{}", app_id.clone());
    env::set_var("OPENWEATHER_API_KEY", app_id.clone());
    println!(
        "{:?}",
        assert_eq!(env::var("OPENWEATHER_API_KEY"), Ok(app_id))
    );
}

fn set_city(city: String) {
    let city_copy = city;
    println!("{}", city_copy.clone());
    env::set_var("WEATHER_CITY", city_copy.clone());
    println!("{:?}", assert_eq!(env::var("WEATHER_CITY"), Ok(city_copy)));
}

fn save_all(city_entry: &Input, key_entry: &Input, gui_window: &App) {
    set_key(key_entry.value());
    set_city(city_entry.value());
    gui_window.quit();
}
