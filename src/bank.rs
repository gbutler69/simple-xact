use std::collections::HashMap;

use bigdecimal::BigDecimal;

#[cfg(test)]
mod tests;

#[derive(Default)]
pub struct Bank {
    accounts: HashMap<u16, (Account, HashMap<u32, Transaction>)>,
}

impl Bank {
    pub fn apply(&mut self, transaction: Transaction) {
        match transaction {
            Transaction::Deposit {
                client, tx, amount, ..
            } => self.apply_deposit(client, tx, amount),
            Transaction::Withdrawal {
                client, tx, amount, ..
            } => self.appy_withdrawal(client, tx, amount),
            Transaction::Dispute {
                client,
                referenced_tx,
                ..
            } => self.apply_dispute(client, referenced_tx),
            Transaction::Resolve {
                client,
                referenced_tx,
                ..
            } => self.apply_resolve(client, referenced_tx),
            Transaction::ChargeBack {
                client,
                referenced_tx,
                ..
            } => self.apply_chargeback(client, referenced_tx),
        }
    }

    pub fn balances(&self) -> impl Iterator<Item = &Account> {
        self.accounts.values().map(|(acct, _)| acct)
    }

    fn apply_deposit(&mut self, client: u16, tx: u32, amount: BigDecimal) {
        self.accounts
            .entry(client)
            .and_modify(|(acct, transactions)| acct_deposit(acct, transactions, tx, &amount))
            .or_insert_with(|| new_acct_deposit(client, tx, &amount));
    }

    fn appy_withdrawal(&mut self, client: u16, tx: u32, amount: BigDecimal) {
        self.accounts
            .entry(client)
            .and_modify(|(acct, transactions)| acct_withdrawal(acct, transactions, tx, amount));
    }

    fn apply_dispute(&mut self, client: u16, referenced_tx: u32) {
        self.accounts
            .entry(client)
            .and_modify(|(acct, transactions)| acct_dispute(acct, transactions, referenced_tx));
    }

    fn apply_resolve(&mut self, client: u16, referenced_tx: u32) {
        self.accounts
            .entry(client)
            .and_modify(|(acct, transactions)| acct_resolve(acct, transactions, referenced_tx));
    }

    fn apply_chargeback(&mut self, client: u16, referenced_tx: u32) {
        self.accounts
            .entry(client)
            .and_modify(|(acct, transactions)| acct_chargeback(acct, transactions, referenced_tx));
    }
}

fn acct_deposit(
    acct: &mut Account,
    transactions: &mut HashMap<u32, Transaction>,
    tx: u32,
    amount: &BigDecimal,
) {
    acct.available += amount;
    transactions.insert(
        tx,
        Transaction::Deposit {
            client: acct.client,
            tx,
            amount: amount.clone(),
            disputed: false,
        },
    );
}

fn new_acct_deposit(
    client: u16,
    tx: u32,
    amount: &BigDecimal,
) -> (Account, HashMap<u32, Transaction>) {
    (
        Account {
            client,
            available: amount.clone(),
            held: BigDecimal::default(),
            locked: false,
        },
        HashMap::from([(
            tx,
            Transaction::Deposit {
                client,
                tx,
                amount: amount.clone(),
                disputed: false,
            },
        )]),
    )
}

fn acct_withdrawal(
    acct: &mut Account,
    transactions: &mut HashMap<u32, Transaction>,
    tx: u32,
    amount: BigDecimal,
) {
    if acct.available >= amount {
        acct.available -= &amount;
        transactions.insert(
            tx,
            Transaction::Withdrawal {
                client: acct.client,
                tx,
                amount,
                disputed: false,
            },
        );
    }
}

fn acct_dispute(
    acct: &mut Account,
    transactions: &mut HashMap<u32, Transaction>,
    referenced_tx: u32,
) {
    match transactions.get_mut(&referenced_tx) {
        Some(Transaction::Deposit {
            disputed,
            ref amount,
            ..
        }) if !*disputed => {
            *disputed = true;
            acct.available -= amount;
            acct.held += amount;
        }
        Some(Transaction::Withdrawal {
            disputed,
            ref amount,
            ..
        }) if !*disputed => {
            *disputed = true;
            acct.available += amount;
            acct.held -= amount;
        }
        _ => (),
    };
}

fn acct_resolve(
    acct: &mut Account,
    transactions: &mut HashMap<u32, Transaction>,
    referenced_tx: u32,
) {
    match transactions.get_mut(&referenced_tx) {
        Some(Transaction::Deposit {
            disputed,
            ref amount,
            ..
        }) if *disputed => {
            *disputed = false;
            acct.available += amount;
            acct.held -= amount;
        }
        Some(Transaction::Withdrawal {
            disputed,
            ref amount,
            ..
        }) if *disputed => {
            *disputed = false;
            acct.available -= amount;
            acct.held += amount;
        }
        _ => (),
    };
}

fn acct_chargeback(
    acct: &mut Account,
    transactions: &mut HashMap<u32, Transaction>,
    referenced_tx: u32,
) {
    match transactions.get_mut(&referenced_tx) {
        Some(Transaction::Deposit {
            disputed,
            ref amount,
            ..
        }) if *disputed => {
            *disputed = false;
            acct.held -= amount;
            acct.locked = true;
        }
        Some(Transaction::Withdrawal {
            disputed,
            ref amount,
            ..
        }) if *disputed => {
            *disputed = false;
            acct.held += amount;
            acct.locked = true;
        }
        _ => (),
    };
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub enum Transaction {
    Deposit {
        client: u16,
        tx: u32,
        amount: BigDecimal,
        disputed: bool,
    },
    Withdrawal {
        client: u16,
        tx: u32,
        amount: BigDecimal,
        disputed: bool,
    },
    Dispute {
        client: u16,
        referenced_tx: u32,
    },
    Resolve {
        client: u16,
        referenced_tx: u32,
    },
    ChargeBack {
        client: u16,
        referenced_tx: u32,
    },
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub struct Account {
    pub client: u16,
    pub available: BigDecimal,
    pub held: BigDecimal,
    pub locked: bool,
}

impl Account {
    pub fn total(&self) -> BigDecimal {
        &self.available + &self.held
    }
}
