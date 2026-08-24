use uuid::Uuid;

use crate::{db::{db::DB, label::{label::Label, transport_label::TransportLabel}}, error::error::AppError};




pub trait LabelServices {



    async fn add_transport_label(&self, transport_label: TransportLabel) -> Result<Uuid, AppError>;
    async fn get_label(&self, label_id: Uuid) -> Result<Label, AppError>;
    async fn get_all_labels(&self) -> Result<Vec<Label>, AppError>;

}


impl LabelServices for DB {
    async fn add_transport_label(&self, transport_label: TransportLabel) -> Result<Uuid, AppError> {

        let id: Uuid = sqlx::query_scalar("INSERT INTO label (name, path) VALUES ($1,$2) RETURNING label_id")
            .bind(&transport_label.name)
            .bind(&transport_label.path)
            .fetch_one(&*self.pool)
            .await?;

        Ok(id)
    }

    async fn get_label(&self, label_id: Uuid) -> Result<Label, AppError> {
        
        let label: Label = sqlx::query_as("SELECT * FROM label WHERE label_id = ($1)")
            .bind(&label_id)
            .fetch_one(&*self.pool)
            .await?;

        Ok(label)
    }
    
    async fn get_all_labels(&self) -> Result<Vec<Label>, AppError> {
        let labels: Vec<Label> = sqlx::query_as("SELECT * FROM label")
            .fetch_all(&*self.pool)
            .await?;

        Ok(labels)
    }
}