#![windows_subsystem = "windows"]
mod image_creation;

//use std::path;

//#[cfg(target_os = "windows")]
fn main() -> Result<(), systray::Error> {
    let mut app;
    match systray::Application::new() {
        Ok(w) => app = w,
        Err(_) => panic!("Can't create window!"),
    }
    // w.set_icon_from_file(&"C:\\Users\\qdot\\code\\git-projects\\systray-rs\\resources\\rust.ico".to_string());
    // w.set_tooltip(&"Whatever".to_string());
    //let cam = "D\\pactw\\RustProjects\\teste_sys_tray\\temp.ico";
    println!("Hello, world!");
    image_creation::cria_imagem("13");

    //Mudar para apenas o nome do arquivo no final
    app.set_icon_from_file(&"..\\..\\..\\temp.ico".to_string())?;
    app.set_icon_from_file(&"..\\..\\..\\temp.ico".to_string())?;

    app.add_menu_item("Print a thing", |_| {
        println!("file!()");
        Ok::<_, systray::Error>(())
    })?;

    app.add_menu_item("Add Menu Item", |window| {
        window.add_menu_item("Interior item", |_| {
            println!("what");
            Ok::<_, systray::Error>(())
        })?;
        window.add_menu_separator()?;
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
