use std::io::Cursor;

use image::{imageops::FilterType, GenericImageView, ImageReader};
use serde::{Deserialize, Serialize};

use crate::db::component::service::write_component_files;
use crate::db::transport::transport_component_type_value::TransportComponentTypeValue;
use crate::db::types::component_type_value::ComponentTypeValue;
use crate::{config::config::Config};
use crate::db::{component::component::Component, db::DB};



#[derive(Serialize, Deserialize)]
pub struct TransportComponent {

    // COMPONENT 
    pub name: String,
    pub stock: i32,
    pub price: Option<f32>,
    pub manufacturer: Option<String>,
    pub label: Option<String>,
    pub image: bool,
    pub datasheet: bool,

    // ATTRIBUTES
    pub attributes: Vec<TransportComponentTypeValue>,

}


impl TransportComponent {

    pub fn create_component_type_values(&self, c_id: i64) -> Vec<ComponentTypeValue> {

        let mut list = Vec::new();

        for attribute in &self.attributes {

            list.push(
                ComponentTypeValue{
                    component_id: c_id,
                    type_id: attribute.type_id,
                    attributes: attribute.attributes.clone(),
                }
            )

        }

        return list

    }

}


// impl TransportComponent {

//     pub fn update_component_file_bools(&mut self) {

//         self.component.image = self.image.is_some();
//         self.component.datasheet = self.datasheet.is_some();


//     }


//     pub fn optimise_image(&mut self) {
//         if let Some(some_image) = &self.image {
//             let res_img_reader = ImageReader::new(Cursor::new(some_image)).with_guessed_format();

//             if let Ok(img_reader) = res_img_reader{
//                 let res_img = img_reader.decode();

//                 if let Ok(img) = res_img {

//                     let new_img;
                    
//                     if img.dimensions() != (1000, 1000) {

//                         new_img = img.resize(1000, 1000, FilterType::Nearest);

//                     } else {
//                         new_img = img;
//                     }

//                     let mut bytes: Vec<u8> = Vec::new();
                        
//                     new_img.write_to(&mut Cursor::new(&mut bytes), image::ImageFormat::Png);

//                     self.image = Some(bytes);


//                     return;


//                 }
//             }
//         }


//         // self.image = None;
//         // self.component.image = false;
//     }

//     pub fn create_assets(&self, id: i64, config: &Config) {
//         write_component_files(id, "full.png", &config.asset_location, &self.image, self.component.image);
//         write_component_files(id, "datasheet.pdf", &config.asset_location, &self.datasheet, self.component.datasheet);
//     }
// }



