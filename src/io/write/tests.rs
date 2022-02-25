use std::{error, str::FromStr};

use bigdecimal::BigDecimal;

use crate::bank::Account;

#[test]
fn it_writes_all_account_amount_and_locked_combinations() -> Result<(), Box<dyn error::Error>> {
    // Arrange
    let acct1 = Account {
        client: 1,
        available: BigDecimal::from_str("0.0").unwrap(),
        held: BigDecimal::from_str("0.0").unwrap(),
        locked: false,
    };
    let acct2 = Account {
        client: 2,
        available: BigDecimal::from_str("10.0").unwrap(),
        held: BigDecimal::from_str("0.0").unwrap(),
        locked: false,
    };
    let acct3 = Account {
        client: 3,
        available: BigDecimal::from_str("0.0").unwrap(),
        held: BigDecimal::from_str("20.0").unwrap(),
        locked: false,
    };
    let acct4 = Account {
        client: 4,
        available: BigDecimal::from_str("10.0").unwrap(),
        held: BigDecimal::from_str("20.0").unwrap(),
        locked: false,
    };
    let acct5 = Account {
        client: 5,
        available: BigDecimal::from_str("5.0").unwrap(),
        held: BigDecimal::from_str("0.0").unwrap(),
        locked: true,
    };
    let accounts = [&acct1, &acct2, &acct3, &acct4, &acct5].into_iter();
    let expected = "client,available,held,total,locked\n\
                         1,0.0,0.0,0.0,false\n\
                         2,10.0,0.0,10.0,false\n\
                         3,0.0,20.0,20.0,false\n\
                         4,10.0,20.0,30.0,false\n\
                         5,5.0,0.0,5.0,true\n\
                        ";
    // Act
    let mut output = Vec::<u8>::new();
    super::write_accounts_to_csv(&mut output, accounts)?;
    let actual = String::from_utf8(output)?;
    // Assert
    assert_eq!(expected, actual);
    Ok(())
}
