use image::{DynamicImage, ImageOutputFormat, Rgba};
use imageproc::drawing::draw_text_mut;
use rusttype::{Font, Scale};
use std::error::Error;
use winreg::enums::*;
use winreg::RegKey;

// Fetching data from API, returning the ready-to-display icon (or an error)
pub fn create_icon() -> Result<Vec<u8>, Box<dyn Error>> {
    let t = light_theme_query();
    let mut img = match t {
        Ok(1) => DynamicImage::new_rgba8(256, 256),
        _ => DynamicImage::new_rgb8(256, 256),
    };
    let temp = crate::request_maker::get_temp()?;

    let font = include_bytes!("../assets/DejaVuSans.ttf");
    let font = match Font::try_from_bytes(font) {
        Some(f) => f,
        None => return Err(Box::new(systray::Error::UnknownError)),
    };

    // Text scale
    let scale = Scale {
        x: 300.0 * 0.70,
        y: 300.0 * 1.0,
    };
    // Writing text
    if let Ok(1) = t {
        draw_text_mut(
            &mut img,
            Rgba([0u8, 0u8, 0u8, 255u8]),
            0,
            0,
            scale,
            &font,
            &temp,
        );
    } else {
        draw_text_mut(
            &mut img,
            Rgba([255u8, 255u8, 255u8, 255u8]),
            0,
            0,
            scale,
            &font,
            &temp,
        )
    };
    let mut icon = Vec::new();
    img.write_to(&mut icon, ImageOutputFormat::Ico)?;
    Ok(icon)
}

fn light_theme_query() -> Result<u32, Box<dyn Error>> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let ie_settings = hkcu.open_subkey_with_flags(
        "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Themes\\Personalize",
        KEY_READ,
    )?;
    let light_theme: u32 = ie_settings.get_value("SystemUsesLightTheme")?;
    Ok(light_theme)
}
