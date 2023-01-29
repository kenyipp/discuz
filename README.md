# Discuz

[![Actions Status](https://github.com/kenyipp/discuz/workflows/CI/badge.svg)](https://github.com/kenyipp/discuz/actions/workflows/ci.yml) [![codecov](https://codecov.io/gh/kenyipp/discuz/branch/master/graph/badge.svg?token=AMBNXM57T8)](https://codecov.io/gh/kenyipp/discuz)

This rust-based forum server, named Discuz, is a sample project showcasing the use of [Actix-web](https://actix.rs) for CRUD operations, authentication, routing, pagination and more. 

However, this implementation is not thoroughly tested, so please refer to the [Contributing](#contributing) section for further instructions before use.

## Getting started
 - Install [Rust](https://www.rust-lang.org)
 - Install [MySql](https://www.mysql.com) if you don't have it already. You can use the [docker-compose.yml](./docker-compose.sample.yml) file to start the MySql instance in docker.
 - Clone this repo to a folder on your computer.
 - Setup your database by running `cargo run --bin db-migration`
 - Start the api server by `cargo run --bin discuz-server`

## How it works
The backend web service for this application is developed using [Rust](https://www.rust-lang.org) and the Actix framework.

A comprehensive list of crates utilized in this project can be found in Cargo.toml, however, the following are some of the key ones to note:

 - [Cargo Make](https://github.com/sagiegurari/cargo-make) - the task runner. A set of tasks, including unit tests, linting, and formatting, need to be done for each commit. The [Makefile.toml](./Makefile.toml) includes configuration and code snippets to run those tasks. 
 - [Nextest](https://nexte.st/) - rust next generation test framework.It provides a clean interface for the test results for rust. It is faster than using the `cargo test`. Users can choose the test cases to run with by using this framework. It is 100% compatible with `cargo test` so you can use `cargo test` to perform unit testing instead of this framework.

## Program Design & Features
This project encompasses the fundamental functionality of a forum, where both users and admins can create posts and comment within pre-defined categories. Moreover, a set of restrictions has been implemented to enhance the performance of the application and foster a fair environment for discussion.
 
 - [ ] Limit post and comment management to only administrators, with the option for users to request deletion of their own content
 - [ ] Automatically archive posts that have not received new comments in a certain period of time using a cron job.
 - [ ] Improve error handling on the server API for more efficient and effective troubleshooting.
 - [ ] Once the maximum limit is reached, automatically archive the post and prevent further commenting
 - [ ] Implement caching for archived posts to reduce database traffic
 - [x] Implement the [maximum number](./libs/core/src/constants.rs) of comments per post
 - [x] Allow administrators to manually increase the comment limit for specific posts
 - [x] Ensure that deleting a comment does not decrease the overall comment count
 - [x] Implement a ban system to prevent banned users from commenting

## Database Design

### Types of the table
This repository contains three types of tables: Entity Tables, Definition Tables, and Relation Tables. 

- **Entity Table**  
The basic unit of the program, containing essential information for various domains such as post, user, and post comment tables
- **Definition Table**  
Contains information on system configurations, typically only accessible for modification by the admin. The contents of these tables are typically loaded into the application at startup and are identified by the "def_" prefix
- **Relation Table**  
Establishes connections between entity tables and definition tables, such as the post_tag table linking the post and def_post_tag tables

### Naming convention

#### Indexes
The format of the foreign keys is `FK-{Table name}-{Table column}-{Target table name}-{Target table column}`, and index is `IDX-{Table name}-{Column name}`.

## Testing
To execute the tests, simply run cargo nextest run or cargo test if you do not have Nextest installed. Additionally, you can use tools such as Postman or Newman by referring to the /tests directory.

## Logging
I uses the [tracing](https://docs.rs/tracing/latest/tracing/index.html) module for the logging instead of the [env_logger](https://docs.rs/env_logger/latest/env_logger/) module. 

The tracing module provides an additional module called `tracing-subscriber` to allow us to subscribe to the logging information. We can use this module to manipulate the loggings, like sending the logs to the logging monitoring system like [cloudwatch](https://aws.amazon.com/tw/cloudwatch) or [datadog](https://www.datadoghq.com).

There are five types of logging levels (list from low priority to high priority):  
`Trace` -> `Debug` -> `Info` -> `Warn` -> `Error`

## Contributing
Please review the existing issues in this repository for areas that require improvement.  
If you identify any missing or potential areas for improvement, feel free to open a new issue for them.

### Before commit
Before deploying and integrating the application, it is necessary to perform a series of validations such as testing, linting, and formatting. We recommend running cargo make pre-commit before making each commit to ensure compliance.

## Reference Projects
 - [Actix Examples](https://github.com/actix/examples)  
A list of actix web examples
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
 - [The Rust Programming Language](https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/first-edition/README.html)  
The git book from MIT
