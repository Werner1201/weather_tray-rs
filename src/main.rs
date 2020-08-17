#![windows_subsystem = "windows"]
mod image_creation;
mod request_maker;

//use std::path;

#[cfg(target_os = "windows")]
fn main() -> Result<(), systray::Error> {
    let mut app;
    match systray::Application::new() {
        Ok(w) => app = w,
        Err(_) => panic!("Can't create window!"),
    }

    // At app init : we create systray icon (TODO : error handling)
    let icon = image_creation::create_icon().unwrap();
    app.set_icon_from_buffer(&icon[0..icon.len()], 256, 256).expect("Cannot set icon");

    // Refresh : we recreate systray icon (TODO : automatic update ?)
    app.add_menu_item("Refresh", |window| {
        let icon = image_creation::create_icon().unwrap();
        window.set_icon_from_buffer(&icon[0..icon.len()], 256, 256).expect("Cannot set icon");
        Ok::<_, systray::Error>(())
    })?;

    app.add_menu_separator()?;

    app.add_menu_item("Quit", |window| {
        window.quit();
        Ok::<_, systray::Error>(())
    })?;

    println!("Waiting on message!");
    app.wait_for_message()?;
    Ok(())
}
