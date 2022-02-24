use std::str::FromStr;

use bigdecimal::BigDecimal;

use super::{Account, Bank, Transaction};

#[test]
fn it_handles_deposits() {
    // Arrange
    let mut bank = Bank::default();
    apply_test_deposits(&mut bank);
    let expected = [
        Account {
            client: 1,
            available: BigDecimal::from_str("400.2800").unwrap(),
            held: BigDecimal::default(),
            locked: false,
        },
        Account {
            client: 2,
            available: BigDecimal::from_str("672.6200").unwrap(),
            held: BigDecimal::default(),
            locked: false,
        },
    ];
    // Act
    let mut actual = bank.balances().collect::<Vec<_>>();
    actual.sort_by_key(|v| v.client);
    // Assert
    for (expected, actual) in expected.iter().zip(actual) {
        assert_eq!(*expected, *actual);
    }
}

#[test]
fn it_handles_a_withdrawal() {
    let mut bank = Bank::default();
    apply_test_deposits(&mut bank);
    bank.apply(Transaction::Withdrawal {
        client: 2,
        tx: 2000,
        amount: BigDecimal::from_str("72.6052").unwrap(),
        disputed: false,
    });
    let expected = [
        Account {
            client: 1,
            available: BigDecimal::from_str("400.2800").unwrap(),
            held: BigDecimal::default(),
            locked: false,
        },
        Account {
            client: 2,
            available: BigDecimal::from_str("600.0148").unwrap(),
            held: BigDecimal::default(),
            locked: false,
        },
    ];
    // Act
    let mut actual = bank.balances().collect::<Vec<_>>();
    actual.sort_by_key(|v| v.client);
    // Assert
    for (expected, actual) in expected.iter().zip(actual) {
        assert_eq!(*expected, *actual);
    }
}

#[test]
fn it_handles_a_dispute_of_a_deposit() {
    let mut bank = Bank::default();
    apply_test_deposits(&mut bank);
    bank.apply(Transaction::Dispute {
        client: 2,
        referenced_tx: 200,
    });
    let expected = [
        Account {
            client: 1,
            available: BigDecimal::from_str("400.2800").unwrap(),
            held: BigDecimal::default(),
            locked: false,
        },
        Account {
            client: 2,
            available: BigDecimal::from_str("200.0022").unwrap(),
            held: BigDecimal::from_str("472.6178").unwrap(),
            locked: false,
        },
    ];
    // Act
    let mut actual = bank.balances().collect::<Vec<_>>();
    actual.sort_by_key(|v| v.client);
    // Assert
    for (expected, actual) in expected.iter().zip(actual) {
        assert_eq!(*expected, *actual);
    }
}

#[test]
fn it_handles_a_dispute_of_a_withdrawal() {
    let mut bank = Bank::default();
    apply_test_deposits(&mut bank);
    bank.apply(Transaction::Withdrawal {
        client: 2,
        tx: 2000,
        amount: BigDecimal::from_str("72.6052").unwrap(),
        disputed: false,
    });
    bank.apply(Transaction::Dispute {
        client: 2,
        referenced_tx: 2000,
    });
    let expected = [
        Account {
            client: 1,
            available: BigDecimal::from_str("400.2800").unwrap(),
            held: BigDecimal::default(),
            locked: false,
        },
        Account {
            client: 2,
            available: BigDecimal::from_str("672.62").unwrap(),
            held: BigDecimal::from_str("-72.6052").unwrap(),
            locked: false,
        },
    ];
    // Act
    let mut actual = bank.balances().collect::<Vec<_>>();
    actual.sort_by_key(|v| v.client);
    // Assert
    for (expected, actual) in expected.iter().zip(actual) {
        assert_eq!(*expected, *actual);
    }
}

