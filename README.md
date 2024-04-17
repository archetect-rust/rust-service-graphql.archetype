# Archetype: Rust Modular GraphQL Microservice

A Rust GraphQL-based Microservice archetype, with the following qualities:

- Strong adherence to [Twelve Factor](https://12factor.net/) principles
  - Layered, hierarchical configuration using [config](https://github.com/mehcode/config-rs) 
  - Run and managed by an ergonomic CLI interface, powered by [clap](https://github.com/clap-rs/clap)
- Completely asynchronous, powered by [Tokio](https://tokio.rs/)
- Modular, with individually usable and tested layers:
  - Server: [GraphQL](https://graphql.org/) based remoting layer using [async-graphql](https://github.com/async-graphql/async-graphql).
  - Core: business layer, adapting the GraphQL API implementation over the persistence and/or remote client layers
  - Persistence: persistence tier abstraction and database migrations, provided by [SeaORM](https://github.com/SeaQL/sea-orm)
- [testcontainers-async](https://github.com/jimmiebfulton/testcontainers-async-rust) are leveraged for testing and rapid prototyping and development
- Additional build and development tooling through:
  - [xtask](https://github.com/matklad/cargo-xtask/) pattern
  - [just](https://github.com/casey/just) command runner


To generate a project from this archetype using [Archetect](https://github.com/archetect/archetect):

```shell
archetect render https://github.com/archetect-rust/rust-service-graphql.archetype.git
```
