use serde::Serialize;
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Serialize, FromRow)]
pub struct GroupInfo {
    pub id: Uuid,
    pub name: String,
    pub owner_id: Uuid,
    pub channel_id: Uuid,
}

pub type MemberId = uuid::Uuid;
