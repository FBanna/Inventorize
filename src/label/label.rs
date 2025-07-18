use std::{collections::HashMap, fs, io::{Cursor, Write}, path::{Path, PathBuf}, sync::{Arc, Mutex}, thread};

use axum::extract::State;
use typst::{foundations::{Dict, Value}, html::tag::template, pdf, Feature, Features, Library};
use typst_kit::fonts::{FontSearcher, Fonts};
use typst_pdf::PdfOptions;
use zip::write::SimpleFileOptions;

use crate::{cli::config::Config, db::components::Component};

use super::typst_wrapper;

pub trait Label {

    //fn debug_build(&self, label_path: String);
    
    fn build(&self, config: &Config) -> Option<Vec<u8>>;

    fn build_cached(&self, file: String, config: &Config, fonts: Arc<Fonts>) -> Vec<u8>;

    fn get_inputs(&self, config: &Config) -> Library;

    fn build_save(&self, config: &Config);

    fn build_zip(labels: Vec<Self>, config: &Config) -> Option<Vec<u8>> where Self:Sized;
    

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
            let font: &str = &config.font_location;
            let path = PathBuf::new().join(location).join(label.to_owned()+".typ"); // VERY SLOW OPPERATION
            
            if path.exists(){// VERY SLOW OPPERATION
                let data = fs::read_to_string(path).expect("Unable to read File!");// VERY SLOW OPPERATION

                let fonts = FontSearcher::new().include_system_fonts(true).search_with([PathBuf::from(location).join(font)]);

                let world = typst_wrapper::TypstWrapperWorld::new(location.to_owned(), data, self.get_inputs(config), Arc::new(fonts));

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

    fn build_cached(&self, file: String, config: &Config, fonts: Arc<Fonts>) -> Vec<u8> { // catch all these errors please!

        let world = typst_wrapper::TypstWrapperWorld::new(config.label_location.to_owned(), file, self.get_inputs(config), fonts.to_owned());

        let document: typst::layout::PagedDocument = typst::compile(&world)
            .output
            .expect("ahahha!");

        let pdf = typst_pdf::pdf(&document, &PdfOptions::default()).expect("ERROR EXPORTING");

        return pdf;


    }


    

    fn build_zip(labels: Vec<Self>, config: &Config) -> Option<Vec<u8>> {


        let mut handles = Vec::new();

        let c_arc = Arc::new(config.clone());

        let template_cache_arc: Arc<Mutex<HashMap<String, String>>> = Arc::new(Mutex::new(HashMap::new()));


        let root = PathBuf::from(config.label_location.to_owned());

        let font = config.font_location.to_owned();

        println!("fonts");

        let fonts = Arc::new(FontSearcher::new().include_system_fonts(true).search_with([root.join(font)]));

        println!("fonts done");

        
        

        let mut i = 0;
        
        for label in labels {

            i+=1;

            if let Some(label_name) = label.label.clone() {
                let c = Arc::clone(&c_arc);

                let template_cache_mutex = Arc::clone(&template_cache_arc);

                let fonts_arc = Arc::clone(&fonts);

                handles.push(thread::spawn(move || {
                    let name: String;
                    
                    if let Some(id) = label.id {
                        name = format!("{}-{}.pdf", label.name, id);
                    } else {

                        println!("ERROR COULDNT FIND ID");
                        return None;

                    }

                    

                    let mut template_cache = template_cache_mutex.lock().unwrap();

                    let template_data: String;

                    

                    if !template_cache.contains_key(&label_name) {
                        let location: &str = &c.label_location;
                        let fonts: &str = &c.font_location;
                        let path = PathBuf::new().join(location).join(label_name.to_owned()+".typ"); // VERY SLOW OPPERATION
                        
                        if path.exists(){// VERY SLOW OPPERATION

                            template_data = fs::read_to_string(path).expect("Unable to read File!");// VERY SLOW OPPERATION
                            template_cache.insert(label_name, template_data.clone());
                        } else {
                            return None;
                        }
                    } else {

                        println!("found a cached version");
                        template_data = template_cache.get(&label_name).expect("fauked").to_owned();
                    }

                    drop(template_cache);
                    
                    println!("building {}!", i);

                    let pdf_bytes = label.build_cached(template_data, &c, fonts_arc);

                    println!("finished building {}!", i);

                    return Some((name, pdf_bytes));



                }));
            }
            
        }

        let mut bytes: Vec<u8> = Vec::new();

        let mut zip = zip::ZipWriter::new(Cursor::new(&mut bytes));

        let options = SimpleFileOptions::default().compression_method(zip::CompressionMethod::Deflated);

        println!("compressing!");

        for handle in handles{

            if let Some(data) = handle.join().expect("nah") {
                let r1 = zip.start_file(data.0, options);

                if r1.is_err() {
                    println!("failed to add label to zip!");
                    zip.abort_file();
                    continue;
                }

                let r2 = zip.write(&data.1);

                if r2.is_err(){
                    println!("failed to write label to zip!");
                    zip.abort_file();
                    continue;
                }

            }
        }

        println!("finished compressing!");
        

        let result = zip.finish();

        println!("finished");

        
        if result.is_err() {
            return None;
        }

        typst::comemo::evict(0); // clear all typst cache
        

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

        let temp = Library::builder().with_inputs(dict).build();


        return temp;
    }
    
    
}

fn insert_optional(dict: &mut Dict, key: &str, value: &Option<String>) {
    dict.insert(key.into(), Value::Str(value.clone().unwrap_or_default().into()));
}