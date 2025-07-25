use std::{cell::OnceCell, collections::HashMap, fs, hash::Hash, io::{Cursor, Write}, ops::Deref, path::{Path, PathBuf}, sync::{Arc, Mutex, RwLock}, thread};

use axum::extract::State;

use futures::{future::{join_all, BoxFuture, Shared}, FutureExt};
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

    async fn build_cached(&self, file: String, config: &Config, fonts: Arc<FontCombined>) -> Vec<u8>;

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

    async fn build_cached(&self, file: String, config: &Config, fonts: Arc<FontCombined>) -> Vec<u8> { // catch all these errors please!

        let world = typst_wrapper::TypstWrapperWorld::new(config.label_location.to_owned(), file, self.get_inputs(config), fonts);

        let document: typst::layout::PagedDocument = typst::compile(&world)
                .output
                .expect("ahahha!");

        let pdf = tokio::task::spawn_blocking(move || {

            
            
            typst_pdf::pdf(&document, &PdfOptions::default())
        
        })
            .await
            .expect("failed to build!").expect("nah");

        return pdf;


    }

    



    async fn build_zip(labels: Vec<Self>, config: &Config) -> Option<Vec<u8>> {

        let template_cache_arc: Arc<tokio::sync::RwLock<TemplateMap>> = Arc::new(tokio::sync::RwLock::new(HashMap::new()));

        let package_cache_arc: Arc<RwLock<HashMap<String, String>>> = Arc::new(RwLock::new(HashMap::new()));



        let root = PathBuf::from(config.label_location.to_owned());

        let font = config.font_location.to_owned();

        let fonts = FontSearcher::new().include_system_fonts(true).search_with([root.join(font)]);


        let font_combined = Arc::new(FontCombined {
            book: LazyHash::new(fonts.book),
            fonts: fonts.fonts
        });


        let mut handles: Vec<tokio::task::JoinHandle<Option<(String, Vec<u8>)>>> = Vec::new(); //: Vec<thread::ScopedJoinHandle<'_, Option<(String, Vec<u8>)>>>

        let mut i = 0;
    
        for label in labels {

            i += 1;

            if label.label.is_some() {

                let c = config.clone(); // get rid of this clone!


                let template_cache = Arc::clone(&template_cache_arc);
                let font_combined_arc = Arc::clone(&font_combined);



                handles.push(tokio::spawn( async move {

                


                    let name: String;
                    
                    if let Some(id) = label.id {
                        name = format!("{}-{}.pdf", label.name, id);
                    } else {

                        println!("ERROR COULDNT FIND ID");
                        return None;
                    }

                    


                    let template_data = get_template(template_cache, &label.label.as_ref().unwrap(), &c).await;
                    

                    

                    

                    
                    
                    println!("building {}!", i);

                    let pdf_bytes = label.build_cached(template_data.expect("couldnt find it"), &c, font_combined_arc).await;

                    println!("finished building {}!", i);

                    return Some((name, pdf_bytes));
                
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

async fn get_template(template_cache: Arc<tokio::sync::RwLock<TemplateMap>>, label_name: &String, config: &Config) -> Option<String> {


    {
        let r =  template_cache.read().await;

        if let Some(cached) = r.get(label_name) {
            println!("found a cached version");

            return cached.clone().await
        }
    }


    let mut w = template_cache.write().await;

    let location: &str = &config.label_location;
    let path = PathBuf::new().join(location).join(label_name.to_owned()+".typ");

    let future = async{
        if path.exists() {
            let template_data = tokio::fs::read_to_string(path).await.expect("Unable to read File!");// VERY SLOW OPPERATION
            //println!("reading it");
            //w.insert(label_name.to_string(), template_data.clone());

            return Some(template_data);

        } else {
            println!("ERROR! Path is {}", path.display());
        }

        return None;
    }.boxed().shared();

    w.insert(
        label_name.to_string(),
        future.clone()
    );

    drop(w);

    return future.await;

}


fn insert_optional(dict: &mut Dict, key: &str, value: &Option<String>) {
    dict.insert(key.into(), Value::Str(value.clone().unwrap_or_default().into()));
}

#[cfg(test)]
mod tests {
    use tokio::runtime::Runtime;

    use crate::db;

    use super::*;

    #[test]

    fn test_build_zip() {

        let rt  = Runtime::new().unwrap();

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


        let result = rt.block_on(Component::build_zip(components, &config));

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

