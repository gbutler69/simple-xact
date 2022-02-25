use std::{
    error,
    fs::File,
    io::{stdout, BufReader},
};

use simple_xact::{
    bank::Bank,
    io::{read::processs_transactions_from_csv, write::write_accounts_to_csv},
};

use clap::StructOpt;

mod cli;

pub fn main() -> Result<(), Box<dyn error::Error>> {
    let reader = BufReader::new(File::open(cli::Arguments::parse().path)?);
    let mut bank = Bank::default();
    processs_transactions_from_csv(reader, |transaction| {
        if let Ok(transaction) = transaction {
            bank.apply(transaction);
        }
    });
    write_accounts_to_csv(&mut stdout(), bank.balances())?;
    Ok(())
}
