use std::{fs::File, path::{Path, PathBuf}, sync::Arc};

use async_trait::async_trait;
use axum::{Json, body::Bytes, extract::{Multipart, State}, http::StatusCode, response::IntoResponse};
use axum_typed_multipart::{FieldData, FieldMetadata, TryFromChunks, TryFromMultipart, TypedMultipart, TypedMultipartError};
use futures::Stream;
use tempfile::NamedTempFile;
use uuid::Uuid;
use crate::{db::component::{properties::{file::{file::ComponentFile, service::ComponentFileService}, files::{self, DownloadedFile}, image::{image::ComponentImage, service::ComponentImageService}}, service::ComponentServices, transport_component::TransportComponent}, error::error::AppError, server::{server_state::ServerState}};


#[derive(TryFromMultipart)]
pub struct ComponentWithFile {
    component: TransportComponent,

    #[form_data(field_name = "file")]
    files: Vec<FieldData<NamedTempFile>>,

    image: Option<FieldData<NamedTempFile>>
}


pub async fn post_component_with_files(

    State(shared_state): State<Arc<ServerState>>,
    TypedMultipart(c_with_file): TypedMultipart<ComponentWithFile>,
) -> Result<impl IntoResponse, AppError> {

    println!("start");


    let id = shared_state.db.add_transport_component(&c_with_file.component).await?;

    

    for file in c_with_file.files {

        println!("found a file!");

        let downloaded_file = DownloadedFile {
            component_id: id,
            file_field: file
        };

        let file = ComponentFile::new(downloaded_file, &shared_state.config)?;

        shared_state.db.add_file(file).await?;
    }



    if let Some(image_present) = c_with_file.image {

        println!("found an image!");

        let downloaded_file = DownloadedFile {
            component_id: id,
            file_field: image_present
        };

        let image = ComponentImage::new(downloaded_file)?;

        println!("10");

        shared_state.db.add_img(image).await?;

    }

    println!("end");
    
    Ok(StatusCode::OK.into_response())

}


