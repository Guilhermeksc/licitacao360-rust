use druid::{ImageBuf, piet::ImageFormat};
use std::path::Path;
use std::sync::Arc;

use std::io::Error as IoError;

pub fn load_image(path: &Path) -> Result<ImageBuf, IoError> {
    let image_data = std::fs::read(path)?;
    let image = image::load_from_memory(&image_data)
        .map_err(|e| IoError::new(std::io::ErrorKind::Other, e))?
        .to_rgba8();
    let (width, height) = image.dimensions();
    let raw_pixels: Arc<[u8]> = Arc::from(image.into_raw());
    Ok(ImageBuf::from_raw(raw_pixels, ImageFormat::RgbaSeparate, width as usize, height as usize))
}

use druid::widget::Label;
use druid::{Color, FontDescriptor, FontFamily, FontWeight, TextAlignment, Widget, WidgetExt};
use crate::AppState;

pub fn create_title_content(text: &str, text_size: f64, text_color: Color, weight: FontWeight, alignment: TextAlignment, padding: f64) -> impl Widget<AppState> {
    Label::new(text.to_string())
        .with_text_size(text_size)
        .with_text_color(text_color)
        .with_font(FontDescriptor::new(FontFamily::SYSTEM_UI).with_weight(weight).with_size(text_size))
        .with_text_alignment(alignment)
        .padding(padding)
}
