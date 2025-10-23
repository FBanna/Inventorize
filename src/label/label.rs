use std::{collections::HashMap, fs, io::{Cursor, Write}, path::{PathBuf}, sync::{Arc, RwLock}};



use futures::{future::{BoxFuture, Shared}, FutureExt};
use serde::{Deserialize, Deserializer, Serialize};
use serde_yaml::with::singleton_map::deserialize;
use tokio::runtime::Handle;
use typst::{foundations::{Array, Bytes, Dict, IntoValue, Str, ToArray, Value}, layout::{Page, PagedDocument}, syntax::{ast::{Int}, package::PackageSpec}, text::FontBook, utils::{LazyHash, SliceExt}, Library};
use typst_kit::fonts::{FontSearcher, FontSlot};
use typst_pdf::PdfOptions;
use zip::write::SimpleFileOptions;

use crate::{cli::config::Config, db::components::Component};

use super::typst_wrapper;





pub trait Label {

    //fn debug_build(&self, label_path: String);
    
    //fn build(&self, config: &Config) -> Option<Vec<u8>>;

    fn build_pdf(labels: Vec<Self>, config: &Config) -> Option<Vec<u8>> where Self:Sized;

    //async fn build_cached(&self, file: String, config: &Config, fonts: Arc<FontCombined>, package_cache: Arc<tokio::sync::RwLock<PackageMap>>) -> Vec<u8>;

    fn get_inputs(labels: Vec<Self>, config: &Config) -> Library where Self:Sized;

    //fn build_save(&self, config: &Config);

    //async fn build_zip(labels: Vec<Self>, config: &Config) -> Option<Vec<u8>> where Self:Sized;

    //fn get_input_labels(labels: Vec<Self>, config: &Config) -> Library where Self:Sized;
    

}



impl Label for Component{


    /// Builds oen then saves it to pdf
    // fn build_save(&self, config: &Config) {

    //     let bytes = self.build(config);


    //     if let Some(some) = bytes {
    //         fs::write("./output.pdf",some).expect("Error Writing");
    //     }
    // }

    // /// Build individual returning pdf bytes
    // fn build(&self, config: &Config) -> Option<Vec<u8>> {
    //     if let Some(label) = &self.label {

    //         let location: &str = &config.label_location;
    //         let font: &str = &config.font_location;
    //         let path = PathBuf::new().join(location).join(label.to_owned()+".typ"); // VERY SLOW OPPERATION
            
    //         if path.exists(){// VERY SLOW OPPERATION
    //             let data = fs::read_to_string(path).expect("Unable to read File!");// VERY SLOW OPPERATION

    //             let fonts = FontSearcher::new().include_system_fonts(true).search_with([PathBuf::from(location).join(font)]);

    //             let font_arc = Arc::new(FontCombined {
    //                 book: LazyHash::new(fonts.book),
    //                 fonts: fonts.fonts
    //             });
                

    //             // let world = typst_wrapper::TypstWrapperWorld::new(location.to_owned(), data, self.get_inputs(config), font_arc);

    //             // let document: typst::layout::PagedDocument = typst::compile(&world)
    //             //     .output
    //             //     .expect("ahahha!");

    //             // let pdf = typst_pdf::pdf(&document, &PdfOptions::default()).expect("ERROR EXPORTING");

    //             // return Some(pdf);
                

                
    //         }
    //     }

    //     println!("failed for some reason");
    //     return None;
    // }


    /// takes vec of labels returns pdf bytes of all labels
    fn build_pdf(labels: Vec<Self>, config: &Config) -> Option<Vec<u8>> {

        let mut label_types: HashMap<String, Vec<Self>> = HashMap::new();

        for label in labels {
            if label.label.is_none() {
                continue;
            }

            label_types.entry(label.label.clone().unwrap())
                .or_default()
                .push(label);
        }

        let location: &str = &config.label_location;
        let font: &str = &config.font_location;
        let fonts = FontSearcher::new().include_system_fonts(true).search_with([PathBuf::from(location).join(font)]);
        let arc_font_slot = Arc::new(fonts.fonts);

        let mut pdfs = Vec::new();
        

        for (label_type, label_group) in label_types{

            let path = PathBuf::new().join(location).join(label_type.to_owned()+".typ");

            if !path.exists(){

                println!("couldnt find it AHHH!"); // ERROR NEEDED
                break;
            }

            let label_template = fs::read_to_string(path).expect("Unable to read File!");// VERY SLOW OPPERATION

            let world = typst_wrapper::TypstWrapperWorld::new(
                config.label_location.to_owned(), 
                label_template, 
                Component::get_inputs(label_group, config), 
                &fonts.book,
                Arc::clone(&arc_font_slot)
            );

            let document: typst::layout::PagedDocument = typst::compile(&world)
                .output
                .expect("ERROR building!");

            pdfs.push(document);


        }

        let flatterned: Vec<Page> = pdfs.iter().map(|x| x.pages.clone()).flatten().collect();


        let final_document = PagedDocument{
            pages: flatterned,
            ..Default::default()
        };

        let final_pdf = typst_pdf::pdf(&final_document, &PdfOptions::default()).expect("OH NO!");
        
        

        

        return Some(final_pdf);
    }