#[test]
fn it_handles_a_resolve_of_a_disputed_deposit() {
    let mut bank = Bank::default();
    apply_test_deposits(&mut bank);
    bank.apply(Transaction::Dispute {
        client: 2,
        referenced_tx: 200,
    });
    bank.apply(Transaction::Resolve {
        client: 2,
        referenced_tx: 200,
    });
    let expected = [
        Account {
            client: 1,
            available: BigDecimal::from_str("400.2800").unwrap(),
            held: BigDecimal::default(),
            locked: false,
        },
        Account {
            client: 2,
            available: BigDecimal::from_str("672.62").unwrap(),
            held: BigDecimal::default(),
            locked: false,
        },
    ];
    // Act
    let mut actual = bank.balances().collect::<Vec<_>>();
    actual.sort_by_key(|v| v.client);
    // Assert
    for (expected, actual) in expected.iter().zip(actual) {
        assert_eq!(*expected, *actual);
    }
}

#[test]
fn it_handles_a_resolve_of_a_disputed_withdrawal() {
    let mut bank = Bank::default();
    apply_test_deposits(&mut bank);
    bank.apply(Transaction::Withdrawal {
        client: 2,
        tx: 2000,
        amount: BigDecimal::from_str("72.6052").unwrap(),
        disputed: false,
    });
    bank.apply(Transaction::Dispute {
        client: 2,
        referenced_tx: 2000,
    });
    bank.apply(Transaction::Resolve {
        client: 2,
        referenced_tx: 2000,
    });
    let expected = [
        Account {
            client: 1,
            available: BigDecimal::from_str("400.2800").unwrap(),
            held: BigDecimal::default(),
            locked: false,
        },
        Account {
            client: 2,
            available: BigDecimal::from_str("600.0148").unwrap(),
            held: BigDecimal::default(),
            locked: false,
        },
    ];
    // Act
    let mut actual = bank.balances().collect::<Vec<_>>();
    actual.sort_by_key(|v| v.client);
    // Assert
    for (expected, actual) in expected.iter().zip(actual) {
        assert_eq!(*expected, *actual);
    }
}

#[test]
fn it_handles_a_chargeback_of_a_disputed_deposit() {
    let mut bank = Bank::default();
    apply_test_deposits(&mut bank);
    bank.apply(Transaction::Dispute {
        client: 2,
        referenced_tx: 200,
    });
    bank.apply(Transaction::ChargeBack {
        client: 2,
        referenced_tx: 200,
    });
    let expected = [
        Account {
            client: 1,
            available: BigDecimal::from_str("400.2800").unwrap(),
            held: BigDecimal::default(),
            locked: false,
        },
        Account {
            client: 2,
            available: BigDecimal::from_str("200.0022").unwrap(),
            held: BigDecimal::default(),
            locked: true,
        },
    ];
    // Act
    let mut actual = bank.balances().collect::<Vec<_>>();
    actual.sort_by_key(|v| v.client);
    // Assert
    for (expected, actual) in expected.iter().zip(actual) {
        assert_eq!(*expected, *actual);
    }
}

#[test]
fn it_handles_a_chargeback_of_a_disputed_withdrawal() {
    let mut bank = Bank::default();
    apply_test_deposits(&mut bank);
    bank.apply(Transaction::Withdrawal {
        client: 2,
        tx: 2000,
        amount: BigDecimal::from_str("72.6052").unwrap(),
        disputed: false,
    });
    bank.apply(Transaction::Dispute {
        client: 2,
        referenced_tx: 2000,
    });
    bank.apply(Transaction::ChargeBack {
        client: 2,
        referenced_tx: 2000,
    });
    let expected = [
        Account {
            client: 1,
            available: BigDecimal::from_str("400.2800").unwrap(),
            held: BigDecimal::default(),
            locked: false,
        },
        Account {
            client: 2,
            available: BigDecimal::from_str("672.62").unwrap(),
            held: BigDecimal::default(),
            locked: true,
        },
    ];
    // Act
    let mut actual = bank.balances().collect::<Vec<_>>();
    actual.sort_by_key(|v| v.client);
    // Assert
    for (expected, actual) in expected.iter().zip(actual) {
        assert_eq!(*expected, *actual);
    }
}

