use uuid::Uuid;

use crate::{db::files::file::file::ComponentFile, error::error::AppError};




pub trait ComponentFileService {

    async fn add_file(&self, c_id: i64, file: ComponentFile) -> Result<(), AppError>;
    async fn del_file(&self, c_id: i64, uuid: String) -> Result<(), AppError>;
    async fn get_file(&self, c_id: i64, uuid: Uuid) -> Result<ComponentFile, AppError>;

    //async fn add_image(&self, )

}