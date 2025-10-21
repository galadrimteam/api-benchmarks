# Benchmark Runner

## Requirements
- Bun
- Python
- Node.js
- Rust
- PostgreSQL client
- Go
- uv
- Grafana k6

## Launch benchmarks

### (Optional) Start database from devcontainer docker-compose
```bash
docker compose up -f .devcontainer/docker-compose.yml -d db 
```

### (Optional) Pre-compile rust code to avoid timeout on first run

```bash
cd src/rust-axum && cargo build --release
```

### Run all the benchmarks

```bash
bun run scripts/k6/benchmark_runner.mjs --parallel
``` 

## See results

Open `dashboard.html` in your browser