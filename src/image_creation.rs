use image::{Rgba, DynamicImage, ImageOutputFormat};
use imageproc::drawing::draw_text_mut;
use rusttype::{Font, Scale};
use std::error::Error;

pub fn create_icon() -> Result<Vec<u8>, Box<dyn Error>> {
    let mut img = DynamicImage::new_rgb8(256, 256);
    let temp = crate::request_maker::get_temp()?;
    let font = Vec::from(include_bytes!("docs/DejaVuSans.ttf") as &[u8]);
    let font = Font::try_from_vec(font).unwrap();

    //Aqui esta as definicioes perfeitinhas para criar uma imagem certinha como a do meu antigo projeto python.
    let height = 300.0;
    let scale = Scale {
        x: height * 0.57,
        y: height * 1.0,
    };
    //Aqui se desenha o texto na imagem com essa funcao
    draw_text_mut(
        &mut img,
        Rgba([255u8, 255u8, 255u8, 255u8]),
        0,
        0,
        scale,
        &font,
        &format!("{}°", temp),
    );
    let mut icon = Vec::new();
    img.write_to(&mut icon, ImageOutputFormat::Ico).expect("Cannot generate icon");
    Ok(icon)
}
