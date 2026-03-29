# Deployment Guide

This document outlines the end-to-end process for deploying the Ruuderie AI application in both local development environments (using OrbStack/Docker Desktop) and production Kubernetes clusters.

## Architecture Overview

The system runs as a Rust/Leptos application backed by a PostgreSQL database. It is containerized and deployed using Kubernetes manifests via Kustomize.

**Configuration Structure (`k8s/`)**
- `k8s/base/`: Contains the core `Deployment`, `Service`, and `Namespace` components.
- `k8s/overlays/local/`: Environment overlay for local dev, including local Postgres, Secrets, and Ingress routing.
- `k8s/overlays/prod/`: Environment overlay for production environments.

## Prerequisite: Building the Docker Image
Before deploying to any environment, the application must be compiled and bundled into a Docker image.

```bash
# From the root of the repository
docker build -t ruuderie_ai-app:latest .
```
> [!NOTE]
> If using OrbStack locally, you do not need to push this image to an external registry. OrbStack's Kubernetes engine automatically shares and pulls images from its native Docker engine. For production, you will need to tag and push this to a registry (e.g., ECR, Docker Hub) and update the `k8s/base/app.yaml` image reference.

---

## 1. Local Development Deployment (OrbStack)

The local environment is configured to run flawlessly on your machine using strict `localhost` settings to ensure Passkeys (WebAuthn) work cleanly.

### Setup and Apply
1. Set your `kubectl` context to your local cluster (e.g., OrbStack or Docker Desktop):
   ```bash
   kubectl config use-context orbstack
   ```
2. Apply the Kustomize local overlay:
   ```bash
   kubectl apply -k k8s/overlays/local
   ```
3. Watch the pods initialize until both your database and the `ruuderie-app` show as `Running`:
   ```bash
   kubectl get pods -n ruuderie-ai -w
   ```

### Accessing the Local Environment (Passkeys Requirement)
> [!IMPORTANT]
> Passkeys enforce a strict "Secure Context" requirement. To develop Passkeys locally without complicated HTTPS certificates, we rely on the browser's hardcoded security exception for `http://localhost`.

To securely route traffic to `localhost` and perfectly match your `config.yaml` environment variables:
1. Initialize a port-forward tunnel into the application service:
   ```bash
   kubectl port-forward svc/ruuderie-app 3000:80 -n ruuderie-ai
   ```
2. Keep the terminal running, and open your browser to **http://localhost:3000**.

### Refreshing Local Changes

Unlike running `cargo leptos watch` natively on your host machine (which provides hot-reloading), running inside Kubernetes means your application is completely isolated within a container. Modifying your local Rust source files will **not** automatically update the running app.

Whenever you make code changes, you must manually rebuild the Docker container and instruct Kubernetes to reboot the pods so they launch with the fresh code:

1. Rebuild the local Docker image:
   ```bash
   docker build -t ruuderie_ai-app:latest .
   ```
2. Force Kubernetes to restart the pods to pull the fresh `latest` image:
   ```bash
   kubectl rollout restart deployment/ruuderie-app -n ruuderie-ai
   ```

---

## 2. Production Deployment

In a production setting, you will have an external HTTPS domain (e.g., `https://ruuderie.ai`) and likely an external managed database (e.g., AWS RDS).

### Step 1: Configure Production Secrets
Ensure your `k8s/overlays/prod/config.yaml` or equivalent secret manager (like ExternalSecrets) is configured with your production variables.

You **must** update the Passkey environment variables to align exactly with your production domain:
```yaml
stringData:
  DATABASE_URL: "postgres://user:password@production-db-host:5432/ruuderie_ai"
  RP_ORIGIN: "https://your-production-domain.com"
  RP_ID: "your-production-domain.com"
  LEPTOS_SITE_ADDR: "0.0.0.0:3000"
  LEPTOS_ENV: "PROD"
```
> [!WARNING]
> WebAuthn requires the `RP_ORIGIN` to explicitly include the `https://` scheme and port if non-standard, and `RP_ID` must strictly be the domain suffix without the scheme or port. Mismatches here will result in `Device setup rejected` errors from the browser or `Validation failed` errors from the server.

### Step 2: Apply Production Manifests

```bash
# Make sure context targets production
kubectl config use-context your-production-cluster

# Apply the manifests
kubectl apply -k k8s/overlays/prod
```

### Step 3: Zero-Downtime Rollouts
If you update your application code and push a new `ruuderie_ai-app:latest` image, you must restart the deployment so your pods pull the fresh code. Because your `app.yaml` references `latest`, rolling out a restart is the standard update process:

```bash
kubectl rollout restart deployment/ruuderie-app -n ruuderie-ai
```
> [!TIP]
> You may see `Error` states on your old terminating pods during a rollout. This is perfectly normal. By default, this Rust app does not catch `SIGTERM` signals for graceful shutdowns, so Kubernetes cleanly `SIGKILL`s them after an automatic 30-second timeout.
