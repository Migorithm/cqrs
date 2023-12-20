use crate::services::BankAccountServices;

use super::events::BankAccountEvent;
use super::BankAccountCommand::*;
use super::*;

use cqrs_es::test::TestFramework;

type AccountTestFramework = TestFramework<BankAccount>;

#[test]
fn test_deposit_money() {
    let expected = BankAccountEvent::CustomerDepositedMoney {
        amount: 200.0,
        balance: 200.0,
    };

    AccountTestFramework::with(BankAccountServices)
        .given_no_previous_events()
        .when(DepositMoney { amount: 200.0 })
        .then_expect_events(vec![expected]);
}

#[test]
fn test_deposit_money_with_balance() {
    let previous = BankAccountEvent::CustomerDepositedMoney {
        amount: 200.0,
        balance: 200.0,
    };
    let expected = BankAccountEvent::CustomerDepositedMoney {
        amount: 200.0,
        balance: 400.0,
    };

    AccountTestFramework::with(BankAccountServices)
        .given(vec![previous])
        .when(DepositMoney { amount: 200.0 })
        .then_expect_events(vec![expected]);
}

#[test]
fn test_withdraw_money() {
    let previous = BankAccountEvent::CustomerDepositedMoney {
        amount: 200.0,
        balance: 200.0,
    };
    let expected = BankAccountEvent::CustomerWithdrewCash {
        amount: 100.0,
        balance: 100.0,
    };

    AccountTestFramework::with(BankAccountServices)
        .given(vec![previous])
        .when(WithdrawMoney { amount: 100.0 })
        .then_expect_events(vec![expected]);
}

#[test]
fn test_withdraw_money_funds_not_available() {
    AccountTestFramework::with(BankAccountServices)
        .given_no_previous_events()
        .when(BankAccountCommand::WithdrawMoney { amount: 200.0 })
        .then_expect_error(BankAccountError("funds not available".to_string()));
}
