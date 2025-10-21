# Rust Axum API Implementation

This is a Rust implementation of the API using the Axum web framework, equivalent to the Python FastAPI implementation.

## Features

- **Authentication**: JWT-based authentication with bcrypt password hashing
- **User Management**: Admin-only CRUD operations for users
- **Posts**: Create, read, list, and delete posts (users can only delete their own posts)
- **Comments**: Create and list comments on posts
- **Likes**: Like and unlike posts with conflict detection
- **Database**: PostgreSQL with connection pooling using SQLx
- **Error Handling**: Comprehensive error handling with proper HTTP status codes
- **Logging**: Structured logging with tracing

## API Endpoints

### Authentication
- `POST /auth/login` - Login with email/password
- `GET /auth/me` - Get current user info (requires auth)

### Users (Admin only)
- `POST /users` - Create a new user
- `GET /users` - List all users (with pagination)
- `GET /users/{userId}` - Get user by ID
- `PUT /users/{userId}` - Update user
- `DELETE /users/{userId}` - Delete user

### Posts
- `POST /posts` - Create a new post (requires auth)
- `GET /posts` - List all posts (with pagination, public)
- `GET /posts/{post_id}` - Get post by ID (public)
- `DELETE /posts/{post_id}` - Delete post (author only)

### Comments
- `POST /posts/{post_id}/comments` - Create comment (requires auth)
- `GET /posts/{post_id}/comments` - List comments (public)

### Likes
- `POST /posts/{post_id}/like` - Like a post (requires auth)
- `DELETE /posts/{post_id}/like` - Unlike a post (requires auth)

## Configuration

Environment variables:
- `DATABASE_URL`: PostgreSQL connection string (default: `postgresql://apibench:apibench_password@localhost:15432/apibench`)
- `JWT_SECRET`: Secret key for JWT tokens (default: `dev-secret`)
- `JWT_EXPIRE_MINUTES`: JWT token expiration time in minutes (default: `60`)

## Running the Server

1. Make sure you have Rust installed
2. Install dependencies:
   ```bash
   cargo build
   ```
3. Run the server:
   ```bash
   # For development
   cargo run
   
   # For maximum performance (recommended for benchmarking)
   cargo run --release
   ```

The server will start on `http://0.0.0.0:3000`.

## Database

This implementation uses the same PostgreSQL database schema as the Python version. Make sure to run the database migrations in the `database/migrations/` directory.

## Architecture

- **main.rs**: Server setup, routing, and middleware configuration
- **handlers.rs**: HTTP request handlers for all endpoints
- **models.rs**: Request/response models and database row structs
- **auth.rs**: Authentication logic, JWT handling, and password hashing
- **error.rs**: Error types and HTTP response conversion
- **sql.rs**: SQL query constants loaded at compile time

## Performance Features

- Connection pooling with SQLx (20 max connections)
- Compile-time SQL query validation
- Async bcrypt password hashing with threadpool offloading (prevents blocking)
- Optimized release build with LTO and single codegen unit
- Minimal logging overhead in production
- CORS support for web clients

## Performance Notes

The initial slow performance was caused by:
1. **Blocking bcrypt operations** - Fixed by using `tokio::task::spawn_blocking`
2. **Small connection pool** - Increased from 5 to 20 connections
3. **Debug logging overhead** - Reduced to info level
4. **Debug build** - Use `cargo run --release` for benchmarking

These optimizations should provide **10-100x performance improvement** over the initial implementation.

## Differences from Python Version

- Uses compile-time SQL query inclusion for better performance
- More explicit error handling with custom error types
- Connection pooling is handled by SQLx instead of asyncpg
- Middleware architecture is different but functionally equivalent
- Uses Rust's type system for better compile-time safety
