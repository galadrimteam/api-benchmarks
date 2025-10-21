# Python FastAPI Implementation

This is a FastAPI implementation of the API benchmarks with performance optimizations.

## Performance Optimizations

### 1. Async bcrypt with Threadpool
- `bcrypt.checkpw` is offloaded to a threadpool to prevent blocking the event loop
- This prevents CPU-bound password hashing from delaying other async operations

### 2. Multi-worker Configuration
- Use the provided `start_server.py` script for optimal performance
- Automatically scales to use all available CPU cores
- Includes uvloop and httptools for better async performance

## Running the Server

### Development (single worker)
```bash
cd src/python-fastapi
uv run uvicorn src.main:app --host 0.0.0.0 --port 8000 --reload
```

### Production (multi-worker, optimized)
```bash
cd src/python-fastapi
uv run python start_server.py
```

### Manual multi-worker setup
```bash
cd src/python-fastapi
uv run uvicorn src.main:app --host 0.0.0.0 --port 8000 --workers 4
```

## Performance Notes

- **Login latency**: With threadpool optimization, login should be ~220ms (bcrypt time) instead of 400-900ms
- **Concurrency**: Multi-worker setup allows true parallelism for CPU-bound operations
- **Database**: Connection pooling is configured (min=1, max=5 per worker)
- **bcrypt cost**: Currently set to 12 (strong security). For testing, consider lowering to 10 or 8

## Dependencies

See `pyproject.toml` for the complete list. Key dependencies:
- `fastapi[standard]` - Web framework with async support
- `asyncpg` - Async PostgreSQL driver
- `bcrypt` - Password hashing
- `PyJWT` - JWT token handling
