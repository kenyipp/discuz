### Local Development

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
