
# Overview

The API is built using the [Axum](https://github.com/tokio-rs/axum) web framework and [SQLx](https://github.com/launchbadge/sqlx) for database connectivity with PostgreSQL.

The API provides endpoints to create various resources like users, bikes etc. The API is versioned using a version parameter in the URL path. The version number will be part of the URL path as `/:version/api`.

The API is integrated with a **postgres** instance, and an **Amazon S3 bucket**, which enables data to be persisted. S3 stores assets; the assets' respective keys are stored in an asset table in the postgres database.

## Prerequisites

Before running the API, ensure you have the following set up:

1. [Rust](https://www.rust-lang.org/) installed on your system.
2. A PostgreSQL database with an appropriate schema. Set the `DATABASE_URL` environment variable in the `.env` to the connection string of your PostgreSQL database. 
3. Install [sqlx-cli](https://github.com/launchbadge/sqlx/blob/main/sqlx-cli/README.md)
4. Use `sqlx migrate run` to run the migration on the database.
5. Use `cargo run` to run the api. It should be hosted locally at `http://localhost:1690/v1/api/`

At current, there is a database setup on supabase, so steps [2], [3], [4] should be un-necessary, if you're only looking to play around with it.

## API Endpoints

The API provides the following endpoints:

- **POST /:version/api/users**: Create a new user.


## Testing

To test the API, use a tool like `curl` or API testing clients like [Postman](https://www.postman.com/) or [Insomnia](https://insomnia.rest/).


