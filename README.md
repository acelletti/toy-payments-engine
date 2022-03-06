# Toy Payments Engine

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

This repository uses [rustfmt](https://github.com/rust-lang/rustfmt) to format formatting 
Rust code according to style guidelines.

## Testing

Besides unit tests, this application has been tested with sample data from `sample/transactions.csv`.

The full breakdown of test coverage can be found on [Codecov.io](https://app.codecov.io/gh/acelletti/toy-payments-engine/).