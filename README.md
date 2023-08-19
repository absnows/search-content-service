[![Rust](https://github.com/absnows/search-content-service/actions/workflows/rust-test.yml/badge.svg)](https://github.com/absnows/search-content-service/actions/workflows/rust-test.yml)

# search-content-service
Project Name is a Rust-based web service for searching and retrieving content information for a specific date. It utilizes Redis as the database backend to store and retrieve information efficiently.

## Table of Contents

- [Features](#features)
- [Requirements](#requirements)
- [Usage](#usage)
- [Configuration](#configuration)
- [Docker Compose](#docker-compose)
- [Contributing](#contributing)
- [License](#license)

## Features

- RESTful API for searching content information by date.
- Data stored and retrieved from a Redis database.
- Easy-to-use configuration using environment variables.

## Requirements

- Rust (1.53 or higher)
- Cargo (Rust package manager)
- Redis (for database storage)
- Docker and Docker Compose (for local development)

1. Clone the repository:

```sh
git clone https://github.com/absnows/search-content-service.git
cd search-content-service
```

### Installation instructions: https://www.rust-lang.org/tools/install

2. Install project dependencies:

```sh
cargo build
```

## Usage

Start the server:
```sh
cargo run
```

## Access the API in your browser or use tools like curl or Postman:
```sh
GET http://localhost:8080/api/dates/2023-01-01
```

## Response:

Access the API in your browser or use tools like curl or Postman:
```json
{
  "content": "Some content information for the specified date."
}

```


## Configuration

Configuration is managed through environment variables. Rename the .env.example file to .env and update the values as needed.

## Docker Compose

For local development, you can use Docker Compose to set up a Redis database:

1 - Install Docker and Docker Compose.
2 - Run the following command:

```sh
docker-compose up -d
```

Contributing

Contributions are welcome! Please read the Contributing Guidelines for more details.