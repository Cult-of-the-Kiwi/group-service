use crate::application::bus::EventBus;

// TODO(Sa4dUs): Implement for fluvio event stream
impl EventBus for () {
    fn publish(
        &self,
        event: (),
    ) -> crate::types::Async<'_, Result<(), crate::application::error::AppError>> {
        todo!()
    }
}
