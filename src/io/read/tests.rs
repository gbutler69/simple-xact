use std::str::FromStr;

use bigdecimal::BigDecimal;

use crate::bank::Transaction;

#[test]
fn it_reads_a_properly_formatted_csv_with_all_transaction_types() {
    // Arrange
    let input = stringreader::StringReader::new(
        "type,client,tx,amount\n\
              deposit,1,1,1.0\n\
              deposit,2,2,2.0\n\
              deposit,1,3,2.0\n\
              withdrawal,1,4,1.5\n\
              withdrawal,2,5,3.0\n\
              dispute,2,2\n\
              resolve,2,2\n\
              dispute,1,1\n\
              chargeback,1,1\n\
             ",
    );
    let expected = [
        Transaction::Deposit {
            client: 1,
            tx: 1,
            amount: BigDecimal::from_str("1.0").unwrap(),
            disputed: false,
        },
        Transaction::Deposit {
            client: 2,
            tx: 2,
            amount: BigDecimal::from_str("2.0").unwrap(),
            disputed: false,
        },
        Transaction::Deposit {
            client: 1,
            tx: 3,
            amount: BigDecimal::from_str("2.0").unwrap(),
            disputed: false,
        },
        Transaction::Withdrawal {
            client: 1,
            tx: 4,
            amount: BigDecimal::from_str("1.5").unwrap(),
            disputed: false,
        },
        Transaction::Withdrawal {
            client: 2,
            tx: 5,
            amount: BigDecimal::from_str("3.0").unwrap(),
            disputed: false,
        },
    ];
    // Act
    let mut actual = Vec::new();
    super::processs_transactions_from_csv(input, |trans| actual.push(trans.unwrap()));
    // Assert
    for (expected, actual) in expected.into_iter().zip(actual) {
        assert_eq!(expected, actual);
    }
}

#[test]
fn it_reads_a_csv_file_containing_an_invalid_amount_but_processes_all_other_transactions_normally()
{
    // Arrange
    let input = stringreader::StringReader::new(
        "type,client,tx,amount\n\
              deposit,1,1,1.0\n\
              deposit,2,2,2.0\n\
              deposit,1,3,2.x\n\
              withdrawal,1,4,1.5\n\
              withdrawal,2,5,3.0\n\
              dispute,2,2\n\
              resolve,2,2\n\
              dispute,1,1\n\
              chargeback,1,1\n\
             ",
    );
    let expected = [
        Transaction::Deposit {
            client: 1,
            tx: 1,
            amount: BigDecimal::from_str("1.0").unwrap(),
            disputed: false,
        },
        Transaction::Deposit {
            client: 2,
            tx: 2,
            amount: BigDecimal::from_str("2.0").unwrap(),
            disputed: false,
        },
        Transaction::Withdrawal {
            client: 1,
            tx: 4,
            amount: BigDecimal::from_str("1.5").unwrap(),
            disputed: false,
        },
        Transaction::Withdrawal {
            client: 2,
            tx: 5,
            amount: BigDecimal::from_str("3.0").unwrap(),
            disputed: false,
        },
    ];
    // Act
    let mut actual = Vec::new();
    super::processs_transactions_from_csv(input, |trans| {
        if let Ok(trans) = trans {
            actual.push(trans)
        }
    });
    // Assert
    for (expected, actual) in expected.into_iter().zip(actual) {
        assert_eq!(expected, actual);
    }
}

#[test]
fn it_reads_a_csv_file_containing_an_invalid_transaction_type_but_processes_all_other_transactions_normally(
) {
    // Arrange
    let input = stringreader::StringReader::new(
        "type,client,tx,amount\n\
              deposit,1,1,1.0\n\
              deposit,2,2,2.0\n\
              deposits,1,3,2.0\n\
              withdrawal,1,4,1.5\n\
              withdrawal,2,5,3.0\n\
              dispute,2,2\n\
              resolve,2,2\n\
              dispute,1,1\n\
              chargeback,1,1\n\
             ",
    );
    let expected = [
        Transaction::Deposit {
            client: 1,
            tx: 1,
            amount: BigDecimal::from_str("1.0").unwrap(),
            disputed: false,
        },
        Transaction::Deposit {
            client: 2,
            tx: 2,
            amount: BigDecimal::from_str("2.0").unwrap(),
            disputed: false,
        },
        Transaction::Withdrawal {
            client: 1,
            tx: 4,
            amount: BigDecimal::from_str("1.5").unwrap(),
            disputed: false,
        },
        Transaction::Withdrawal {
            client: 2,
            tx: 5,
            amount: BigDecimal::from_str("3.0").unwrap(),
            disputed: false,
        },
    ];
    // Act
    let mut actual = Vec::new();
    super::processs_transactions_from_csv(input, |trans| {
        if let Ok(trans) = trans {
            actual.push(trans)
        }
    });
    // Assert
    for (expected, actual) in expected.into_iter().zip(actual) {
        assert_eq!(expected, actual);
    }
}

#[test]
fn it_reads_a_csv_file_containing_malformed_transactions_of_various_kinds_but_processes_all_properly_formed_transactions_correctly(
) {
    // Arrange
    let input = stringreader::StringReader::new(
        "type,    client, tx, amount\n\
              deposit, 1,      1,  1.0\n\
              deposit,1.1,1,1.0\n\
              deposit,2,2,2.0\n\
              deposit,2,2.x,2.0\n\
              deposit,1,3,2.0\n\
              1,3,2.0\n\
              withdrawal  ,1,4,1.5\n\
              withdrawal  ,2,5,3.0\n\
              withdrawal,2,5,3.0,5\n\
              dispute,2,2\n\
              dispute,2\n\
              resolve,2,2\n\
              reslve,2,2\n\
              dispute,1,1\n\
              dispute,1,,\n\
              chargeback,1,1\n\
              1,1,chargeback\n\
             ",
    );
    let expected = [
        Transaction::Deposit {
            client: 1,
            tx: 1,
            amount: BigDecimal::from_str("1.0").unwrap(),
            disputed: false,
        },
        Transaction::Deposit {
            client: 2,
            tx: 2,
            amount: BigDecimal::from_str("2.0").unwrap(),
            disputed: false,
        },
        Transaction::Deposit {
            client: 1,
            tx: 3,
            amount: BigDecimal::from_str("2.0").unwrap(),
            disputed: false,
        },
        Transaction::Withdrawal {
            client: 1,
            tx: 4,
            amount: BigDecimal::from_str("1.5").unwrap(),
            disputed: false,
        },
        Transaction::Withdrawal {
            client: 2,
            tx: 5,
            amount: BigDecimal::from_str("3.0").unwrap(),
            disputed: false,
        },
    ];
    // Act
    let mut actual = Vec::new();
    super::processs_transactions_from_csv(input, |trans| {
        if let Ok(trans) = trans {
            actual.push(trans)
        }
    });
    // Assert
    for (expected, actual) in expected.into_iter().zip(actual) {
        assert_eq!(expected, actual);
    }
}
