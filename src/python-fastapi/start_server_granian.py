#!/usr/bin/env python3
"""
FastAPI server startup script using Granian ASGI server for high performance.

Granian is a Rust-based ASGI server that can provide better performance than uvicorn
in many scenarios, especially for CPU-intensive workloads and high concurrency.

Key advantages:
- Written in Rust for better performance
- Multi-threading support with RuntimeModes.mt
- Built-in HTTP/2 support
- Lower memory footprint
- Better handling of CPU-bound tasks
"""
import multiprocessing
from granian import Granian
from granian.constants import Interfaces, Loops, RuntimeModes, HTTPModes

def main():
    # Use number of CPU cores for workers (or set a specific number)
    workers = multiprocessing.cpu_count()
    
    print(f"Starting FastAPI server with Granian using {workers} workers...")
    
    # Create Granian server instance
    server = Granian(
        "src.main:app",
        address="0.0.0.0",
        port=8000,
        interface=Interfaces.ASGI,
        workers=workers,
        # Performance optimizations
        loop=Loops.uvloop,  # Use uvloop for better async performance
        runtime_mode=RuntimeModes.mt,  # Use multi-threading runtime mode
        # Disable access logs for better performance
        log_access=False,
        # Additional performance settings
        backlog=2048,  # Increase connection backlog
        http=HTTPModes.auto,  # Auto-detect HTTP mode
        # Enable WebSockets support
        websockets=True,
    )
    
    # Start the server
    server.serve()

if __name__ == "__main__":
    main()
