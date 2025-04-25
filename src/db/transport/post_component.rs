use std::io::Cursor;

use image::{imageops::FilterType, GenericImageView, ImageReader};
use serde::{Deserialize, Serialize};

use crate::{cli::config::Config, db::components::{write_files, Component}};


#[derive(Serialize, Deserialize)]
pub struct PostComponent {
    pub component: Component,
    image: Option<Vec<u8>>,
    datasheet: Option<Vec<u8>>,
}


impl PostComponent {

    pub fn update_component_file_bools(&mut self) {

        if self.image.is_some(){
            self.component.image = true;
        } else {
            self.component.image = false;
        }

        if self.datasheet.is_some(){
            self.component.datasheet = true;
        } else {
            self.component.datasheet = false;
        }

    }


    pub fn optimise_image(&mut self) {
        if let Some(some_image) = &self.image {
            let res_img_reader = ImageReader::new(Cursor::new(some_image)).with_guessed_format();

            if let Ok(img_reader) = res_img_reader{
                let res_img = img_reader.decode();

                if let Ok(img) = res_img {

                    let new_img;
                    
                    if img.dimensions() != (1000, 1000) {

                        new_img = img.resize(1000, 1000, FilterType::Nearest);

                    } else {
                        new_img = img;
                    }

                    let mut bytes: Vec<u8> = Vec::new();
                        
                    new_img.write_to(&mut Cursor::new(&mut bytes), image::ImageFormat::Png);

                    self.image = Some(bytes);

                    println!("optimised!");

                    return;


                }
            }
        }

        println!("not optimised!");

        self.image = None;
    }

    pub fn create_assets(&self, id: i64, config: &Config) {
        write_files(id, "full.png", &config.asset_location, &self.image);
        write_files(id, "datasheet.pdf", &config.asset_location, &self.datasheet);
    }
}



