# Discuz
This codebase is a sample project building the forum server called Discuz. It built with [Actix-web](https://actix.rs) including CRUD operations, authentication, routing, pagination, and more.

## Getting started
```sh
# Install rustup
curl https://sh.rustup.rs -sSf | sh

# Start mysql and seed the database
cargo run --bin db-migration

# Start the Discuz server
cargo run --bin discuz-server
```

### Develop dependencies
This project uses different frameworks to accelerate the development and ensure code quality. 

#### Cargo Make
This project uses [Cargo Make](https://github.com/sagiegurari/cargo-make) as the task runner. A set of tasks, including unit tests, linting, and formatting, need to be done for each commit. The [Makefile.toml](./Makefile.toml) includes configuration and code snippets to run those tasks. 

#### Nextest (Optional)
[Nextest](https://nexte.st/) provides a clean interface for the test results for rust. It is faster than using the `cargo test`. Users can choose the test cases to run with by using this framework. It is 100% compatible with `cargo test` so you can use `cargo test` to perform unit testing instead of this framework.


## Testing
Simply run:
```sh
cargo test
```
You can also check postman / newman. See the `/tests` directory

### Logging
I uses the [tracing](https://docs.rs/tracing/latest/tracing/index.html) module for the logging instead of the [env_logger](https://docs.rs/env_logger/latest/env_logger/) module. 

The tracing module provides an additional module called `tracing-subscriber` to allow us to subscribe to the logging information. We can use this module to manipulate the loggings, like sending the logs to the logging monitoring system like `cloudwatch` or `datadog`.

There are five types of logging levels (list from low priority to high priority):  
Trace -> Debug -> Info -> Warn -> Error

### Reference Projects
 - [Real world example app](https://github.com/TatriX/realworld-rust-rocket)  
Rust implementation of [Real world](https://github.com/gothinkster/realworld)
 - [Rust Async-GraphQL Example: Caster Api](https://github.com/bkonkle/rust-example-caster-api)  
A sample project that use SeaORM as the database ORM and Axum as the server framework
 - [Alkonost](https://github.com/Asapin/alkonost)  
Simple console spam detector for YouTube chats.
 - [SeaOrm with Actix](https://github.com/SeaQL/sea-orm/tree/master/examples/actix_example)  
Integrate the Sea Orm with Actix
 - [Whatsoo](https://github.com/Whatsoo/whatsoo)  
A simple opensource community
 - [Artix-web async middleware example](https://github.com/actix/examples/blob/344bcfce/middleware/middleware/src/read_request_body.rs)  
An example for creating async middleware
 - [The Rust Programming Language](https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/first-edition/README.html)  
The git book from MIT
