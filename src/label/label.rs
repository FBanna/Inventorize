use std::{fs, path::{Path, PathBuf}, sync::Arc};

use axum::extract::State;
use typst::{foundations::{Dict, Value}, Library};
use typst_pdf::PdfOptions;

use crate::{cli::config::Config, db::components::Component};

use super::typst_wrapper;

pub trait Label {

    //fn debug_build(&self, label_path: String);
    
    fn build(&self, label_location: &str, config: &Config) -> Option<Vec<u8>>;

    fn get_inputs(&self, config: &Config) -> Library;

    fn build_save(&self, label_location: &str, config: &Config);

}


impl Label for Component{

    fn build_save(&self, label_location: &str, config: &Config) {

        let bytes = self.build(label_location, config);


        if let Some(some) = bytes {
            fs::write("./output.pdf",some).expect("Error Writing");
        }

        
        
    }

    fn build(&self, label_location: &str, config: &Config) -> Option<Vec<u8>> {
        if let Some(label) = &self.label {
            let path = PathBuf::new().join(label_location).join(label.to_owned()+".typ");
            
            if path.exists(){
                let data = fs::read_to_string(path).expect("Unable to read File!");

                let world = typst_wrapper::TypstWrapperWorld::new("./".to_owned(), data, self.get_inputs(config));

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