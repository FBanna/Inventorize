use std::{fs::File, io::{BufReader, Read}, path::PathBuf};

use std::io::Cursor;
use image::{GenericImageView, ImageReader, codecs::avif::AvifEncoder};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use tokio::fs;

use crate::{config::config::Config, error::{error::AppError, file::FileError}};



#[derive(Clone, Debug, FromRow, Serialize, Deserialize)]
pub struct ComponentImage{
    pub component_id: i64,
    pub full: String,
    pub thumb: String
}


impl ComponentImage {

    pub async fn new(c_id: i64, temp_path: PathBuf, config: &Config) -> Result<Self, AppError> {

        let file = File::open(temp_path)?;

        let reader = BufReader::new(file);

        //let bytes = fs::read(temp_path).await?;

        let img = ImageReader::new(reader).with_guessed_format()?.decode().map_err(|_| FileError::WriteUpload)?;


        let encoder = AvifEncoder::new_with_speed_quality(w, speed, quality)

        

        let full_img = img.resize(1000, 1000, image::imageops::FilterType::Nearest);

        let thumb_img = img.resize(250, 250, image::imageops::FilterType::Nearest);

        let mut full_bytes = Vec::new();
        let mut thumb_bytes = Vec::new();

        full_img.write_to(&mut Cursor::new(&mut full_bytes), image::ImageFormat::Avif);

        thumb_img.write_to(&mut Cursor::new(&mut thumb_bytes), image::ImageFormat::Avif);




        todo!()

    }

}