#[test]
fn it_ignores_the_transaction_if_the_account_has_insufficient_funds_on_a_withdrawal() {
    let mut bank = Bank::default();
    apply_test_deposits(&mut bank);
    bank.apply(Transaction::Withdrawal {
        client: 2,
        tx: 2000,
        amount: BigDecimal::from_str("1000.00").unwrap(),
        disputed: false,
    });
    let expected = [
        Account {
            client: 1,
            available: BigDecimal::from_str("400.2800").unwrap(),
            held: BigDecimal::default(),
            locked: false,
        },
        Account {
            client: 2,
            available: BigDecimal::from_str("672.62").unwrap(),
            held: BigDecimal::default(),
            locked: false,
        },
    ];
    // Act
    let mut actual = bank.balances().collect::<Vec<_>>();
    actual.sort_by_key(|v| v.client);
    // Assert
    for (expected, actual) in expected.iter().zip(actual) {
        assert_eq!(*expected, *actual);
    }
}

#[test]
fn it_ignores_the_transaction_if_the_referenced_transaction_does_not_exist_on_a_dispute() {
    let mut bank = Bank::default();
    apply_test_deposits(&mut bank);
    bank.apply(Transaction::Dispute {
        client: 2,
        referenced_tx: 201,
    });
    let expected = [
        Account {
            client: 1,
            available: BigDecimal::from_str("400.2800").unwrap(),
            held: BigDecimal::default(),
            locked: false,
        },
        Account {
            client: 2,
            available: BigDecimal::from_str("672.62").unwrap(),
            held: BigDecimal::default(),
            locked: false,
        },
    ];
    // Act
    let mut actual = bank.balances().collect::<Vec<_>>();
    actual.sort_by_key(|v| v.client);
    // Assert
    for (expected, actual) in expected.iter().zip(actual) {
        assert_eq!(*expected, *actual);
    }
}

#[test]
fn it_ignores_the_transaction_if_the_referenced_transaction_is_already_disputed_on_a_dispute() {
    let mut bank = Bank::default();
    apply_test_deposits(&mut bank);
    bank.apply(Transaction::Dispute {
        client: 2,
        referenced_tx: 200,
    });
    bank.apply(Transaction::Dispute {
        client: 2,
        referenced_tx: 200,
    });
    let expected = [
        Account {
            client: 1,
            available: BigDecimal::from_str("400.2800").unwrap(),
            held: BigDecimal::default(),
            locked: false,
        },
        Account {
            client: 2,
            available: BigDecimal::from_str("200.0022").unwrap(),
            held: BigDecimal::from_str("472.6178").unwrap(),
            locked: false,
        },
    ];
    // Act
    let mut actual = bank.balances().collect::<Vec<_>>();
    actual.sort_by_key(|v| v.client);
    // Assert
    for (expected, actual) in expected.iter().zip(actual) {
        assert_eq!(*expected, *actual);
    }
}

#[test]
fn it_ignores_the_transaction_if_the_referenced_transaction_does_not_exist_on_a_resolve() {
    let mut bank = Bank::default();
    apply_test_deposits(&mut bank);
    bank.apply(Transaction::Dispute {
        client: 2,
        referenced_tx: 200,
    });
    bank.apply(Transaction::Resolve {
        client: 2,
        referenced_tx: 201,
    });
    let expected = [
        Account {
            client: 1,
            available: BigDecimal::from_str("400.2800").unwrap(),
            held: BigDecimal::default(),
            locked: false,
        },
        Account {
            client: 2,
            available: BigDecimal::from_str("200.0022").unwrap(),
            held: BigDecimal::from_str("472.6178").unwrap(),
            locked: false,
        },
    ];
    // Act
    let mut actual = bank.balances().collect::<Vec<_>>();
    actual.sort_by_key(|v| v.client);
    // Assert
    for (expected, actual) in expected.iter().zip(actual) {
        assert_eq!(*expected, *actual);
    }
}

