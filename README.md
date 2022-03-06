# Toy Payments Engine

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![build](https://github.com/acelletti/toy-payments-engine/actions/workflows/build.yaml/badge.svg)](https://github.com/acelletti/toy-payments-engine/actions)
[![codecov](https://codecov.io/gh/acelletti/toy-payments-engine/branch/main/graph/badge.svg?token=RP851V9JCZ)](https://codecov.io/gh/acelletti/toy-payments-engine)

A simple toy payments engine that reads a series of transactions from a CSV file, updates 
client accounts, handles disputes and chargebacks, and then outputs the state of clients 
accounts as a CSV file.

## Usage

To run the binary use:

```sh
cargo run -- <path to csv file>
```

The application will output the resulting CSV data to `stdout`.

To save the CSV result to file use:

```sh
cargo run -- <path to csv input> > <path to csv output>
```

Example:

```sh
cargo run -- transactions.csv > accounts.csv
```

## Formatting

This repository uses [rustfmt](https://github.com/rust-lang/rustfmt) to format Rust code 
according to style guidelines.

## Testing

Unit tests are provided for all of the application business logic.

The full breakdown of test coverage can be found on [Codecov.io](https://app.codecov.io/gh/acelletti/toy-payments-engine/).

To test the `main` entrypoint, the application has been tested manually with sample data from `sample/transactions.csv`.
