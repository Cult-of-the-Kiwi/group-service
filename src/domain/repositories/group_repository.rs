use async_trait::async_trait;

use crate::{
    application::dto::{GroupInfo, MemberId},
    infrastructure::error::InfraError,
};

// FIXME(Sa4dUs): check whether this should remain as an async trait or if we should take an
// alternative aproach as desugaring `async fn` as `-> Future<...>` or using trait specialization
// with `trait-variant` crate (https://crates.io/crates/trait-variant)
// NOTE(Sa4dUs): we probs won't need to care about `Future` and `Send` (or related) auto traits
#[async_trait]
pub trait GroupRepository {
    async fn insert_group(
        &self,
        group_id: uuid::Uuid,
        name: String,
        owner_id: uuid::Uuid,
        channel_id: uuid::Uuid,
    ) -> Result<GroupInfo, InfraError>;

    async fn insert_group_members(
        &self,
        group_id: uuid::Uuid,
        member_ids: Vec<uuid::Uuid>,
    ) -> Result<(), InfraError>;
}
