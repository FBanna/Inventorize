use std::{fs::{self, File}, path::{Path, PathBuf}};

use axum::{BoxError, body::Bytes, extract::{Multipart, multipart::{self, Field}}};
use futures::{Stream, StreamExt, TryStreamExt, io};
use tokio::{fs::File as TkFile, io::BufWriter};
use tokio_util::{bytes::{self, Buf}, io::StreamReader};
use uuid::Uuid;

use crate::{config::config::Config, db::{component::properties::{file::file::ComponentFile, files::DownloadedFile, image::image::ComponentImage}, db::DB}, error::{error::AppError, file::FileErrors}};






pub async fn get_file(mut multipart: Multipart, config: &Config) -> Result<DownloadedFile, AppError>{

    let mut option_component_id: Option<Uuid> = None;

    let mut option_file: Option<(PathBuf, Uuid)> = None; // just return bytes / a stream

    

    let mut option_file_name: Option<String> = None;



    while let Some(mut field) = multipart.next_field().await? {

        match field.name() {
            Some("component_id") => option_component_id = {
                let mut bytes = field.bytes().await?;

                // might fail!
                let raw_bytes: &[u8; 16] = bytes.as_array().ok_or(FileErrors::Upload("Could not extract component_id from request!".to_owned()))?;


                let id = Uuid::from_bytes(raw_bytes.to_owned());
                


                //.map_err(AppErrorFileError::Upload("could not get i64 id from input".to_owned()))?;

                Some(id)
            
            },
            Some("name") => option_file_name = Some(field.text().await?),
            Some("file") => {//option_file = Some(field),

                let uuid = Uuid::now_v7();

                let path: PathBuf = Path::new(&config.temp_location).join(uuid.as_hyphenated().to_string());
                
                let mut stream = field.into_stream();
                

                let mut stream_reader = StreamReader::new(stream.map_err(io::Error::other));

                //let body_with_io_error = stream.map_err(io::Error::other);
                // let mut body_reader = pin!(StreamReader::new(body_with_io_error));
                

                let mut file = BufWriter::new(TkFile::create(path.clone()).await?);

                // Copy the body into the file.
                tokio::io::copy(&mut stream_reader, &mut file).await?;


                option_file = Some((path, uuid));


            },
            _ => println!("null field?")
        }


    }



    let component_id = option_component_id.ok_or(FileErrors::WriteUpload)?;
    let name = option_file_name.ok_or(FileErrors::WriteUpload)?;
    let file_path_uuid = option_file.ok_or(FileErrors::WriteUpload)?;

    return Ok(DownloadedFile {
        name,
        component_id,
        file_path: file_path_uuid.0,
        temp_uuid: file_path_uuid.1
    });


    // match file_type.as_str(){
    //     "image" => {

    //         let c_image = ComponentImage::new(c_id, file_path_uuid.0, config)?;

    //         db.add_img(c_id, c_image).await?;

    //     },
    //     "file" => {

    //         let file_name = option_file_name.ok_or(FileError::WriteUpload)?;

    //         let c_file = ComponentFile::add_from_temp_file(c_id, file_path_uuid.1, file_path_uuid.0,  file_name, config)?;

    //         db.add_file(c_id, c_file).await?;
    //     },
    //     _ => return Err(FileError::WriteUpload.into())
    // }

    
    




}