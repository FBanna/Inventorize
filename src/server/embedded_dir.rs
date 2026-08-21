use std::convert::Infallible;

use axum::{body::Body, http::{Request, Response, StatusCode, header}};
use rust_embed::Embed;
use tower::{Service, service_fn};



pub async fn handle_dir<E>(axum::extract::Path(path): axum::extract::Path<String>,) -> Response<Body>
where
    E: Embed
{


    let response = match E::get(&path) {
        Some(file) => {

            let mime = mime_guess::from_path(path)
                .first_or_octet_stream();

            let data = file.data.into_owned();


            Response::builder()
                .status(StatusCode::OK)
                .header(header::CONTENT_TYPE, mime.as_ref())
                .header(header::CONTENT_LENGTH, data.len())
                .body(Body::from(data))
                .unwrap()
        }

        None => Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::empty())
            .unwrap(),
    };

    response
}