use std::{error, io};

use serde::Serialize;

use crate::bank::Account;

#[cfg(test)]
mod tests;

pub fn write_accounts_to_csv(
    writer: &mut impl io::Write,
    accounts: impl Iterator<Item = Account>,
) -> Result<(), Box<dyn error::Error>> {
    let mut writer = csv::WriterBuilder::new().flexible(true).from_writer(writer);
    for account in accounts {
        writer.serialize(AccountRecord::from(account))?;
    }
    writer.flush()?;
    Ok(())
}

#[derive(Clone, Debug, Serialize)]
pub struct AccountRecord {
    client: u16,
    available: String,
    held: String,
    total: String,
    locked: bool,
}

impl From<Account> for AccountRecord {
    fn from(account: Account) -> Self {
        Self {
            client: account.client,
            available: account.available.to_string(),
            held: account.held.to_string(),
            total: account.total().to_string(),
            locked: account.locked,
        }
    }
}
