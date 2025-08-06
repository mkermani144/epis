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

### Prerequisites

- Rust (edition 2024)
- [Ollama](https://github.com/ollama/ollama) running locally (for LLM features)

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
```
