use crate::db::model::ActiveModel;
use sea_orm::{sea_query, ConnectionTrait, DatabaseConnection, DbErr, EntityTrait, Schema};
use std::sync::Arc;

pub mod model;


#[derive(Clone)]
pub struct Database {
    pub conn: Arc<DatabaseConnection>,
}

impl Database {
    pub async fn new(path: &str) -> Result<Self, DbErr> {
        let conn = sea_orm::Database::connect(format!("sqlite:{}?mode=rwc", path)).await?;
        Ok(Self {
            conn: Arc::new(conn),
        })
    }

    pub fn get_conn(&self) -> &DatabaseConnection {
        &self.conn
    }
    pub async fn upsert_device(
        &self,
        device_id: &str,
        battery: Option<f32>,
        rssi: Option<i32>,
    ) -> Result<model::Device, DbErr> {
        Ok(model::Entity::insert(ActiveModel::new(device_id)
            .with_battery(battery.unwrap_or_default())
            .with_rssi(rssi.unwrap_or_default())
        )
            .on_conflict(
                sea_query::OnConflict::column(model::Column::DeviceId)
                    .update_columns([model::Column::Battery, model::Column::Rssi, model::Column::LastSeen])
                    .to_owned()
            )
            .exec_with_returning(self.get_conn())
            .await?)
    }

    pub async fn get_device(&self, device_id: &str) -> Result<Option<model::Device>, DbErr> {
        model::Entity::find_by_id(device_id)
            .one(self.get_conn())
            .await
    }

    pub async fn get_all_devices(&self) -> Result<Vec<model::Device>, DbErr> {
        model::Entity::find().all(self.get_conn()).await
    }
}


pub async fn init_tables(db: &DatabaseConnection) -> Result<(), DbErr> {
    let builder = db.get_database_backend();
    let schema = Schema::new(builder);

    db.execute(builder.build(
        &schema.create_table_from_entity(model::Entity)
            .if_not_exists()
            .to_owned()
    )).await?;

    Ok(())
}
