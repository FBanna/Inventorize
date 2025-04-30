use std::{fs, io::{Cursor, Write}, path::{Path, PathBuf}, sync::Arc};

use axum::extract::State;
use typst::{foundations::{Dict, Value}, pdf, Library};
use typst_pdf::PdfOptions;
use zip::write::SimpleFileOptions;

use crate::{cli::config::Config, db::components::Component};

use super::typst_wrapper;

pub trait Label {

    //fn debug_build(&self, label_path: String);
    
    fn build(&self, config: &Config) -> Option<Vec<u8>>;

    fn get_inputs(&self, config: &Config) -> Library;

    fn build_save(&self, config: &Config);

    fn build_zip(labels: Vec<&Self>, config: &Config) -> Option<Vec<u8>>;

}


impl Label for Component{

    fn build_save(&self, config: &Config) {

        let bytes = self.build(config);


        if let Some(some) = bytes {
            fs::write("./output.pdf",some).expect("Error Writing");
        }

        
        
    }

    fn build(&self, config: &Config) -> Option<Vec<u8>> {
        if let Some(label) = &self.label {

            let location: &str = &config.label_location;
            let fonts: &str = &config.font_location;
            let path = PathBuf::new().join(location).join(label.to_owned()+".typ");
            
            if path.exists(){
                let data = fs::read_to_string(path).expect("Unable to read File!");

                let world = typst_wrapper::TypstWrapperWorld::new(location.to_owned(), data, self.get_inputs(config), fonts.to_owned());

                let document: typst::layout::PagedDocument = typst::compile(&world)
                    .output
                    .expect("ahahha!");

                let pdf = typst_pdf::pdf(&document, &PdfOptions::default()).expect("ERROR EXPORTING");

                return Some(pdf);

                
            }
        }

        println!("failed for some reason");
        return None;
    }
    

    fn build_zip(labels: Vec<&Self>, config: &Config) -> Option<Vec<u8>> {

        let mut bytes: Vec<u8> = Vec::new();

        let mut zip = zip::ZipWriter::new(Cursor::new(&mut bytes));

        let options = SimpleFileOptions::default().compression_method(zip::CompressionMethod::Deflated);
        
        for label in labels {

            let r1 = zip.start_file("", options);

            if r1.is_err() {
                println!("failed to add label to zip!");
            }

            let option_pdf_bytes = label.build(&config);

            if let Some(pdf_bytes) = option_pdf_bytes {

                let r2 = zip.write(&pdf_bytes);

                if r2.is_err(){
                    println!("failed to write label to zip!")
                }
                
            } 

            
        }

        let result = zip.finish();

        if result.is_err() {
            return None;
        }

        return Some(bytes);
        
    }



    fn get_inputs(&self, config: &Config) -> Library {

        let mut dict: Dict = Dict::new();

        dict.insert("name".into(), Value::Str(self.name.to_owned().into()));
        
        insert_optional(&mut dict, "size", &self.size);
        insert_optional(&mut dict, "value", &self.value);
        insert_optional(&mut dict, "info", &self.info);

        dict.insert("stock".into(), Value::Int(self.stock.into()));

        insert_optional(&mut dict, "origin", &self.origin);

        dict.insert("url".into(), Value::Str((config.host_address.to_owned() + "/component/" + &self.id.clone().get_or_insert_default().to_string()).into()));
        //insert_optional(&mut dict, "url", &self.url);
        

        Library::builder().with_inputs(dict).build()
    }
    
    
}

fn insert_optional(dict: &mut Dict, key: &str, value: &Option<String>) {
    dict.insert(key.into(), Value::Str(value.clone().unwrap_or_default().into()));
}