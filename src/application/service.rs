use std::sync::Arc;

use async_trait::async_trait;
use uuid::Uuid;

use crate::{
    api::error::HandlerError,
    application::{bus::EventBus, dto::GroupInfo, error::AppError},
    domain::repositories::group_repository::GroupRepository,
};

#[async_trait]
pub trait GroupService {
    async fn create_new_group(
        &self,
        name: String,
        user_id: Uuid,
        member_ids: Vec<Uuid>,
    ) -> Result<GroupInfo, AppError>;
    async fn get_user_groups(&self, user_id: Uuid) -> Result<Vec<GroupInfo>, AppError>;
    async fn get_group(&self, user_id: Uuid, group_id: Uuid) -> Result<GroupInfo, AppError>;
    async fn delete_group(&self, user_id: Uuid, group_id: Uuid) -> Result<(), AppError>;
    async fn get_members(&self, user_id: Uuid, group_id: Uuid) -> Result<Vec<Uuid>, AppError>;
    async fn add_members(
        &self,
        user_id: Uuid,
        group_id: Uuid,
        member_ids: Vec<Uuid>,
    ) -> Result<(), AppError>;
    async fn remove_members(
        &self,
        user_id: Uuid,
        group_id: Uuid,
        member_ids: Vec<Uuid>,
    ) -> Result<(), AppError>;
    async fn get_by_channel(&self, user_id: Uuid, channel_id: Uuid) -> Result<GroupInfo, AppError>;
}

// TODO(Sa4dUs): Verify if this is the proper location for the trait impl
type TRepo = Arc<dyn GroupRepository + Send + Sync>;
type TBus = Arc<dyn EventBus + Send + Sync>;
pub struct GroupServiceImpl {
    repo: TRepo,
    bus: TBus,
}

impl GroupServiceImpl {
    pub fn new(repo: TRepo, bus: TBus) -> Self {
        GroupServiceImpl { repo, bus }
    }
}

#[async_trait]
impl GroupService for GroupServiceImpl {
    async fn create_new_group(
        &self,
        name: String,
        user_id: Uuid,
        member_ids: Vec<Uuid>,
    ) -> Result<GroupInfo, AppError> {
        let group_id = Uuid::new_v4();
        let owner_id = user_id;
        let channel_id = Uuid::new_v4();
        let group_info = self
            .repo
            .insert_group(group_id, name, owner_id, channel_id)
            .await
            .map_err(AppError::InfraError)?;

        self.repo.insert_group_members(group_id, member_ids).await?;

        // TODO(Sa4dUs): Add `EventBus` logic.

        Ok(group_info)
    }

    async fn get_user_groups(&self, user_id: Uuid) -> Result<Vec<GroupInfo>, AppError> {
        todo!()
    }

    async fn get_group(&self, user_id: Uuid, group_id: Uuid) -> Result<GroupInfo, AppError> {
        todo!()
    }

    async fn delete_group(&self, user_id: Uuid, group_id: Uuid) -> Result<(), AppError> {
        todo!()
    }

    async fn get_members(&self, user_id: Uuid, group_id: Uuid) -> Result<Vec<Uuid>, AppError> {
        todo!()
    }

    async fn add_members(
        &self,
        user_id: Uuid,
        group_id: Uuid,
        member_ids: Vec<Uuid>,
    ) -> Result<(), AppError> {
        todo!()
    }

    async fn remove_members(
        &self,
        user_id: Uuid,
        group_id: Uuid,
        member_ids: Vec<Uuid>,
    ) -> Result<(), AppError> {
        todo!()
    }

    async fn get_by_channel(&self, user_id: Uuid, channel_id: Uuid) -> Result<GroupInfo, AppError> {
        todo!()
    }
}
