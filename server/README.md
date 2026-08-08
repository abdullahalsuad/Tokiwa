# Tokiwa Backend

Tokiwa is a local-first, self-hosted time tracking engine written in Rust. It runs as a lightweight daemon on your local machine and exposes an HTTP API for local client interfaces.

## Prerequisites

- **OS**: Linux (X11 environment for window tracking capabilities)
- **Rust Toolchain**: `rustc` / `cargo` 1.75+
- **System Libraries**: `pkg-config`, `libX11`, `libxcb`

### Install Native Dependencies

#### Debian / Ubuntu
```bash
sudo apt update && sudo apt install -y build-essential pkg-config libx11-dev libxcb1-dev
```

#### Fedora / RHEL
```bash
sudo dnf install -y @development-tools pkgconf-pkg-config libX11-devel libxcb-devel
```

#### Arch Linux
```bash
sudo pacman -S --needed base-devel pkgconf libx11 libxcb
```

---

## Quick Start

### 1. Install Rust (if not already installed)

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"
```

### 2. Build & Run

```bash
# Clone and enter directory
cd tokiwa-backend

# Run in development mode
cargo run
```

### 3. Verify Health Endpoint

In another terminal window:

```bash
curl http://127.0.0.1:7823/api/health
```

Expected output:
```json
{"service":"tokiwa-backend","status":"ok","version":"0.1.0"}
```

---

## Development Commands

```bash
# Type check without emitting binaries
cargo check

# Build debug binary (target/debug/tokiwa-backend)
cargo build

# Build optimized production binary (target/release/tokiwa-backend)
cargo build --release

# Format code
cargo fmt

# Lint code
cargo clippy
```

---

## Configuration

Settings are managed via `config/config.toml`. If missing or invalid, default values are used automatically.

```toml
[server]
host = "127.0.0.1"
port = 7823

[tracking]
poll_interval_seconds = 2
idle_threshold_minutes = 5

[database]
path = "tokiwa.db"
```

### Logging

Log output is controlled via `RUST_LOG`:

```bash
RUST_LOG=tokiwa_backend=debug cargo run
```

---

## Project Structure

```
tokiwa-backend/
├── Cargo.toml          # Package manifest & dependencies
├── config/
│   └── config.toml     # Application configuration
└── src/
    ├── main.rs         # HTTP server entrypoint (Axum)
    ├── config.rs       # Config parser & fallback defaults
    ├── api/            # API handlers
    ├── db/             # Database storage layer
    └── watcher/        # Activity & window detection watcher
```

---

## API Endpoints

| Method | Route | Description |
|---|---|---|
| `GET` | `/api/health` | Service health status |
