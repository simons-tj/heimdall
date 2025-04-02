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

3. Run the Neo4j database

```bash
docker run \
 --privileged \
 -p 7474:7474 -p 7687:7687 \
 --name neo4j-apoc \
 -e NEO4J_apoc_export_file_enabled=true \
 -e NEO4J_apoc_import_file_enabled=true \
 -e NEO4J_apoc_import_file_use__neo4j__config=true \
 -e NEO4J_PLUGINS='["apoc"]'\
 -v $HOME/neo4j/data:/data \
 -v $HOME/neo4j/logs:/logs \
 -v $HOME/neo4j/conf:/var/lib/neo4j/conf \
 -v $HOME/neo4j/import:/var/lib/neo4j/import \
 -v $HOME/neo4j/plugins:/var/lib/neo4j/plugins \
 neo4j:4.2
```

4. Run the development server:
```bash
cargo run
# or with cargo watch
cargo watch -x run
```

The server will start on `http://localhost:8080`

## API Endpoints

### Policies

- `GET /relationships` - Given an entitiy, list the related entities with attached properties
- `POST /relationships` - Relate 2 things together with the provided relation and payload
- `DELETE /relationships` - Delete a relationship
