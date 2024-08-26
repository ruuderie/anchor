# Frontend Build Stage
FROM rust:1.80.1 as frontend-builder

RUN apt-get update && apt-get install -y \
    curl \
    build-essential \
    pkg-config \
    libssl-dev \
    nodejs \
    npm

RUN npm install -g sass

RUN rustup default nightly && \
    rustup target add wasm32-unknown-unknown

RUN cargo install cargo-leptos

WORKDIR /app
COPY ruuderie_ai_app .

RUN cd app && cargo leptos build --release

# Backend Build Stage
FROM rust:1.80.1 as backend-builder

RUN apt-get update && apt-get install -y \
    curl \
    build-essential \
    pkg-config \
    libssl-dev \
    gcc-aarch64-linux-gnu \
    g++-aarch64-linux-gnu

RUN rustup default stable && \
    rustup target add aarch64-unknown-linux-gnu

RUN mkdir -p ~/.cargo && \
    echo '[target.aarch64-unknown-linux-gnu]' >> ~/.cargo/config && \
    echo 'linker = "aarch64-linux-gnu-gcc"' >> ~/.cargo/config

WORKDIR /app
COPY ruuderie_ai_app .

RUN cargo build --release --target aarch64-unknown-linux-gnu --package backend

# Runtime Stage
FROM debian:bullseye-slim as runtime

RUN apt-get update && apt-get install -y \
    ca-certificates \
    tzdata \
    && rm -rf /var/lib/apt/lists/*

COPY --from=backend-builder /app/target/aarch64-unknown-linux-gnu/release/backend /usr/local/bin/
COPY --from=frontend-builder /app/app/target/site /app/site

WORKDIR /app

ENV LEPTOS_OUTPUT_NAME="ruuderie-ai"
ENV LEPTOS_SITE_ROOT="/app/site"
ENV LEPTOS_SITE_PKG_DIR="pkg"
ENV LEPTOS_SITE_ADDR="0.0.0.0:3000"
ENV LEPTOS_RELOAD_PORT="3001"

EXPOSE 3000
EXPOSE 3001

CMD ["/usr/local/bin/backend"]