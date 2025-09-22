use crate::{domain::error::DomainError, types::Async};

// FIXME(Sa4dUs): check whether this should remain as an async trait or if we should take an
// alternative aproach as desugaring `async fn` as `-> Future<...>` or using trait specialization
// with `trait-variant` crate (https://crates.io/crates/trait-variant)
// NOTE(Sa4dUs): we probs won't need to care about `Future` and `Send` (or related) auto traits
pub trait GroupRepository {
    fn foo(&self) -> Async<'_, Result<(), DomainError>>;
}
