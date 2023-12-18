use async_trait::async_trait;
use cqrs_es::Aggregate;
use serde::{Deserialize, Serialize};

use crate::services::BankAccountServices;

use self::{commands::BankAccountCommand, error::BankAccountError, events::BankAccountEvent};

pub mod commands;
pub mod error;
pub mod events;

#[derive(Serialize, Default, Deserialize)]
pub struct BankAccount {
    opened: bool,
    // this is a floating point for our example, don't do this IRL
    balance: f64,
}

#[async_trait]
impl Aggregate for BankAccount {
    type Command = BankAccountCommand;
    type Event = BankAccountEvent;
    type Error = BankAccountError;
    type Services = BankAccountServices;

    fn aggregate_type() -> String {
        "Account".to_string()
    }

    async fn handle(
        &self,
        command: Self::Command,
        services: &Self::Services,
    ) -> Result<Vec<Self::Event>, Self::Error> {
        todo!()
    }

    // Note that the apply function has no return value.
    // The act of applying an event is simply bookkeeping, the action has already taken place.
    fn apply(&mut self, event: Self::Event) {
        match event {
            Self::Event::AccountOpened { .. } => self.opened = true,
            Self::Event::CustomerDepositedMoney { amount: _, balance } => self.balance = balance,
            Self::Event::CustomerWithdrewCash { amount: _, balance } => {
                self.balance = balance;
            }
            BankAccountEvent::CustomerWroteCheck { balance, .. } => self.balance = balance,
        }
    }
}
