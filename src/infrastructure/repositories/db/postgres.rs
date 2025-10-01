use async_trait::async_trait;
use sqlx::{PgPool, QueryBuilder};

use crate::{
    application::dto::GroupInfo, domain::repositories::group_repository::GroupRepository,
    infrastructure::error::InfraError,
};

#[async_trait]
impl GroupRepository for PgPool {
    async fn insert_group(
        &self,
        group_id: uuid::Uuid,
        name: String,
        owner_id: uuid::Uuid,
        channel_id: uuid::Uuid,
    ) -> Result<GroupInfo, InfraError> {
        sqlx::query("INSERT INTO groups (id, owner_id, channel_id) VALUES ($1, $2, $3)")
            .bind(group_id)
            .bind(owner_id)
            .bind(channel_id)
            .execute(self)
            .await
            .map_err(|_| InfraError::GenericError)?;

        Ok(GroupInfo {
            id: group_id,
            name,
            owner_id,
            channel_id,
        })
    }

    async fn insert_group_members(
        &self,
        group_id: uuid::Uuid,
        member_ids: Vec<uuid::Uuid>,
    ) -> Result<(), InfraError> {
        let mut query_builder = QueryBuilder::new("INSERT INTO group_members (group_id, user_id)");
        query_builder.push_values(&member_ids, |mut b, member_id| {
            b.push_bind(group_id).push_bind(member_id);
        });

        query_builder.push("ON CONFLICT DO NOTHING");

        query_builder
            .build()
            .execute(self)
            .await
            .map_err(|_| InfraError::GenericError)?;
        Ok(())
    }

    async fn get_groups_by_user(&self, user_id: uuid::Uuid) -> Result<Vec<GroupInfo>, InfraError> {
        todo!()
    }
}
