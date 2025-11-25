# workingon

[![Build and test](https://github.com/keller00/workingon/actions/workflows/coverage.yml/badge.svg)](https://github.com/keller00/workingon/actions/workflows/coverage.yml)
![GitHub Release](https://img.shields.io/github/v/release/keller00/workingon)
[![codecov](https://codecov.io/github/keller00/workingon/graph/badge.svg?token=OFRWWDT7BT)](https://codecov.io/github/keller00/workingon)

Workingon is a way to manage TODOs right in your terminal with your own EDITOR.

## What makes workingon different?

TODOs are stored locally in SQLite and database migrations run automatically on startup.

## Typical usage

```shell
$ workingon add "Finish writing a better README for workingon"
bl5kg created
$ workingon list
 id     created      title
 bl5kg  2 days ago   Finish writing a better README for workingon
 5d6ay  5 days ago   Order batteries
$ workingon complete bl5kg
bl5kg completed, if this was a mistake reopen with `workingon reopen bl5kg`
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

Install workingon with Homebrew:

```shell
brew install keller00/workingon-tap/workingon
```

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
