#![windows_subsystem = "windows"]
mod image_creation;
mod request_maker;

fn main() -> Result<(), systray::Error> {
    let mut app = match systray::Application::new() {
        Ok(w) => w,
        Err(_) => return Err(systray::Error::UnknownError),
    };

    // At app init : we create systray icon (generated from data fetched from API, or a pre-drawn error icon whatever the error)
    let error_icon = include_bytes!("../assets/error-5-16.ico");
    let icon = match image_creation::create_icon() {
        Ok(i) => i,
        Err(_) => error_icon.to_vec()
    };
    app.set_icon_from_buffer(&icon[0..icon.len()], 256, 256)?;

    // Refresh menu : we fetch api data and update systray icon (TODO : automatic update ?)
        app.add_menu_item("Refresh", move |window| {
        let icon = match image_creation::create_icon() {
            Ok(i) => i,
            Err(_) => error_icon.to_vec()
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
