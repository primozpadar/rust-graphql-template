# Rust GraphQL server template - 🚧 WIP 🚧

## Stuff used 👀

- **Actix** - Web server
- **Juniper** - GraphQL library
- **Diesel** - Database ORM with PostgreSQL

## Setup ⚙️

1. Install [diesel cli](https://diesel.rs/)

```
cargo install diesel_cli --no-default-features --features postgres
```

2. Setup database

```
docker-compose up -d

diesel setup
diesel migration run
```

3. Start server

```
cargo run
```
