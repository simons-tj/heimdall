# Heimdall Access Control Service

Heimdall is a modern access control service built with Rust, using Cedar policies for authorization and a graph database for relationship management.

## Features

- Cedar policy-based authorization
- Graph database for relationship management
- RESTful API built with Axum
- JWT-based authentication
- Comprehensive logging and monitoring

## Prerequisites

- Rust 1.70 or later (for local development)
- Docker and Docker Compose (for containerized deployment)
- Neo4j database (for graph storage)

## Getting Started

### Using Docker (Recommended)

1. Clone the repository:
```bash
git clone https://github.com/yourusername/heimdall.git
cd heimdall
```

2. Start the services using Docker Compose:
```bash
docker compose up --build
```

This will:
- Build and start the Heimdall service
- Start a Neo4j database
- Set up the necessary networking between services
- Expose the following ports:
  - Heimdall API: http://localhost:3000
  - Neo4j Browser: http://localhost:7474
  - Neo4j Bolt: bolt://localhost:7687

### Local Development

1. Clone the repository:
```bash
git clone https://github.com/yourusername/heimdall.git
cd heimdall
```

2. Set up environment variables:
```bash
cp .env.example .env
# Edit .env with your configuration
```

3. Run the development server:
```bash
cargo run
```

The server will start on `http://localhost:3000`

## API Endpoints

### Policies

- `GET /policies` - List all policies
- `POST /policies` - Create a new policy
- `DELETE /policies/:id` - Delete a policy

### Example Policy

```cedar
permit(
    principal == User::"alice",
    action == Action::"view",
    resource == Resource::"document"
);
```

## Development

### Running Tests

```bash
cargo test
```

### Code Style

This project uses `rustfmt` for code formatting. Run:

```bash
cargo fmt
```

### Docker Commands

- Start services: `docker compose up`
- Start services in detached mode: `docker compose up -d`
- Stop services: `docker compose down`
- View logs: `docker compose logs -f`
- Rebuild services: `docker compose up --build`
- Access Neo4j browser: http://localhost:7474
  - Username: neo4j
  - Password: development_password

## License

This project is licensed under the MIT License - see the LICENSE file for details. 