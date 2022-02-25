# simple-xact
Simple Bank-Like Transaction Processor in Rust

## Unit & Integration Tests

  * All logic is tested with thorough unit tests for the following modules in the simple-xact crate:
    * bank - src/bank/tests.rs
    * io::read - src/io/read/tests.rs
    * io::write - src/io/write/tests.rs
  * The CLI and whole binary application is tested with integration tests:
    * src/tests/test-cli.rs

## Error Handling

  * Individual transaction conversion, malformedness, and invalid transactions are handled by Result<Transaction,Error> and error transactions are ignored by the higher level logic
  * Global errors (such as an non-existent input file) propogate out and cause the CLI application to end with a failure exit status and print detailed error messages
