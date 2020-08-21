#![windows_subsystem = "windows"]
//#[path = "open_browser.rs"]
mod open_browser;
use fltk::{app::*, button::*, frame::*, input::*, window::*};
use globalenv::set_var;

fn main() {
    construct_gui();
}

fn construct_gui() {
    let gui_window = App::default();
    let mut wind = Window::new(100, 100, 400, 300, "Temperature Systray Setup");
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

fn save_all(city_entry: &Input, key_entry: &Input, gui_window: &App) {
    set_var("OPENWEATHER_API_KEY", &key_entry.value()).unwrap();
    set_var("OPENWEATHER_LOCATION", &city_entry.value()).unwrap();
    gui_window.quit();
}
