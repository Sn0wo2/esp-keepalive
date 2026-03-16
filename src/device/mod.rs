use crate::db::model;
use chrono::Local;
use sea_orm::Set;

impl model::ActiveModel {
    pub fn new(device_id: impl Into<String>) -> Self {
        Self {
            device_id: Set(device_id.into()),
            last_seen: Set(Local::now().naive_local()),
            ..Default::default()
        }
    }


    pub fn with_battery(mut self, battery: f32) -> Self {
        self.battery = Set(Some(battery));
        self
    }

    pub fn with_rssi(mut self, rssi: i32) -> Self {
        self.rssi = Set(Some(rssi));
        self
    }
}