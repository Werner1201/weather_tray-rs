#[path = "open_browser.rs"]
mod open_browser;
use fltk::{app::*, button::*, frame::*, input::*, window::*};

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
    let _city_input = Input::new(150, 130, 150, 20, "City Name:");
    let _key_input = Input::new(200, 170, 100, 20, "Your Personal Key:");
    let mut but = Button::new(100, 230, 200, 40, "Save");
    wind.end();
    wind.show();
    link.set_callback(Box::new(move || open_browser::open_browser()));
    but.set_callback(Box::new(move || println!("Saved in executable folder")));
    gui_window.run().unwrap();
}
