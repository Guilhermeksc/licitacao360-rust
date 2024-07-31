// utils.rs

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

use polars::prelude::*;
use std::fs::File;
use crate::paths::Paths;

pub fn load_or_create(paths: &Paths, parquet_filename: &str, series: Vec<Series>) -> DataFrame {
    let parquet_path = paths.parquet_files.iter()
        .find(|path| path.0.ends_with(parquet_filename))
        .expect(&format!("Failed to find {}", parquet_filename));

    if parquet_path.0.exists() {
        LazyFrame::scan_parquet(parquet_path.0.to_str().unwrap(), Default::default())
            .expect(&format!("Failed to read {}", parquet_filename))
            .collect()
            .expect("Failed to collect LazyFrame to DataFrame")
    } else {
        let mut df = DataFrame::new(series).expect("Failed to create DataFrame");
        let file = File::create(&parquet_path.0).expect("Failed to create Parquet file");
        ParquetWriter::new(file).finish(&mut df).expect("Failed to write Parquet file");
        df
    }
}
