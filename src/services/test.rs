use cqrs_es::{mem_store::MemStore, CqrsFramework};

use crate::{
    domain::{commands::BankAccountCommand, BankAccount},
    services::{query::SimpleLoggingQuery, BankAccountServices},
};

#[tokio::test]
async fn test_event_store() {
    let event_store = MemStore::<BankAccount>::default();
    let query = SimpleLoggingQuery {};
    let cqrs = CqrsFramework::new(event_store, vec![Box::new(query)], BankAccountServices);

    let aggregate_id = "aggregate-instance-A";

    // deposit $1000
    cqrs.execute(
        aggregate_id,
        BankAccountCommand::DepositMoney { amount: 1000_f64 },
    )
    .await
    .unwrap();

    // write a check for $236.15
    cqrs.execute(
        aggregate_id,
        BankAccountCommand::WriteCheck {
            check_number: "1337".to_string(),
            amount: 236.15,
        },
    )
    .await
    .unwrap();
}
