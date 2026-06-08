# even-tinier-url-api

A URL shortener REST API built with Rust, Actix-web, and SQLite. This was just a
fun project taking a system design question and applying it.

## Tech Stack

- **Rust** + **Actix-web 4** — HTTP server
- **SQLx 0.8** — async SQLite driver with compile-time query checking
- **SQLite** — embedded database (auto-created on first run)
- **UUID v4** — short code generation

## Getting Started

### Prerequisites

- [Rust](https://rustup.rs/) (stable)

### Setup

1. Clone the repo and navigate to the project directory.

2. Create a `.env` file:
   ```
   DATABASE_URL=sqlite:tinyurl.db
   ```

3. Run the server:
   ```bash
   cargo run
   ```

The server starts at `http://127.0.0.1:8080`. The SQLite database and schema are created automatically on first run.

## API Endpoints

### `GET /urls`
Returns all stored URL mappings.

**Response** `200 OK`
```json
[
  {
    "id": "uuid",
    "short_code": "a1b2c3",
    "long_url": "https://example.com/some/long/path",
    "created_at": "2026-06-08T12:00:00"
  }
]
```

---

### `POST /new`
Creates a new shortened URL. The short code is a random 6-character prefix of a UUID v4.

**Request body**
```json
{ "long_url": "https://example.com/some/long/path" }
```

**Response** `200 OK`
```json
{
  "long_url": "https://example.com/some/long/path",
  "short_code": "a1b2c3",
  "short_url": "localhost:8080/a1b2c3",
  "status": 200
}
```

---

### `GET /retrieve/{short_code}`
Returns the full URL record for a given short code (JSON, no redirect).

**Response** `200 OK`
```json
{
  "id": "uuid",
  "short_code": "a1b2c3",
  "long_url": "https://example.com/some/long/path",
  "created_at": "2026-06-08T12:00:00"
}
```

---

### `GET /{short_code}`
Redirects to the original URL.

**Response** `302 Found` with `Location` header set to the original URL.

## Project Structure

```
src/
├── main.rs              # Server setup, routing, CORS config
├── db/
│   ├── db.rs            # SQLite pool init and query functions
│   ├── migrations/
│   │   └── 001_create_urls.sql
│   └── mod.rs
├── handler/
│   ├── urls.rs          # Request handlers
│   └── mod.rs
└── models/
    ├── models.rs        # Url, CreateRequest, CreateResponse structs
    └── mod.rs
```

## CORS

The API allows requests from `http://localhost:8000` (the companion frontend). To change the allowed origin, update `main.rs`.
