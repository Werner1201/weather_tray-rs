use image::{Rgb, RgbImage};
use imageproc::drawing::draw_text_mut;
use rusttype::{Font, Scale};
use std::path::Path;

pub fn cria_imagem(value: &str) {
    //aqui ele atribui o caminho ao local onde foram enviados os argumentos
    let path = Path::new("temp.ico");
    let image = RgbImage::new(256, 256);

    //aqui se salva a imagem no path definido
    let _ = escreve_texto(image, value).save(path).unwrap();
}

fn escreve_texto(mut img: RgbImage, temp: &str) -> RgbImage {
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
        Rgb([255u8, 255u8, 255u8]),
        0,
        0,
        scale,
        &font,
        &format!("{}°", temp).to_string(),
    );
    return img;
}
