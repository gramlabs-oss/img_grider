use magick_rust::{
    bindings::CompositeOperator_OverCompositeOp, magick_wand_genesis, MagickWand, PixelWand,
};
use std::{path::PathBuf, sync::Once};
use thiserror::Error as ThisError;
use uuid::Uuid;

static START: Once = Once::new();

#[derive(ThisError, Debug)]
pub enum Error {
    // 无效的文件名
    #[error("Invalid filename")]
    InvalidFilename,
    // 转换 MagickError
    #[error("Magick error: {0}")]
    MagickError(#[from] magick_rust::MagickError),
}

type Result<T> = std::result::Result<T, Error>;

pub struct Scheme {
    // 输出目标目录
    pub target_dir: String,
    // 扩展名
    pub extname: String,
    // 个体宽度
    pub indi_width: usize,
    // 个体高度
    pub indi_height: usize,
}

pub fn generate(photos: Vec<String>, scheme: Scheme) -> Result<String> {
    START.call_once(magick_wand_genesis);

    let output_path = PathBuf::from(scheme.target_dir).join(random_fname(&scheme.extname));
    let output = output_path.to_str().ok_or(Error::InvalidFilename)?;
    let mut wand = MagickWand::new();
    wand.new_image(
        scheme.indi_width * 3,
        scheme.indi_height * 3,
        &PixelWand::new(),
    )?;

    let mut photo_wands = vec![];
    for photo in photos {
        let wand = MagickWand::new();
        wand.read_image(&photo)?;
        photo_wands.push(wand);
    }

    wand.set_format(&scheme.extname)?;

    for (i, photo_wand) in photo_wands.iter().enumerate() {
        // TODO: 识别宽度或高度不足的图片，居中（或拉伸）它。
        let x = ((i % 3) * scheme.indi_width) as isize;
        let y = ((i / 3) * scheme.indi_height) as isize;

        wand.compose_images(photo_wand, CompositeOperator_OverCompositeOp, false, x, y)?;
    }

    wand.write_image(output)?;

    Ok(output.to_string())
}

fn random_fname(ext: &str) -> String {
    format!("{}.{}", Uuid::new_v4(), ext)
}

#[test]
fn test_generate() {
    let assets_path = PathBuf::from("..").join("test").join("assets");
    let mut photos = vec![];

    for i in 1..10 {
        let fpath = assets_path.clone().join(format!("photo-{}.jpg", i));

        photos.push(fpath.as_os_str().to_str().unwrap().to_string());
    }

    let r = generate(
        photos,
        Scheme {
            target_dir: assets_path.to_str().unwrap().to_string(),
            extname: String::from("jpg"),
            indi_width: 180,
            indi_height: 120,
        },
    );

    assert!(matches!(r, Ok(_)));
    assert!(PathBuf::from(r.unwrap()).exists());
}
