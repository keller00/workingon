# workingon

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


## Other CLI commands

### Find the database file

(a hidden command)

```shell
cargo run -- locate-db
```

### Adding a TODO

(this command will not stick around, it's just for testing purposes)

```shell
$ cargo run -- add-todo "this is a test"

```

### Listing all TODOs

(this command will not stick around, it's just for testing purposes)

```shell
$ cargo run -- list-todos
this is a test
-----------



```
