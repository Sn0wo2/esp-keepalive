use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "device")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub device_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub battery: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rssi: Option<i32>,
    pub last_seen: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

pub type Device = Model;
pub type DeviceActive = ActiveModel;