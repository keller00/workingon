# workingon

[![Build and test](https://github.com/keller00/workingon/actions/workflows/coverage.yml/badge.svg)](https://github.com/keller00/workingon/actions/workflows/coverage.yml)
[![codecov](https://codecov.io/github/keller00/workingon/graph/badge.svg?token=OFRWWDT7BT)](https://codecov.io/github/keller00/workingon)

Workingon is a way to manage TODOs right in your terminal with your own EDITOR.

## What makes workingon different?

TODOs are stored locally in SQLite and database migrations run automatically on startup.

## Typical usage

```shell
$ workingon add "Finish writing a better README for workingon"
TODO added successfully
$ workingon list                                                
9yszt Finish writing a better README for workingon
xuzrv Order batteries
$ workingon complete 9yszt
TODO was completed
```

## Usage (quick reference)

```shell
workingon add [<title>]                       # Add TODO
workingon list|ls [--open|--completed|--all]  # List TODOs (default: --open)
workingon show <id>                           # Show full TODO
workingon complete <id>                       # Mark as completed
workingon reopen <id>                         # Mark as open
workingon edit <id>                           # Edit in $EDITOR
workingon delete|rm <id>                      # Delete TODO
workingon version|-v|--version                # Print version
```

## Getting started

## Build the project

```shell
cargo build --release
./target/release/workingon --help
```

## Run the tests

To run tests execute:

```shell
cargo test
```

## License

Copyright (c) Mark Keller. All rights reserved.

Licensed under the [MIT](LICENSE) license.
