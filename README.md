# Discuz

[![Actions Status](https://github.com/kenyipp/discuz/workflows/CI/badge.svg)](https://github.com/kenyipp/discuz/actions/workflows/ci.yml) [![codecov](https://codecov.io/gh/kenyipp/discuz/branch/master/graph/badge.svg?token=AMBNXM57T8)](https://codecov.io/gh/kenyipp/discuz)

This codebase is a sample project building the forum server called Discuz. It built with [Actix-web](https://actix.rs) including CRUD operations, authentication, routing, pagination, and more.

This implementation is not reviewed. See the [Contributing](#contributing) section below.

## Getting started
 - Install [Rust](https://www.rust-lang.org)
 - Install [MySql](https://www.mysql.com) if you don't have it already. You can use the [docker-compose.yml](./docker-compose.sample.yml) file to start the MySql instance in docker.
 - Clone this repo to a folder on your computer.
 - Setup your database by running `cargo run --bin db-migration`
 - Start the api server by `cargo run --bin discuz-server`

## How it works
This [Rust](https://www.rust-lang.org) application utilizes Actix to develop the backend web service.

You can view a full list of crates being used in Cargo.toml, but here are some of the main ones of note:

 - [Cargo Make](https://github.com/sagiegurari/cargo-make) - the task runner. A set of tasks, including unit tests, linting, and formatting, need to be done for each commit. The [Makefile.toml](./Makefile.toml) includes configuration and code snippets to run those tasks. 
 - [Nextest](https://nexte.st/) - rust next generation test framework.It provides a clean interface for the test results for rust. It is faster than using the `cargo test`. Users can choose the test cases to run with by using this framework. It is 100% compatible with `cargo test` so you can use `cargo test` to perform unit testing instead of this framework.

## Program Design & Features
This project includes the basic functionalities of the forum. Both users and admins can create a post and comment on it under the pre-defined category. In addition, we apply a list of restrictions to improve the performance of this application and create a fair environment of discussion. 

 - [x] There is a hard [limit](./libs/core/src/constants.rs) on the number of comments on each post. 
 - [ ] After the post reaches the maximum limit of comments, the program will archive the post. Users can't leave any comments after the post is archived.
 - [ ] We will cache archived posts so the traffic of archived posts will not go to the database.
 - [x] Admins can increase the limit of the number of comments for specific topics manually.
 - [ ] Only the admin can update or delete the post and comment. If the user wishes to delete the content they created, they can create the request for admins.
 - [x] Deleting the comment of the post will not decrease the comment count. 
 - [ ] Users can't comment on posts if they have been banned.
 - [ ] A cron job will archive the post if there are no more new comments.

## Database Design

### Types of the table
This repository has three types of tables: Entity Tables, Definition Tables, and Relation Tables. 

- **Entity Table**  
It is the basic unit of the program. It includes the basic information of different domains, including the post, user, and post comment table.
- **Definition Table**  
The definition tables include information on system configs. In most cases, only the admin can modify the content in definition tables. Most of the content in this type of table will be loaded into the application when it starts. All the definition tables start with the `def_` prefix.
- **Relation Table**  
The relation tables build the connection between entity table to entity table and the definition table. For example, the `post_tag` table describes the relationship between the post table and the def_post_tag table.

### Naming convention

#### Indexes
The format of the foreign keys is `FK-{Table name}-{Table column}-{Target table name}-{Target table column}`, and index is `IDX-{Table name}-{Column name}`.

## Testing
Simply run `cargo nextest run` or `cargo test` if you don't want to install Nextest.  
You can also check postman / newman. See the `/tests` directory.  

## Logging
I uses the [tracing](https://docs.rs/tracing/latest/tracing/index.html) module for the logging instead of the [env_logger](https://docs.rs/env_logger/latest/env_logger/) module. 

The tracing module provides an additional module called `tracing-subscriber` to allow us to subscribe to the logging information. We can use this module to manipulate the loggings, like sending the logs to the logging monitoring system like [cloudwatch](https://aws.amazon.com/tw/cloudwatch) or [datadog](https://www.datadoghq.com).

There are five types of logging levels (list from low priority to high priority):  
`Trace` -> `Debug` -> `Info` -> `Warn` -> `Error`

## Contributing
Feel free to look at the current issues in this repo for anything that needs to improve.  
You are also welcome to open a new issue if you see something missing or could be improved.

### Before commit
Before deploying and integrating the application, a set of validations (testing, linting, and formatting) are required. We advise to run `cargo make pre-commit` before each commit. 

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
