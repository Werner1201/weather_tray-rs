use image::{Rgba, DynamicImage, ImageOutputFormat};
use imageproc::drawing::draw_text_mut;
use rusttype::{Font, Scale};
use std::error::Error;

// Fetching data from API, returning the ready-to-display icon (or an error)
pub fn create_icon() -> Result<Vec<u8>, Box<dyn Error>> {
    let mut img = DynamicImage::new_rgb8(256, 256);
    let temp = crate::request_maker::get_temp()?;
    let font = Vec::from(include_bytes!("../assets/DejaVuSans.ttf") as &[u8]);
    let font = match Font::try_from_vec(font) {
        Some(f) =>f,
        None => return Err(Box::new(systray::Error::UnknownError))
    };

    // Text scale
    let scale = Scale {
        x: 300.0 * 0.70,
        y: 300.0 * 1.0,
    };
    // Writing text
    draw_text_mut(
        &mut img,
        Rgba([255u8, 255u8, 255u8, 255u8]),
        0,
        0,
        scale,
        &font,
        &temp,
    );
    let mut icon = Vec::new();
    img.write_to(&mut icon, ImageOutputFormat::Ico)?;
    Ok(icon)
}
