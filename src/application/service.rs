use std::sync::Arc;

use uuid::Uuid;

use crate::{application::bus::EventBus, domain::repositories::group_repository::GroupRepository};

pub trait GroupService {
    fn create_new_group(&self, user_id: Uuid, member_ids: Vec<Uuid>);
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

impl GroupService for GroupServiceImpl {
    fn create_new_group(&self, user_id: uuid::Uuid, member_ids: Vec<uuid::Uuid>) {
        todo!()
    }
}