    /// gets inputs necessary for building labels
    fn get_inputs(labels: Vec<Self>, config: &Config) -> Library {
        
        let mut library: Dict = Dict::new();

        let label_vec: Vec<Value> = labels
            .iter()
            .map(|x| {


                    let mut dict = Dict::new();
                                    
                    dict.insert(
                        Str::from("name"), 
                        Value::Str(Str::from(x.name.clone()))
                    );

                    dict.insert(
                        Str::from("size"), 
                        Value::Str(Str::from(x.size.clone().unwrap_or_default()))
                    );

                    dict.insert(
                        Str::from("value"), 
                        Value::Str(Str::from(x.value.clone().unwrap_or_default()))
                    );

                    dict.insert(
                        Str::from("info"), 
                        Value::Str(Str::from(x.info.clone().unwrap_or_default()))
                    );

                    dict.insert(
                        Str::from("url"), 
                        Value::Str(Str::from(config.host_address.to_owned() + "/component/" + &x.id.clone().get_or_insert_default().to_string()))
                    );

                    return Value::Dict(dict);
                }
            ).collect::<Vec<Value>>();

        library.insert(
            Str::from("labels"),
            Value::Array(Array::from(label_vec.as_slice()))
        );

    
        let temp = Library::builder().with_inputs(library).build();


        return temp;
    }
    
    
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]

    fn test_build_zip() {

        //let rt  = Runtime::new().unwrap();

        let mut components = Vec::new();

        println!("100");

        for i in 1..100 {
            components.push(
                Component {
                    //ID:5000,
                    id: Some(i),
                    name:("Resistor".to_string()),
                    size:Some("0402".to_string()),
                    value:Some("60 OHM".to_string()),
                    info:None,
                    stock:5000,
                    origin:None, 
                    //url: None,
                    label: Some("vial".to_string()),
                    image: false,
                    datasheet: false
                }
            );
        };

        println!("config");
        

        let config = Config { 
            port: 0, 
            user: "".to_owned(), 
            password: "".to_owned(), 
            host_address: "localhost".to_owned(), 
            db_location: "".to_owned(), 
            label_location: "labels".to_owned(), 
            font_location: "fonts".to_owned(), 
            asset_location: "data".to_owned()
        };

        println!("building");


        let result = Component::build_pdf(components, &config);

        println!("done");


        assert!(result.is_some())


    }
}

// cargo test --release -- --nocapture
// 0: 28.86, 32.39, 32.83                       ISSUES: fonts calculated on every label
// 1: 22.24, 21.48, 15.36, 17.44, 17.66, 16.07  ISSUES: errors when having to download files at the same time
// 2: 20.01, 24.01, 23.45, 23.77, 23.45, 18.04  ISSUES: no improvement, same bug as before - tried to cache fonts even more
// 3: 15.49, 14.65, 15.05, 14.21, 14.39, 14.75  ISSUES: - changed to RwLock, tokio, futures cache. The whole 5 yards + more boiler plate in the actual test
// 4: 16.59, 14.58, 14.38, 14.12, 13.96, 14.46  ISSUES: no error catching - changed to spawn blocking for typst build
// 5: 15.61, 17.77, 19.07, 20.68, 20.41, 22.36  ISSUES: no catching errors - moved typst compile inside blocking *REVERT BACK TO (4)- rerun = runs much nicer in 4 no idea why?*
// 6: 14.36, 14.45, 15.78, 14.49, 14.66, 15.15  ISSUES: no error catching + downloading templates - back 10 (4) + arc for config so no more cloning!!!
