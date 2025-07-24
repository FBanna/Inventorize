use std::{cell::OnceCell, collections::HashMap, fs, hash::Hash, io::{Cursor, Write}, ops::Deref, path::{Path, PathBuf}, sync::{Arc, Mutex, RwLock}, thread};

use axum::extract::State;

use futures::{future::{join_all, BoxFuture, Shared}, lock::Mutex, FutureExt};
use typst::{foundations::{Dict, Value}, html::tag::template, pdf, text::FontBook, utils::LazyHash, Feature, Features, Library};
use typst_kit::fonts::{FontSearcher, FontSlot, Fonts};
use typst_pdf::PdfOptions;
use zip::write::SimpleFileOptions;

use crate::{cli::config::Config, db::components::Component};

use super::typst_wrapper;


pub struct FontCombined {
    pub book: LazyHash<FontBook>,
    pub fonts: Vec<FontSlot>
}

type TemplateMap = HashMap<String, Shared<BoxFuture<'static, Option<String>>>>;


pub trait Label {

    //fn debug_build(&self, label_path: String);
    
    fn build(&self, config: &Config) -> Option<Vec<u8>>;

    fn build_cached(&self, file: String, config: &Config, fonts: Arc<FontCombined>) -> Vec<u8>;

    fn get_inputs(&self, config: &Config) -> Library;

    fn build_save(&self, config: &Config);

    async fn build_zip(labels: Vec<Self>, config: &Config) -> Option<Vec<u8>> where Self:Sized;
    

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

                let font_arc = Arc::new(FontCombined {
                    book: LazyHash::new(fonts.book),
                    fonts: fonts.fonts
                });
                

                let world = typst_wrapper::TypstWrapperWorld::new(location.to_owned(), data, self.get_inputs(config), font_arc);

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

    fn build_cached(&self, file: String, config: &Config, fonts: Arc<FontCombined>) -> Vec<u8> { // catch all these errors please!

        let world = typst_wrapper::TypstWrapperWorld::new(config.label_location.to_owned(), file, self.get_inputs(config), fonts);

        let document: typst::layout::PagedDocument = typst::compile(&world)
            .output
            .expect("ahahha!");

        let pdf = typst_pdf::pdf(&document, &PdfOptions::default()).expect("ERROR EXPORTING");

        return pdf;


    }

    



