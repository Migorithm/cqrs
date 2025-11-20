use std::marker::PhantomData;

use chrono::{DateTime, Utc};
use serde_json::{from_str, json};

use crate::{
    aggregate::TAggregate,
    event::{EventEnvolope, TEvent},
    event_store::TEventStore,
};
#[derive(Default)]
pub struct InMemoryDB {
    table: Vec<EventTable>,
}

struct EventTable {
    envelope: EventEnvolope,
    timestamp: DateTime<Utc>,
}

impl EventTable {
    fn aggregate_id(&self) -> &str {
        &self.envelope.aggregate_id
    }

    fn envelope(&self) -> &EventEnvolope {
        &self.envelope
    }
}

pub struct SqlRepository<A: TAggregate> {
    executor: InMemoryDB,
    _phantom: PhantomData<A>,
}

impl<A: TAggregate> SqlRepository<A> {
    pub fn new() -> Self {
        Self {
            executor: InMemoryDB { table: vec![] },
            _phantom: Default::default(),
        }
    }
    fn extract_events(aggregate: &A) -> Vec<EventEnvolope> {
        let mut current_sequence = aggregate.sequence();
        let aggregate_type = aggregate.aggregate_type();
        let aggregate_id = aggregate.aggregate_id();
        aggregate
            .events()
            .iter()
            .map(|event| {
                current_sequence += 1;
                EventEnvolope {
                    aggregate_type: aggregate_type.clone(),
                    aggregate_id: aggregate_id.clone(),
                    sequence: current_sequence,
                    event_type: event.event_type(),
                    event_version: event.event_version(),
                    payload: json!(event).to_string(),
                }
            })
            .collect()
    }
}

impl<A: TAggregate> TEventStore<A> for SqlRepository<A> {
    async fn load_events(&self, agg_id: &str) -> Result<Vec<EventEnvolope>, String> {
        Ok(self
            .executor
            .table
            .iter()
            .filter_map(|rec| {
                if rec.aggregate_id() == agg_id {
                    Some(rec.envelope().clone())
                } else {
                    None
                }
            })
            .collect())
    }

    async fn load_aggregate(&self, aggregate_id: &str) -> Result<A, String> {
        let events = self.load_events(aggregate_id).await?;
        let mut aggregate = A::default();
        // current sequence should be the same as the number of events you can fetch
        let current_sequence = events.len() as i64;
        events
            .into_iter()
            .for_each(|event| aggregate.apply(from_str(&event.payload).unwrap()));
        aggregate.set_sequence(current_sequence);
        Ok(aggregate)
    }

    async fn commit(&mut self, aggregate: &A) -> Result<(), String> {
        let events = Self::extract_events(aggregate);
        if events.is_empty() {
            return Ok(());
        }

        self.executor
            .table
            .extend(events.into_iter().map(|envelope| EventTable {
                envelope,
                timestamp: Utc::now(),
            }));
        Ok(())
    }
}
