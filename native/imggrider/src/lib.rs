// use libc::{c_char, size_t};
use magick_rust::{
    bindings::CompositeOperator_OverCompositeOp, magick_wand_genesis, DrawingWand, MagickWand,
    PixelWand,
};
use std::{/*ffi::CStr,*/ path::PathBuf, str::Utf8Error, sync::Once};
use thiserror::Error as ThisError;
use uuid::Uuid;

static START: Once = Once::new();

#[derive(ThisError, Debug)]
pub enum Error {
    // 无效的文件名
    #[error("Invalid filename")]
    InvalidFilename,
    // 转换 MagickError
    #[error("Magick: {0}")]
    MagickError(#[from] magick_rust::MagickError),
    // 转换 Utf8Error
    #[error("Utf8: {0}")]
    Utf8Error(#[from] Utf8Error),
}

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
// TODO: 支持从外部传入 `Scheme` 结构。
pub struct Scheme {
    // 输出目录
    pub target_dir: String,
    // 扩展名
    pub extname: String,
    // 个体宽度
    pub indi_width: usize,
    // 个体高度
    pub indi_height: usize,
    // 水印字体大小
    pub watermark_font_size: f64,
}

// #[derive(Debug)]
// #[repr(C)]
// pub struct CArray<T> {
//     pub len: size_t,
//     pub data: *const T,
// }

// type PhotosPtr = *const CArray<*const c_char>;

// trait AsVec<T> {
//     fn as_vec(self) -> Result<Vec<T>>;
// }

// impl AsVec<String> for PhotosPtr {
//     fn as_vec(self) -> Result<Vec<String>> {
//         let c_array = unsafe { &*self };
//         let convert_rs = unsafe {
//             let slice = std::slice::from_raw_parts(c_array.data, c_array.len);
//             slice
//                 .iter()
//                 .map(|ptr| CStr::from_ptr(*ptr).to_str())
//                 .collect::<Vec<_>>()
//         };

//         let mut vector = vec![];
//         for r in convert_rs {
//             match r {
//                 Err(e) => return Err(Error::from(e)),
//                 Ok(s) => vector.push(s.to_string()),
//             }
//         }

//         Ok(vector)
//     }
// }

// fn generate(photos_ptr: PhotosPtr) {
//     // TODO: 添加错误处理。
//     let photos = match photos_ptr.as_vec() {
//         Err(_e) => return,
//         Ok(photos) => photos,
//     };

//     let assets_path = PathBuf::from("..").join("test").join("assets");
//     match _generate(
//         photos,
//         Scheme {
//             target_dir: assets_path.to_str().unwrap().to_string(),
//             extname: String::from("jpg"),
//             indi_width: 180,
//             indi_height: 120,
//             watermark_font_size: 54.0,
//         },
//     ) {
//         Ok(_) => return,
//         Err(_e) => return,
//     }
// }

rustler::init!("Elixir.ImgGrider", [generate]);

#[rustler::nif]
fn generate(photos: Vec<String>) {
    let assets_path = PathBuf::from("test").join("assets");

    match _generate(
        photos,
        Scheme {
            target_dir: assets_path.to_str().unwrap().to_string(),
            extname: String::from("jpg"),
            indi_width: 180,
            indi_height: 120,
            watermark_font_size: 54.0,
        },
    ) {
        Ok(_) => {
            // TODO: 处理成功
            return;
        }
        Err(_e) => {
            // TODO: 处理失败
            return;
        }
    }
}

fn _generate(photos: Vec<String>, scheme: Scheme) -> Result<String> {
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
    for (i, photo) in photos.iter().enumerate() {
        let mut wand = MagickWand::new();
        let mut draw = DrawingWand::new();
        let mut fill = PixelWand::new();
        let mut border = PixelWand::new();
        // 设置水印颜色和透明度
        fill.set_color("white")?;
        fill.set_alpha(0.45);
        // 设置水印边框颜色
        border.set_color("black")?;
        // 设置水印的字体家族、大小、粗细、颜色
        draw.set_font_family("mono")?;
        draw.set_font_size(scheme.watermark_font_size);
        draw.set_font_weight(800);
        draw.set_fill_color(&fill);
        // 设置水印的边框颜色和宽度
        draw.set_stroke_color(&border);
        draw.set_stroke_width(1.0);
        // 绘制水印和位置
        draw.draw_annotation(1.0, scheme.watermark_font_size, &(i + 1).to_string())?;
        wand.read_image(photo)?;
        wand.draw_image(&draw)?;
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
    let assets_path = PathBuf::from("..").join("..").join("test").join("assets");
    let mut photos = vec![];

    for i in 1..10 {
        let fpath = assets_path.clone().join(format!("photo-{}.jpg", i));

        photos.push(fpath.as_os_str().to_str().unwrap().to_string());
    }

    let r = _generate(
        photos,
        Scheme {
            target_dir: assets_path.to_str().unwrap().to_string(),
            extname: String::from("jpg"),
            indi_width: 180,
            indi_height: 120,
            watermark_font_size: 54.0,
        },
    );

    assert!(matches!(r, Ok(_)));
    assert!(PathBuf::from(r.unwrap()).exists());
}