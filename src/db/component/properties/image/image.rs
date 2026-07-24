use std::{fs::File, io::{BufReader, Read}, path::PathBuf};

use std::io::Cursor;
use image::{GenericImageView, ImageReader, codecs::avif::AvifEncoder};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use tokio::fs;
use uuid::Uuid;

use crate::{config::config::Config, db::component::properties::files::{DownloadedFile}, error::{error::AppError, file::FileError}};


const FULL_SIZE: u32 = 1000;
const THUMB_SIZE: u32 = 250;

#[derive(Clone, Debug, FromRow, Serialize, Deserialize)]
pub struct ComponentImage{
    pub component_id: Uuid,
    pub full: Vec<u8>,
    pub thumb: Vec<u8>
}



impl ComponentImage {

    pub fn new(file: DownloadedFile) -> Result<Self, AppError> {

        let temp_file = File::open(file.file_path)?;

        let reader = BufReader::new(temp_file);

        let img = ImageReader::new(reader).with_guessed_format()?.decode().map_err(|_| FileError::WriteUpload)?;

        let full_img = img.resize(FULL_SIZE, FULL_SIZE, image::imageops::FilterType::Nearest);
        let thumb_img = img.resize(THUMB_SIZE, THUMB_SIZE, image::imageops::FilterType::Nearest);

        let mut full_bytes: Vec<u8> = Vec::new();
        let mut thumb_bytes: Vec<u8> = Vec::new();


        let full_encoder = AvifEncoder::new_with_speed_quality(Cursor::new(&mut full_bytes), 3, 25);
        let thumb_encoder = AvifEncoder::new_with_speed_quality(Cursor::new(&mut thumb_bytes), 3, 25);


        full_img.write_with_encoder(full_encoder);
        thumb_img.write_with_encoder(thumb_encoder);

        Ok(ComponentImage { 
            component_id: file.component_id,
            full: full_bytes,
            thumb: thumb_bytes 
        })

    }


}