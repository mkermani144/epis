# Epis

Epis is an extensible assistant designed to help you learn anything.

## Features

- **Extensible LLM Provider Support**: Easily swap or extend large language model backends (currently supports [Ollama](https://github.com/ollama/ollama)).

## Usage

1. **Clone the repository:**
   ```sh
   git clone https://github.com/mkermani144/epis
   cd epis
   ```

2. **Build and run:**
   ```sh
   cargo run
   ```

3. **Follow the prompts** to start a conversation.

## Development

### Option 1: DevContainer (Recommended)

The easiest way to get started is using the provided DevContainer setup:

1. **Install VS Code and Dev Containers extension**
2. **Clone and open the repository:**
   ```sh
   git clone https://github.com/mkermani144/epis
   code epis
   ```
3. **Open in Container**: When prompted, click "Reopen in Container"
4. **Create `.env` file** with the following content:
   ```bash
   PROVIDER=ollama
   GENERATION_MODEL=some-gen-model
   EMBEDDING_MODEL=some-embedding-model
   DATABASE_URL=postgresql://epis_user:epis_password@postgres:5432/epis_db
   RUST_LOG=info
   ```

The DevContainer includes:
- Rust development environment
- PostgreSQL with pgvector extension
- All necessary dependencies pre-installed

### Option 2: Local Development

#### Prerequisites

- Rust (edition 2024)
- PostgreSQL with pgvector extension
- [Ollama](https://github.com/ollama/ollama) running locally (for LLM features)

#### Setup

1. **Install PostgreSQL with pgvector:**
   ```sh
   # Using Docker
   docker run -d --name epis-postgres \
     -e POSTGRES_DB=epis_db \
     -e POSTGRES_USER=epis_user \
     -e POSTGRES_PASSWORD=epis_password \
     -p 5432:5432 \
     pgvector/pgvector:pg16
   ```

2. **Initialize the database:**
   ```sh
   docker exec -i epis-postgres psql -U epis_user -d epis_db < init-db.sql
   ```

3. **Set environment variables:**
   ```sh
   export DATABASE_URL="postgresql://epis_user:epis_password@localhost:5432/epis_db"
   export PROVIDER="ollama"
   export GENERATION_MODEL="some-gen-model"
   export EMBEDDING_MODEL="some-embedding-model"
   export RUST_LOG="info"
   ```

### Development Commands

```sh
# Format code
cargo fmt

# Run linter
cargo clippy

# Run tests
cargo test

# Build
cargo build

# Run the application
cargo run
```
