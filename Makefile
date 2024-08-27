# Variables
DOCKER_COMPOSE = docker-compose
CARGO = cargo
LEPTOS = cargo leptos
TAILWIND = tailwindcss
APP_DIR = ruuderie_ai_app
TAILWIND_INPUT := $(shell find $(CURDIR)/$(APP_DIR)/app/public/style -name "*.css" | head -n 1)
TAILWIND_OUTPUT = ./$(APP_DIR)/app/public/style/output.css

# Default target
.PHONY: all
all: build run

# Build the Docker containers
.PHONY: build
build:
	$(DOCKER_COMPOSE) build

# Run the Docker containers
.PHONY: run
run:
	$(DOCKER_COMPOSE) up

# Stop and remove the Docker containers
.PHONY: down
down:
	$(DOCKER_COMPOSE) down

# Clean up Docker resources
.PHONY: clean
clean: down
	$(DOCKER_COMPOSE) rm -f

# Build the Docker image (includes both backend and frontend)
.PHONY: docker-build
docker-build:
	@echo "Building Docker image..."
	$(DOCKER_COMPOSE) build app

# Run the Docker container
.PHONY: docker-run
docker-run:
	@echo "Running Docker container..."
	$(DOCKER_COMPOSE) up app

# Build the backend project locally (not in Docker)
.PHONY: backend-build-local
backend-build-local:
	@echo "Building backend locally..."
	cd $(APP_DIR)/backend && $(CARGO) build --release

# Build the frontend project locally (not in Docker)
.PHONY: frontend-build-local
frontend-build-local:
	@echo "Building frontend locally..."
	cd $(APP_DIR)/app && $(LEPTOS) build --release

# Run the backend project locally (not in Docker)
.PHONY: backend-run-local
backend-run-local:
	@echo "Running backend locally..."
	cd $(APP_DIR)/backend && $(CARGO) run --release

# Run the frontend project locally (not in Docker)
.PHONY: frontend-run-local
frontend-run-local:
	@echo "Running frontend locally..."
	cd $(APP_DIR)/app && $(LEPTOS) serve

# Run both frontend and backend locally
.PHONY: run-both-local
run-both-local:
	$(MAKE) -j2 backend-run-local frontend-run-local

# Run tests for both projects
.PHONY: test
test:
	cd $(APP_DIR)/backend && $(CARGO) test
	cd $(APP_DIR)/app && $(CARGO) test --target wasm32-unknown-unknown

# Format code for both projects
.PHONY: fmt
fmt:
	cd $(APP_DIR)/backend && $(CARGO) fmt
	cd $(APP_DIR)/app && $(CARGO) fmt

# Run clippy for both projects
.PHONY: lint
lint:
	cd $(APP_DIR)/backend && $(CARGO) clippy
	cd $(APP_DIR)/app && $(CARGO) clippy --target wasm32-unknown-unknown

# Watch Tailwind CSS
.PHONY: tailwind-watch
tailwind-watch:
	@echo "Watching Tailwind CSS..."
	@echo "Current directory: $(CURDIR)"
	@echo "APP_DIR: $(APP_DIR)"
	@echo "Input file: $(TAILWIND_INPUT)"
	@echo "Output file: $(TAILWIND_OUTPUT)"
	@echo "Listing contents of $(CURDIR)/$(APP_DIR)/app/public/style:"
	@ls -l $(CURDIR)/$(APP_DIR)/app/public/style
	@if [ -z "$(TAILWIND_INPUT)" ]; then \
		echo "Error: No CSS file found in $(CURDIR)/$(APP_DIR)/app/public/style"; \
		exit 1; \
	fi
	cd $(APP_DIR)/app && $(TAILWIND) -i $(TAILWIND_INPUT) -o $(TAILWIND_OUTPUT) --watch

# Development setup (using Docker)
.PHONY: dev
dev:
	@echo "Starting development environment..."
	$(MAKE) docker-build
	$(MAKE) -j2 docker-run tailwind-watch

# Local development setup (not using Docker)
.PHONY: dev-local
dev-local:
	@echo "Starting local development environment..."
	$(MAKE) -j3 backend-run-local frontend-run-local tailwind-watch

# Production build
.PHONY: prod
prod: build
	$(DOCKER_COMPOSE) run --rm app cargo leptos build --release
	cd $(APP_DIR)/app && $(TAILWIND) -i $(TAILWIND_INPUT) -o ./target/site/style/output.css --minify

# Help target
.PHONY: help
help:
	@echo "Available targets:"
	@echo "  all               - Build and run the project (default)"
	@echo "  build             - Build Docker containers"
	@echo "  run               - Run Docker containers"
	@echo "  down              - Stop and remove Docker containers"
	@echo "  clean             - Clean up Docker resources"
	@echo "  docker-build      - Build the Docker image"
	@echo "  docker-run        - Run the Docker container"
	@echo "  backend-build-local  - Build the backend project locally"
	@echo "  frontend-build-local - Build the frontend project locally"
	@echo "  backend-run-local    - Run the backend project locally"
	@echo "  frontend-run-local   - Run the frontend project locally"
	@echo "  run-both-local    - Run both frontend and backend locally"
	@echo "  test              - Run tests for both projects"
	@echo "  fmt               - Format code for both projects"
	@echo "  lint              - Run clippy for both projects"
	@echo "  tailwind-watch    - Watch Tailwind CSS"
	@echo "  dev               - Set up development environment (Docker)"
	@echo "  dev-local         - Set up local development environment"
	@echo "  prod              - Build for production"
	@echo "  help              - Show this help message"
