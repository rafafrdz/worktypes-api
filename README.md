# WorkTypes API

A RESTful API built in Rust for managing work types and companies. This project implements basic CRUD operations along with a duplication feature.

## Technologies

- **Rust**: Systems programming language used to build the API.
- **Tokio**: Asynchronous runtime.
- **Axum**: HTTP web framework.
- **Serde**: JSON serialization/deserialization.
- **UUID**: For generating unique identifiers.
- **Chrono**: For handling date and time.
- **SQLx**: Async PostgreSQL driver and query builder.

## API Endpoints and Examples

Please, check [docs](./docs/) folder. There you could find descriptions about the endpoints and some examples to show you how to deal with them.

- [Api Endpoint Descriptions](./docs/api-endpoints.md).
- [Examples](./docs/examples.md).

## Installation and Local Development

### Prerequisites

- Rust and Cargo: [`https://rustup.rs/`](https://rustup.rs/)
- PostgreSQL (optional — in-memory fallback is available)

### Setup

1. **Clone the repository:**

   ```bash
   git clone https://github.com/yourusername/worktypes-api.git
   cd worktypes-api
   ```

2. **Run PostgreSQL with Docker (optional but recommended):**

   ```bash
   docker compose -f docker/docker-compose.yml up -d
   ```

3. **Configure your environment variables:**

   - Option A: Export manually

     ```bash
     export DATABASE_URL=postgres://postgres:postgres@localhost:5432/worktypes
     export PORT=3000
     ```

   - Option B: Use a `.env` file in the project root:

     ```text
     DATABASE_URL=postgres://postgres:postgres@localhost:5432/worktypes
     PORT=3000
     ```

4. **Create the database and run migrations (if using `sqlx-cli`):**

   ```bash
   sqlx migrate run
   ```

5. **Build the project:**

   ```bash
   cargo build --release
   ```

6. **Run the server:**

   ```bash
   cargo run --release
   ```

The API will be available at: `http://localhost:3000`

## Deployment

If you're deploying the API, make sure a PostgreSQL instance is available. You can use the provided Docker setup.

### Docker Deployment

1. **Create a Dockerfile:**

   ```dockerfile
   FROM rust:1.70 as builder
   WORKDIR /usr/src/app
   COPY . .
   RUN cargo build --release

   FROM debian:bullseye-slim
   RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
   COPY --from=builder /usr/src/app/target/release/worktypes-api /usr/local/bin/
   EXPOSE 3000
   CMD ["worktypes-api"]
   ```

2. **Build the Docker image:**

   ```bash
   docker build -t worktypes-api .
   ```

3. **Run the container:**

   ```bash
   docker run -p 3000:3000 -e DATABASE_URL=postgres://postgres:postgres@host.docker.internal:5432/worktypes worktypes-api
   ```

## Adding New Modules

To add a new module:

1. Create a new directory under `src/modules/`.
2. Follow the structure used in the `companies` module.
3. Register the module in [`Cargo.toml`](Cargo.toml) under `[workspace]`.
4. Add its routes in `AppModules` in [`apps/api/src/lib.rs`](./apps/api/src/lib.rs).

