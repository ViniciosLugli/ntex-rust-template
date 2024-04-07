# Ntex Rust Template

This template provides a starting point for building web services with [Ntex](https://github.com/ntex-rs/ntex), an asynchronous web framework for Rust aimed at making high-performance, scalable web services with minimal effort.

## Features

-   **Ntex Web Framework:** Leverages Ntex for building efficient, scalable web applications with support for asynchronous I/O.
-   **Serde for Serialization/Deserialization:** Utilizes Serde to effortlessly convert Rust structs into JSON and vice versa.
-   **CORS Support:** Configured with `ntex-cors` for handling Cross-Origin Resource Sharing (CORS), making your API accessible from different domains.
-   **Environment Logging and Configuration:** Uses `pretty_env_logger` for advanced logging capabilities and `dotenvy` for loading environment configurations, simplifying the management of environment variables.

## Prerequisites

To use this template, you need Rust installed on your system. If Rust isn't installed yet, follow the installation guide on the [Rust official website](https://www.rust-lang.org/tools/install).

## Setup

Clone this repository and navigate to the project directory:

```bash
git clone git@github.com:ViniciosLugli/ntex-rust-template.git
cd ntex-rust-template
```

## Configuration

Create a `.env` file in the project's root directory. This file can be used to store environment variables:

```env
# Example of .env content
RUST_LOG=info
```

## Running the Application

Execute the following command to run the application:

```bash
cargo run
```

The server will be available at `127.0.0.1:3000`. Access the service using a web browser or tools like `curl`:

```bash
curl http://localhost:3000
```

## Testing

The project comes with basic tests. Execute the tests using:

```bash
cargo test
```
