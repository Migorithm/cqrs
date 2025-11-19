use crate::event::TEvent;

pub trait TAggregate: Default + Sync + Send + 'static {
    type Event: TEvent;

    fn sequence(&self) -> i64;
    fn set_sequence(&mut self, version: i64);
    fn aggregate_type(&self) -> String;
    fn aggregate_id(&self) -> String;
    fn apply(&mut self, event: Self::Event);

    fn raise_event(&mut self, event: Self::Event);
    fn events(&self) -> &Vec<Self::Event>;
}
