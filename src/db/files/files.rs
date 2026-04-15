use std::{fs::{self, File}, path::{Path, PathBuf}};

use axum::{BoxError, body::Bytes, extract::{Multipart, multipart}};
use futures::{Stream, TryStreamExt, io};
use tokio::{fs::File as TkFile, io::BufWriter};
use tokio_util::{bytes::Buf, io::StreamReader};
use uuid::Uuid;

use crate::{config::config::Config, db::files::file::file::ComponentFile, error::{error::AppError, file::FileError}};





pub async fn stream_file(mut multipart: Multipart, config: &Config) -> Result<(), AppError>{

    let mut option_c_id: Option<i64> = None;
    let mut option_file_type: Option<String> = None;
    let mut option_file: Option<(PathBuf, Uuid)> = None;



    while let Some(mut field) = multipart.next_field().await? {

        match field.name() {
            Some("c_id") => option_c_id = {
                let bytes = field.bytes().await?;

                let id = bytes.try_get_i64().map_err(|_| FileError::Upload("could not get i64 id from input".to_owned()))?;

                //.map_err(AppErrorFileError::Upload("could not get i64 id from input".to_owned()))?;

                Some(id)
            
            },
            Some("file_type") => option_file_type = Some(field.text().await?),
            Some("file") => {


                let uuid = Uuid::new_v4();

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



    let c_id = option_c_id.ok_or(FileError::WriteUpload)?;
    let file_type = option_file_type.ok_or(FileError::WriteUpload)?;
    let file_path_uuid = option_file.ok_or(FileError::WriteUpload)?;


    match file_type.as_str(){
        "image" => {

        },
        "file" => {
            ComponentFile::add_from_temp_file(c_id, file_path_uuid.0, file_path_uuid.1, config)?;
        },
        _ => return Err(FileError::WriteUpload.into())
    }

    
    



    Ok(())

}