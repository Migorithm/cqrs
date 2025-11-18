use crate::aggregate::TAggregateMetadata;

use crate::{aggregate::TAggregateES, event::TEvent};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Account {
    pub(crate) id: i64,
    pub(crate) name: String,
    pub(crate) hashed_password: String,
    pub(crate) version: i64,
    events: Vec<AccountEvent>,
}
impl Account {
    pub(crate) fn create_account(cmd: CreateAccount) -> Self {
        let mut aggregate = Account {
            name: cmd.email.clone(),
            hashed_password: cmd.password + "_hashed",
            ..Default::default()
        };

        aggregate.raise_event(AccountEvent::AccountCreated {
            name: cmd.email,
            hashed_password: aggregate.hashed_password.clone(),
            id: aggregate.id,
        });
        aggregate
    }

    fn verify_password(&self, plain_text: &str) -> Result<(), String> {
        Ok(())
    }
    pub(crate) fn sign_in(&mut self, cmd: SignInAccount) -> Result<(), String> {
        self.verify_password(&cmd.password)?;
        self.raise_event(AccountEvent::SignedIn {
            email: cmd.email,
            password: cmd.password,
        });
        Ok(())
    }
}

impl TAggregateES for Account {
    type Event = AccountEvent;

    fn apply(&mut self, event: Self::Event) {
        match event {
            Self::Event::AccountCreated {
                id,
                name,
                hashed_password,
            } => {
                *self = Account {
                    id,
                    name,
                    hashed_password,
                    ..Default::default()
                }
            }
            Self::Event::SignedIn { .. } => {}
        }
    }

    fn raise_event(&mut self, event: Self::Event) {
        self.events.push(event)
    }
    fn events(&self) -> &Vec<Self::Event> {
        &self.events
    }
}

impl TAggregateMetadata for Account {
    fn aggregate_type(&self) -> String {
        "Account".to_string()
    }
    fn aggregate_id(&self) -> String {
        self.id.to_string()
    }
    fn sequence(&self) -> i64 {
        self.version
    }
    fn set_sequence(&mut self, version: i64) {
        self.version = version
    }
}

#[derive(Deserialize, Clone)]
pub struct CreateAccount {
    pub email: String,
    pub password: String,
}
#[derive(Deserialize, Clone)]
pub struct SignInAccount {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize, PartialEq, Clone, Serialize)]
pub enum AccountEvent {
    AccountCreated {
        id: i64,
        name: String,
        hashed_password: String,
    },
    SignedIn {
        email: String,
        password: String,
    },
}

impl TEvent for AccountEvent {
    fn event_type(&self) -> String {
        let event_type_in_str = match self {
            Self::AccountCreated { .. } => "AccountCreated",
            Self::SignedIn { .. } => "SignIn",
        };
        event_type_in_str.to_string()
    }
    fn event_version(&self) -> String {
        "0.1".to_string()
    }

    fn aggregate_type(&self) -> String {
        "Account".to_string()
    }
}
