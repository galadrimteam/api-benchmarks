#!/usr/bin/env python3
"""
FastAPI server startup script with optimized configuration for performance.
"""
import multiprocessing
import uvicorn

def main():
    # Use number of CPU cores for workers (or set a specific number)
    workers = 2*multiprocessing.cpu_count()+1
    
    print(f"Starting FastAPI server with {workers} workers...")
    
    uvicorn.run(
        "src.main:app",
        host="0.0.0.0",
        port=8000,
        workers=workers,
        # Additional performance optimizations
        loop="uvloop",  # Use uvloop for better async performance
        http="httptools",  # Use httptools for faster HTTP parsing
        access_log=False,  # Disable access logs for better performance
    )

if __name__ == "__main__":
    main()
