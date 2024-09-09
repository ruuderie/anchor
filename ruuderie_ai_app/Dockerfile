# Frontend Build Stage
FROM rust:1.70-bullseye as frontend-builder
RUN apt-get update && apt-get install -y \
    curl \
    build-essential \
    pkg-config \
    libssl-dev \
    nodejs \
    npm \
    git \
    tree
RUN npm install -g sass
RUN rustup default stable && \
    rustup target add wasm32-unknown-unknown
RUN cargo install --git https://github.com/trunk-rs/trunk trunk
WORKDIR /app
COPY ruuderie_ai_app .
# Debug: Print directory structure
RUN echo "Frontend Directory structure:" && tree -L 3
# Debug: Find index.html files
RUN find . -name "index.html"
# Debug: Print Cargo.toml contents
RUN echo "Frontend Cargo.toml:" && cat app/Cargo.toml
# Debug: Show dependency tree for frontend
RUN cd app && echo "Frontend dependency tree:" && cargo tree
# Build frontend with Trunk
RUN cd app && trunk build --release --verbose

# Backend Build Stage
FROM rust:1.70-bullseye as backend-builder
RUN apt-get update && apt-get install -y \
    curl \
    build-essential \
    pkg-config \
    libssl-dev \
    tree
RUN rustup default stable
WORKDIR /app
COPY ruuderie_ai_app .
# Debug: Print directory structure
RUN echo "Backend Directory structure:" && tree -L 3
# Debug: Print Cargo.toml contents
RUN echo "Backend Cargo.toml:" && cat backend/Cargo.toml
# Debug: Show dependency tree for backend
RUN cd backend && echo "Backend dependency tree:" && cargo tree
RUN cargo build --release --package backend --verbose

# Runtime Stage
FROM debian:bullseye-slim as runtime
RUN apt-get update && apt-get install -y \
    ca-certificates \
    tzdata \
    && rm -rf /var/lib/apt/lists/*
COPY --from=backend-builder /app/target/release/backend /usr/local/bin/
COPY --from=frontend-builder /app/app/dist /app/dist
WORKDIR /app
ENV LEPTOS_OUTPUT_NAME="ruuderie-ai"
ENV LEPTOS_SITE_ROOT="/app/dist"
ENV LEPTOS_SITE_PKG_DIR="pkg"
ENV LEPTOS_SITE_ADDR="0.0.0.0:3000"
ENV LEPTOS_RELOAD_PORT="3001"
EXPOSE 3000
EXPOSE 3001
CMD ["/usr/local/bin/backend"]
