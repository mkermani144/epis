FROM rust:slim-bullseye

# Install system dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    libpq-dev \
    curl \
    git \
    sudo \
    zsh \
    && rm -rf /var/lib/apt/lists/*

# Create a non-root user
RUN useradd --create-home --shell /bin/bash vscode \
    && usermod -aG sudo vscode \
    && echo "vscode ALL=(ALL) NOPASSWD:ALL" >> /etc/sudoers

# Set zsh as default shell for the vscode user
RUN chsh -s /usr/bin/zsh vscode

# Set working directory
WORKDIR /workspace

# Change ownership of the workspace
RUN chown -R vscode:vscode /workspace

#### Switch to non-root user
USER vscode

# Install Oh My Zsh non-interactively and provide a default config
ENV RUNZSH=no
RUN sh -c "$(curl -fsSL https://raw.githubusercontent.com/ohmyzsh/ohmyzsh/master/tools/install.sh)" "" --unattended \
    && cp ~/.oh-my-zsh/templates/zshrc.zsh-template ~/.zshrc

# Install additional Rust tools
RUN rustup component add rustfmt clippy rust-analyzer

# Install sqlx-cli for database migrations and management
RUN cargo install sqlx-cli --no-default-features --features postgres

# Set environment variables
ENV RUST_BACKTRACE=1
ENV CARGO_INCREMENTAL=1 
