use std::process::Command;

use assert_cmd::prelude::*;
use predicates::prelude::predicate::str::contains;
use predicates::prelude::*;

#[test]
fn it_works_with_the_test_input_file() -> Result<(), Box<dyn std::error::Error>> {
    Command::cargo_bin("simple-xact")?
        .arg("test-data/test-cli/input")
        .assert()
        .success()
        .stdout(
            contains("client,available,held,total,locked")
                .and(contains("1,6899.80,0,6899.80,false"))
                .and(contains("2,11000.00,0.00,11000.00,false"))
                .and(contains("3,11899.60,3000.00,14899.60,false"))
                .and(contains("4,7899.30,0.00,7899.30,true")),
        );
    Ok(())
}

#[test]
fn it_errors_with_file_not_found_if_file_does_not_exist() -> Result<(), Box<dyn std::error::Error>>
{
    Command::cargo_bin("simple-xact")?
        .arg("test-data/test-cli/input-DOES-NOT-EXIST")
        .assert()
        .failure()
        .stderr(contains("No such file or directory"));
    Ok(())
}