#[test]
fn it_ignores_the_transaction_if_the_referenced_transaction_is_not_under_dispute_on_a_resolve() {
    let mut bank = Bank::default();
    apply_test_deposits(&mut bank);
    bank.apply(Transaction::Dispute {
        client: 2,
        referenced_tx: 200,
    });
    bank.apply(Transaction::Resolve {
        client: 2,
        referenced_tx: 2,
    });
    let expected = [
        Account {
            client: 1,
            available: BigDecimal::from_str("400.2800").unwrap(),
            held: BigDecimal::default(),
            locked: false,
        },
        Account {
            client: 2,
            available: BigDecimal::from_str("200.0022").unwrap(),
            held: BigDecimal::from_str("472.6178").unwrap(),
            locked: false,
        },
    ];
    // Act
    let mut actual = bank.balances().collect::<Vec<_>>();
    actual.sort_by_key(|v| v.client);
    // Assert
    for (expected, actual) in expected.iter().zip(actual) {
        assert_eq!(*expected, *actual);
    }
}

#[test]
fn it_ignores_the_transaction_if_the_referenced_transaction_does_not_exist_on_a_chargeback() {
    let mut bank = Bank::default();
    apply_test_deposits(&mut bank);
    bank.apply(Transaction::Dispute {
        client: 2,
        referenced_tx: 200,
    });
    bank.apply(Transaction::ChargeBack {
        client: 2,
        referenced_tx: 201,
    });
    let expected = [
        Account {
            client: 1,
            available: BigDecimal::from_str("400.2800").unwrap(),
            held: BigDecimal::default(),
            locked: false,
        },
        Account {
            client: 2,
            available: BigDecimal::from_str("200.0022").unwrap(),
            held: BigDecimal::from_str("472.6178").unwrap(),
            locked: false,
        },
    ];
    // Act
    let mut actual = bank.balances().collect::<Vec<_>>();
    actual.sort_by_key(|v| v.client);
    // Assert
    for (expected, actual) in expected.iter().zip(actual) {
        assert_eq!(*expected, *actual);
    }
}

#[test]
fn it_ignores_the_transaction_if_the_referenced_transaction_is_not_under_dispute_on_a_chargeback() {
    let mut bank = Bank::default();
    apply_test_deposits(&mut bank);
    bank.apply(Transaction::Dispute {
        client: 2,
        referenced_tx: 200,
    });
    bank.apply(Transaction::ChargeBack {
        client: 2,
        referenced_tx: 2,
    });
    let expected = [
        Account {
            client: 1,
            available: BigDecimal::from_str("400.2800").unwrap(),
            held: BigDecimal::default(),
            locked: false,
        },
        Account {
            client: 2,
            available: BigDecimal::from_str("200.0022").unwrap(),
            held: BigDecimal::from_str("472.6178").unwrap(),
            locked: false,
        },
    ];
    // Act
    let mut actual = bank.balances().collect::<Vec<_>>();
    actual.sort_by_key(|v| v.client);
    // Assert
    for (expected, actual) in expected.iter().zip(actual) {
        assert_eq!(*expected, *actual);
    }
}

fn apply_test_deposits(bank: &mut Bank) {
    bank.apply(Transaction::Deposit {
        client: 1,
        tx: 1,
        amount: BigDecimal::from_str("100.0057").unwrap(),
        disputed: false,
    });
    bank.apply(Transaction::Deposit {
        client: 2,
        tx: 1,
        amount: BigDecimal::from_str("200.0022").unwrap(),
        disputed: false,
    });
    bank.apply(Transaction::Deposit {
        client: 1,
        tx: 100,
        amount: BigDecimal::from_str("300.2743").unwrap(),
        disputed: false,
    });
    bank.apply(Transaction::Deposit {
        client: 2,
        tx: 200,
        amount: BigDecimal::from_str("472.6178").unwrap(),
        disputed: false,
    });
}