    async fn build_zip(labels: Vec<Self>, config: &Config) -> Option<Vec<u8>> {

        //let template_cache_arc: Arc<RwLock<HashMap<String, OnceCell<String>>>> = Arc::new(RwLock::new(HashMap::new()));

        

        //let test: Arc<RwLock<HashMap<>>>: DashMap<String, Arc<OnceCell<String>>
        let template_cache_arc: Arc<futures::lock::Mutex<TemplateMap>> = Arc::new(futures::lock::Mutex::new(HashMap::new()));

        let package_cache_arc: Arc<RwLock<HashMap<String, String>>> = Arc::new(RwLock::new(HashMap::new()));



        let root = PathBuf::from(config.label_location.to_owned());

        let font = config.font_location.to_owned();

        let fonts = FontSearcher::new().include_system_fonts(true).search_with([root.join(font)]);


        let font_combined = Arc::new(FontCombined {
            book: LazyHash::new(fonts.book),
            fonts: fonts.fonts
        });

        // let rt = tokio::runtime::Builder::new_current_thread()
        //     .enable_all()
        //     .build().expect("couldnt make runtime");

        let c = config.to_owned();




        

        //thread::scope(|scope| {

        // tokio::spawn(async move {

        
            
            let mut handles: Vec<tokio::task::JoinHandle<Option<(String, Vec<u8>)>>> = Vec::new(); //: Vec<thread::ScopedJoinHandle<'_, Option<(String, Vec<u8>)>>>
            
            let test = labels.clone();
        
            for (i, label) in test.iter().enumerate() {

                let t = label.clone();
                

                if let Some(label_name) = &t.label {

                    let m = label_name.clone();

                    
                    let c = config.clone();


                    let template_cache = Arc::clone(&template_cache_arc);
                    let font_combined_arc = Arc::clone(&font_combined);



                    handles.push(tokio::spawn( async move {

                    


                        let name: String;
                        
                        if let Some(id) = t.id {
                            name = format!("{}-{}.pdf", t.name, id);
                        } else {

                            println!("ERROR COULDNT FIND ID");
                            return None;
                        }

                        


                        let template_data = get_template(template_cache, &m, &c).await;
                        

                        

                        

                        
                        
                        println!("building {}!", i);

                        let pdf_bytes = label.build_cached(template_data.expect("couldnt find it"), &c, font_combined_arc);

                        println!("finished building {}!", i);

                        return Some((name, pdf_bytes));

                        // return None;

                    

                    
                    
                    }));


                    
                

                };
                    
                
                
            }

            let mut bytes: Vec<u8> = Vec::new();

            let mut zip = zip::ZipWriter::new(Cursor::new(&mut bytes));

            let options = SimpleFileOptions::default().compression_method(zip::CompressionMethod::Deflated);

            println!("compressing!");

            for handle in handles {

                if let Some(data) = handle.await.expect("nahh") {

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


        // }).await;

        


        return None;



        // let mut handles = Vec::new();

        // let c_arc = Arc::new(config.clone());

        // let template_cache_arc: Arc<Mutex<HashMap<String, String>>> = Arc::new(Mutex::new(HashMap::new()));


        // let root = PathBuf::from(config.label_location.to_owned());

        // let font = config.font_location.to_owned();

        // println!("fonts");

        // let fonts = Arc::new(FontSearcher::new().include_system_fonts(true).search_with([root.join(font)]));

        // println!("fonts done");

        
        
        

        // let mut i = 0;
        
        // for label in labels {

        //     i+=1;

        //     if let Some(label_name) = label.label.clone() {
        //         let c = Arc::clone(&c_arc);

        //         let template_cache_mutex = Arc::clone(&template_cache_arc);

        //         let fonts_arc = Arc::clone(&fonts);

        //         handles.push(thread::spawn(move || {
        //             let name: String;
                    
        //             if let Some(id) = label.id {
        //                 name = format!("{}-{}.pdf", label.name, id);
        //             } else {

        //                 println!("ERROR COULDNT FIND ID");
        //                 return None;

        //             }

                    

        //             let mut template_cache = template_cache_mutex.lock().unwrap();

        //             let template_data: String;

                    

        //             if !template_cache.contains_key(&label_name) {
        //                 let location: &str = &c.label_location;
        //                 let fonts: &str = &c.font_location;
        //                 let path = PathBuf::new().join(location).join(label_name.to_owned()+".typ"); // VERY SLOW OPPERATION
                        
        //                 if path.exists(){// VERY SLOW OPPERATION

        //                     template_data = fs::read_to_string(path).expect("Unable to read File!");// VERY SLOW OPPERATION
        //                     template_cache.insert(label_name, template_data.clone());
        //                 } else {
        //                     return None;
        //                 }
        //             } else {

        //                 println!("found a cached version");
        //                 template_data = template_cache.get(&label_name).expect("fauked").to_owned();
        //             }

        //             drop(template_cache);
                    
        //             println!("building {}!", i);

        //             let pdf_bytes = label.build_cached(template_data, &c, fonts_arc);

        //             println!("finished building {}!", i);

        //             return Some((name, pdf_bytes));



        //         }));
        //     }
            
        // }

        // let mut bytes: Vec<u8> = Vec::new();

        // let mut zip = zip::ZipWriter::new(Cursor::new(&mut bytes));

        // let options = SimpleFileOptions::default().compression_method(zip::CompressionMethod::Deflated);

        // println!("compressing!");

        // for handle in handles{

        //     if let Some(data) = handle.join().expect("nah") {
        //         let r1 = zip.start_file(data.0, options);

        //         if r1.is_err() {
        //             println!("failed to add label to zip!");
        //             zip.abort_file();
        //             continue;
        //         }

        //         let r2 = zip.write(&data.1);

        //         if r2.is_err(){
        //             println!("failed to write label to zip!");
        //             zip.abort_file();
        //             continue;
        //         }

        //     }
        // }

        // println!("finished compressing!");
        

        // let result = zip.finish();

        // println!("finished");

        
        // if result.is_err() {
        //     return None;
        // }

        // typst::comemo::evict(0); // clear all typst cache
        

        // return Some(bytes);
        
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

async fn get_template(template_cache: Arc<futures::lock::Mutex<TemplateMap>>, label_name: &String, config: &Config) -> Option<String> {

    return None;



    match template_cache.try_read() {
        Ok(r) => {

            if let Some(cached) = r.get(label_name) {
                println!("found a cached version");

                return cached.clone().await
            }

        },
        Err(e) => return None,
    }

    match template_cache.try_write() {
        Ok(mut w) => {

            let location: &str = &config.label_location;
            let path = PathBuf::new().join(location).join(label_name.to_owned()+".typ");

            let future = async{
                if path.exists() {
                    let template_data = fs::read_to_string(path).expect("Unable to read File!");// VERY SLOW OPPERATION
                    //println!("reading it");
                    //w.insert(label_name.to_string(), template_data.clone());

                    return Some(template_data);

                }

                return None;
                }.boxed().shared();

            w.insert(
                label_name.to_string(),
                future.clone()
            );

            return future.await;
            

            
        },
        Err(e) => return None
    }




    // let template_cache_result = template_cache.try_read(); //template_cache.lock().unwrap();

    // if let Ok(template_cache) = template_cache_result {
        

    //     if !template_cache.contains_key(label_name) {

    //         drop(template_cache);
    //         let location: &str = &config.label_location;
    //         //let fonts: &str = &config.font_location;
    //         let path = PathBuf::new().join(location).join(label_name.to_owned()+".typ"); // VERY SLOW OPPERATION
            
    //         if path.exists(){// VERY SLOW OPPERATION

    //                 template_data = fs::read_to_string(path).expect("Unable to read File!");// VERY SLOW OPPERATION

    //                 if let Ok(mut template_cache_writer) = template_cache.try_write() {
    //                     template_cache_writer.insert(label_name.to_string(), template_data.clone());
    //                 }

                    
    //             } else {
    //                 return None;
    //             }
    //     } else {

    //         println!("found a cached version");
    //         template_data = template_cache.get(label_name).expect("fauked").to_owned();
    //     }

    //     drop(template_cache);


    // }
}


fn insert_optional(dict: &mut Dict, key: &str, value: &Option<String>) {
    dict.insert(key.into(), Value::Str(value.clone().unwrap_or_default().into()));
}

#[cfg(test)]
mod tests {
    use crate::db;

    use super::*;

    #[test]
    fn test_build_zip() {

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

        let result = Component::build_zip(components, &config);

        println!("done");


        assert!(result.is_some())


    }
}

// cargo test --release -- --nocapture
// 0: 28.86, 32.39, 32.83                       ISSUES: fonts calculated on every label
// 1: 22.24, 21.48, 15.36, 17.44, 17.66, 16.07  ISSUES: errors when having to download files at the same time
// 2: 20.01, 24.01, 23.45, 23.77, 23.45, 18.04  ISSUES: no improvement, same bug as before - tried to cache fonts even more
// 3: ISSUES: - changed to RwLock