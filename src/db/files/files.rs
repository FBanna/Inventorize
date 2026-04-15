use std::{fs::{self, File}, path::{Path, PathBuf}};

use axum::{BoxError, body::Bytes, extract::{Multipart, multipart}};
use futures::{Stream, TryStreamExt, io};
use tokio::{fs::File as TkFile, io::BufWriter};
use tokio_util::io::StreamReader;
use uuid::Uuid;

use crate::{config::config::Config, error::{error::AppError, rest::RestError}};


pub struct ComponentFile {
    pub file_id: Uuid,
    pub component_id: i64,
    pub name: String,
    pub mime: String
}


pub async fn stream_file(mut multipart: Multipart, config: &Config) -> Result<(), AppError>{

    let mut c_id: Option<String> = None;
    let mut file_type: Option<String> = None;
    let mut file_path: Option<PathBuf> = None;


    while let Some(mut field) = multipart.next_field().await? {

        match field.name() {
            Some("c_id") => c_id = Some(field.text().await?),
            Some("file_type") => file_type = Some(field.text().await?),
            Some("file") => {

                let path: PathBuf = Path::new(&config.temp_location).join(Uuid::new_v4().as_hyphenated().to_string());
                
                let mut stream = field.into_stream();

                let mut stream_reader = StreamReader::new(stream.map_err(io::Error::other));

                //let body_with_io_error = stream.map_err(io::Error::other);
                // let mut body_reader = pin!(StreamReader::new(body_with_io_error));
                

                let mut file = BufWriter::new(TkFile::create(path.clone()).await?);

                // Copy the body into the file.
                tokio::io::copy(&mut stream_reader, &mut file).await?;


                file_path = Some(path);


            },
            _ => println!("null field?")
        }


    }

    let final_path = Path::new(&config.asset_location)
        .join(c_id.ok_or(RestError::WriteUpload)?)
        .join({
            match file_type.ok_or(RestError::WriteUpload)?.as_str() {
                "image" => "image."
            }
        });

    fs::rename(file_path.ok_or(RestError::WriteUpload)?, final_path);
    



    Ok(())

}