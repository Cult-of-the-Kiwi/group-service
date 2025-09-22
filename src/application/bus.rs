use crate::{application::error::AppError, types::Async};

pub trait EventBus {
    // TODO(Sa4dUs): Integrate with custom fluvio crate and set `event` type
    fn publish(&self, event: ()) -> Async<'_, Result<(), AppError>>;
}
