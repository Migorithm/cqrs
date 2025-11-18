use crate::{aggregate::TAggregateES, event::EventEnvolope};

pub trait TEventStore<A: TAggregateES>: Sync + Send {
    fn load_events(
        &self,
        aggregate_id: &str,
    ) -> impl std::future::Future<Output = Result<Vec<EventEnvolope>, String>> + Send;
    fn load_aggregate(
        &self,
        aggregate_id: &str,
    ) -> impl std::future::Future<Output = Result<A, String>> + Send;

    fn commit(&self, aggregate: &A)
    -> impl std::future::Future<Output = Result<(), String>> + Send;
}
