# workingon

[![Build and test](https://github.com/keller00/workingon/actions/workflows/coverage.yml/badge.svg)](https://github.com/keller00/workingon/actions/workflows/coverage.yml)
[![codecov](https://codecov.io/github/keller00/workingon/graph/badge.svg?token=OFRWWDT7BT)](https://codecov.io/github/keller00/workingon)

A CLI to track what someone is working on and manage TODOs.

For now this every API is experimental and could change at any time.

## Build the project

```shell
cargo build
```

## Integration tests

To run tests execute:

```shell
cargo test
```

## CLI Commands

### Adding a TODO

```shell
$ workingon add "Review pull request"
TODO added successfully
```

You can also add a TODO without a title and it will open your default editor:

```shell
$ workingon add
# Edit the TODO in your editor
TODO added successfully
```

### Listing TODOs

List open (uncompleted) TODOs (default behavior):

```shell
$ workingon list
tfuen Review pull request
z4wf7 Feed the shrimps
kvim4 Feed Sophia
jioq4 Buy groceries
```

You can also use the `ls` alias:

```shell
$ workingon ls
tfuen Review pull request
z4wf7 Feed the shrimps
```

List only completed TODOs:

```shell
$ workingon list --completed
x9k2m Fix bug in login
a7b3c Update documentation
```

List all TODOs (both open and completed):

```shell
$ workingon list --all
tfuen Review pull request
x9k2m Fix bug in login
z4wf7 Feed the shrimps
a7b3c Update documentation
```

### Showing a TODO

```shell
$ workingon show tfuen
Review pull request
Need to check the implementation details and test coverage

It was completed on: not yet
```

For a completed TODO:

```shell
$ workingon show x9k2m
Fix bug in login
Fixed the authentication issue

It was completed on: 2024-01-15 10:30:00 UTC
```

### Completing a TODO

```shell
$ workingon complete tfuen
TODO was completed
```

### Reopening a TODO

```shell
$ workingon reopen tfuen
TODO was reopened
```

### Editing a TODO

```shell
$ workingon edit tfuen
# Opens your default editor to edit the TODO
TODO updated successfully
```

### Deleting a TODO

```shell
$ workingon delete tfuen
Post with id tfuen was deleted
```

You can also use the `rm` alias:

```shell
$ workingon rm tfuen
Post with id tfuen was deleted
```

### Finding the database file

(Hidden command)

```shell
$ workingon locate-db
/Users/username/Library/Application Support/workingon/todos.sqlite3
```

### Version

```shell
$ workingon version
workingon version 0.0.1
```
