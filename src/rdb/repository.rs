use std::{collections::HashMap, marker::PhantomData};

use chrono::{DateTime, Utc};
use serde_json::{from_str, from_value, json};

use crate::{
    aggregate::{TAggregateES, TAggregateMetadata},
    event::{EventEnvolope, TEvent},
    event_store::TEventStore,
};

struct InMemoryDB {
    table: HashMap<Kind, Vec<Table>>,
}
#[derive(PartialEq, Eq)]
enum Kind {
    Event,
    Snapshot,
}

enum Table {
    Event {
        envelope: EventEnvolope,
        timestamp: DateTime<Utc>,
    },
    Snapshot {
        aggregate_type: String,
        aggregate_id: String,
        last_sequnece: u64,
        current_snapshot: u64,
        payload: String,
        timestamp: DateTime<Utc>,
    },
}
impl Table {
    fn aggregate_id(&self) -> &str {
        match self {
            Table::Event { envelope, .. } => &envelope.aggregate_id,
            Table::Snapshot { aggregate_id, .. } => &aggregate_id,
        }
    }

    fn envelope(&self) -> Option<&EventEnvolope> {
        match self {
            Table::Event { envelope, .. } => Some(envelope),
            _ => None,
        }
    }
}

pub struct SqlRepository<A: TAggregateES> {
    executor: InMemoryDB,
    _phantom: PhantomData<A>,
}

impl<A: TAggregateES + TAggregateMetadata> SqlRepository<A> {
    pub fn new(executor: InMemoryDB) -> Self {
        Self {
            executor,
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

impl<A: TAggregateES + TAggregateMetadata> TEventStore<A> for SqlRepository<A> {
    async fn load_events(&self, agg_id: &str) -> Result<Vec<EventEnvolope>, String> {
        Ok(self
            .executor
            .table
            .iter()
            .map(|(_, table)| {
                let mut events: Vec<EventEnvolope> = vec![];
                for rec in table {
                    if rec.aggregate_id() == agg_id {
                        events.push(rec.envelope().cloned().unwrap())
                    }
                }
                events
            })
            .flatten()
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

    async fn commit(&self, aggregate: &A) -> Result<(), String> {
        let events = Self::extract_events(aggregate);
        if events.is_empty() {
            return Ok(());
        }
        // prepare_bulk_operation!(
        //     &events,
        //     aggregate_type: String,
        //     aggregate_id: String,
        //     sequence:i64,
        //     event_type: String,
        //     event_version: String,
        //     payload: Value
        // );
        // sqlx::query(
        //     r#"
        //     INSERT INTO events (
        //         aggregate_type ,
        //         aggregate_id   ,
        //         sequence       ,
        //         event_type     ,
        //         event_version  ,
        //         payload
        //     )
        //     VALUES (
        //         UNNEST($1::text[]),
        //         UNNEST($2::text[]),
        //         UNNEST($3::bigint[]),
        //         UNNEST($4::text[]),
        //         UNNEST($5::text[]),
        //         UNNEST($6::jsonb[])
        //     )
        //     "#,
        // )
        // .bind(&aggregate_type)
        // .bind(&aggregate_id)
        // .bind(&sequence)
        // .bind(&event_type)
        // .bind(&event_version)
        // .bind(&payload)
        // .execute(self.executor.read().await.connection())
        // .await
        // .unwrap();
        Ok(())
    }
}
