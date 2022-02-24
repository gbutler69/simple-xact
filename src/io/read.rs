use std::{error, io};

use bigdecimal::BigDecimal;
use serde::Deserialize;

use crate::bank::Transaction;

#[cfg(test)]
mod tests;

pub fn processs_transactions_from_csv<ProcessCallback>(
    reader: impl io::Read,
    mut process: ProcessCallback,
) where
    ProcessCallback: FnMut(Result<Transaction, Box<dyn error::Error>>),
{
    for record in csv::ReaderBuilder::new()
        .flexible(true)
        .from_reader(reader)
        .deserialize::<TransactionRecord>()
    {
        process(match record {
            Ok(rec) => Transaction::try_from(rec),
            Err(err) => Err(err.into()),
        });
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct TransactionRecord {
    #[serde(rename = "type")]
    trans_type: String,
    client: u16,
    tx: u32,
    amount: Option<String>,
}

impl TryFrom<TransactionRecord> for Transaction {
    type Error = Box<dyn error::Error>;

    fn try_from(rec: TransactionRecord) -> Result<Self, Self::Error> {
        match rec.trans_type.as_str() {
            "deposit" => try_deposit_from(rec.client, rec.tx, rec.amount),
            "withdrawal" => try_withdrawal_from(rec.client, rec.tx, rec.amount),
            "dispute" => try_dispute_from(rec.client, rec.tx),
            "resolve" => try_resolve_from(rec.client, rec.tx),
            "chargeback" => try_chargeback_from(rec.client, rec.tx),
            _ => Err(Box::new(io::Error::new(
                io::ErrorKind::InvalidData,
                "Invalid transaction type",
            ))),
        }
    }
}

fn try_deposit_from(
    client: u16,
    tx: u32,
    amount: Option<String>,
) -> Result<Transaction, Box<dyn error::Error>> {
    Ok(Transaction::Deposit {
        client,
        tx,
        amount: parse_amount(amount)?,
        disputed: false,
    })
}

fn try_withdrawal_from(
    client: u16,
    tx: u32,
    amount: Option<String>,
) -> Result<Transaction, Box<dyn error::Error>> {
    Ok(Transaction::Withdrawal {
        client,
        tx,
        amount: parse_amount(amount)?,
        disputed: false,
    })
}

fn try_dispute_from(client: u16, tx: u32) -> Result<Transaction, Box<dyn error::Error>> {
    Ok(Transaction::Dispute {
        client,
        referenced_tx: tx,
    })
}

fn try_resolve_from(client: u16, tx: u32) -> Result<Transaction, Box<dyn error::Error>> {
    Ok(Transaction::Resolve {
        client,
        referenced_tx: tx,
    })
}

fn try_chargeback_from(client: u16, tx: u32) -> Result<Transaction, Box<dyn error::Error>> {
    Ok(Transaction::ChargeBack {
        client,
        referenced_tx: tx,
    })
}

fn parse_amount(amount: Option<String>) -> Result<BigDecimal, bigdecimal::ParseBigDecimalError> {
    match amount {
        Some(amount) => amount.parse::<BigDecimal>(),
        _ => todo!(),
    }
}
