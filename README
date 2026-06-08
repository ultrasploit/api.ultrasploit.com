# Ultrasploit API

A fast, lightweight, and extensible REST API built with Rust and Actix-web.

## Features

* Fast and efficient Rust backend
* Modular API architecture
* Structured JSON logging
* Environment-based configuration
* Docker deployment support
* Designed for future expansion

## API

> **Status: Beta** — The API is under active development and new endpoints may be added over time

##### `GET /health`

Returns `200 OK` with body `OK`.

### Passwords

##### `POST /api/v1/passwords/is-common`

Checks whether a password appears in the common password database.

**Request**

```json
{
  "password": "password123"
}
```

**Response**

```json
{
  "common": true
}
```

or

```json
{
  "common": false
}
```

## Running locally

### Prerequisites

* Rust toolchain

### Configure environment

```bash
cp .env.example .env
```

### Create the password database

The password database is not included in the repository. You'll need a password wordlist to generate it. The one I used is [Pwdb_top-10000000.txt](https://github.com/danielmiessler/SecLists/blob/master/Passwords/Common-Credentials/Pwdb_top-10000000.txt)

1. Open `src/bin/build_password_db.rs`.
2. Find the hardcoded path:

```rust
"/home/ultra/Documents/databases/Pwdb_top-10000000.txt"
```

3. Replace it with the path to your own password wordlist.

Once that's done, generate the binary database:

```bash
cargo run --bin build_password_db
```

This will create a `database/passwords.bin` file that the API uses for password lookups.


### Run the server

```bash
cargo run
```

## Project Structure

```text
src/
├── api/            # API routes
├── bin/            # Utility binaries
├── config/         # Configuration
├── state/          # Shared application state
├── telemetry/      # Logging and observability
└── utils/          # Internal utilities
```

## Notes

* Logging is configurable through `RUST_LOG`.
* CORS is restricted by default.
* The API is designed to support multiple services under a versioned API structure.
