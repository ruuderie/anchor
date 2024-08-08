############################################################################
# Chef Stage
FROM clux/muslrust:stable AS chef

ARG PROJECT_DIR
USER root
RUN cargo install --locked cargo-chef
WORKDIR /${PROJECT_DIR}

# Install the x86_64-unknown-linux-musl target
RUN rustup target add x86_64-unknown-linux-musl

############################################################################
# Planning Stage
FROM chef AS planner

ARG PROJECT_DIR
WORKDIR /${PROJECT_DIR}
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

############################################################################
# Cacher Stage
FROM chef AS cacher

ARG PROJECT_DIR
WORKDIR /${PROJECT_DIR}
COPY --from=planner /${PROJECT_DIR}/recipe.json recipe.json
RUN cargo chef cook --release --target x86_64-unknown-linux-musl --recipe-path recipe.json

############################################################################
# Builder Stage
FROM chef AS builder

ARG PROJECT_DIR
ARG BIN_TARGET

WORKDIR /${PROJECT_DIR}
COPY --from=cacher /${PROJECT_DIR}/target/ ./target/
COPY . .
RUN cargo build --release --target x86_64-unknown-linux-musl --bin ${BIN_TARGET}

############################################################################
# Runtime Stage
FROM debian:bullseye-slim AS runtime

ARG PROJECT_DIR
ARG BIN_TARGET
ARG USERNAME
ARG USER_ID
ARG USER_GID

# Copy the project app built in the builder stage
COPY --from=builder /${PROJECT_DIR}/target/x86_64-unknown-linux-musl/release/${BIN_TARGET} /usr/local/bin/

# Add a non-root user and allow them to become root or postgres
RUN groupadd -g ${USER_GID} ${USERNAME} \
    && useradd -s /bin/bash -u ${USER_ID} -g ${USER_GID} -m ${USERNAME} || echo "User already exists." \
    && apt-get update \
    && apt-get install -y sudo \
    && echo ${USERNAME} ALL=\(root, postgres\) NOPASSWD:ALL > /etc/sudoers.d/${USERNAME} \
    && chmod 0440 /etc/sudoers.d/${USERNAME}

# Install necessary packages
RUN apt-get update \
    && DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends \
    build-essential \
    ca-certificates \
    curl \
    git \
    libssl-dev \
    pkg-config \
    postgresql \
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*

# Set up Rust environment
ENV RUSTUP_HOME="/usr/local/rustup" \
    CARGO_HOME="/usr/local/cargo" \
    PATH="/usr/local/cargo/bin:$PATH"

RUN curl https://sh.rustup.rs -sSf | \
    sh -s -- --default-toolchain stable -y \
    && chmod -R a+w ${RUSTUP_HOME} ${CARGO_HOME}

RUN rustup update \
    && rustup target add wasm32-unknown-unknown \
    && cargo install cargo-leptos sea-orm-cli leptosfmt

RUN chown -R ${USERNAME}:${USERNAME} /usr/local/cargo/registry

RUN curl -sLO https://github.com/tailwindlabs/tailwindcss/releases/latest/download/tailwindcss-linux-x64 \
    && chmod a+x tailwindcss-linux-x64 \
    && mv tailwindcss-linux-x64 /usr/local/bin/tailwindcss

USER ${USERNAME}

CMD ["/usr/local/bin/backend"]
