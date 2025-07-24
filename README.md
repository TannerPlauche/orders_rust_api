# Rust API - Order Management System

A comprehensive REST API built with Rust using Axum framework, SQLite database, and comprehensive validation. This API provides full CRUD operations for managing orders with persistent storage.

## ⚡ Quick Start

```bash
# Clone and navigate to project
git clone <repository-url>
cd rustapi

# Run the startup script (builds, tests, and starts server)
./start.sh
```

Server will be available at `http://localhost:3000`

**📖 View API Documentation**: http://localhost:3000/docs

## 🚀 Features

- **RESTful API**: Complete CRUD operations for order management
- **OpenAPI Documentation**: Interactive Swagger UI for API exploration and testing
- **Database Persistence**: SQLite database with connection pooling
- **Input Validation**: Comprehensive validation with detailed error messages
- **Error Handling**: Structured error responses with appropriate HTTP status codes
- **Testing**: 72 comprehensive tests covering all functionality
- **Async**: Built with Tokio async runtime for high performance
- **Type Safety**: Leverages Rust's type system for reliability

**💡 Tip**: You can test all endpoints interactively using the Swagger UI at http://localhost:3000/swagger-ui

## 📚 API Documentation

### Interactive Documentation
Once the server is running, you can access the interactive API documentation:

- **Swagger UI**: http://localhost:3000/docs
- **OpenAPI JSON**: http://localhost:3000/api-docs/openapi.json

The Swagger UI provides:
- Interactive API testing
- Complete endpoint documentation
- Request/response examples
- Schema definitions
- Try-it-out functionality for all endpoints

### API Endpoints


| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/orders` | Get all orders |
| `POST` | `/orders` | Create a new order |
| `GET` | `/orders/{id}` | Get order by ID |
| `PUT` | `/orders/{id}` | Update an order |
| `PATCH` | `/orders/{id}/status` | Update order status |
| `DELETE` | `/orders/{id}` | Delete an order |

## 📦 Order Schema

```json
{
  "id": 1,
  "item": "Product Name",
  "status": "pending",
  "quantity": 5
}
```

### Valid Status Values
- `pending`
- `processing` 
- `shipped`
- `delivered`
- `cancelled`

### Validation Rules
- **ID**: Must be greater than 0, unique
- **Item**: 1-100 characters, cannot be empty or whitespace only
- **Status**: Must be one of the valid status values
- **Quantity**: 1-1000, must be greater than 0

## 🛠️ Prerequisites

- **Rust**: 1.70+ (install from [rustup.rs](https://rustup.rs/))
- **Git**: For cloning the repository

## 🚀 Getting Started

### 1. Clone the Repository

```bash
git clone <repository-url>
cd rustapi
```

### 2. Install Dependencies

The dependencies will be automatically installed when building the project. Key dependencies include:

### Key Dependencies

- `axum` - Web framework
- `tokio` - Async runtime
- `sqlx` - Database toolkit
- `serde` - Serialization
- `uuid` - Unique identifiers
- `utoipa` - OpenAPI documentation generation
- `utoipa-swagger-ui` - Swagger UI integration

### 3. Build the Project

```bash
cargo build
```

### 4. Run the Application

#### Option 1: Using the Startup Script (Recommended)

```bash
./start.sh
```

This script will:
- Verify your environment
- Build the project
- Run all tests
- Start the server

#### Option 2: Manual Startup

```bash
cargo run
```

The server will start on `http://localhost:3000`

You should see output similar to:
```
Database initialized successfully
Server running on http://0.0.0.0:3000
```

## 🧪 Testing

### Run All Tests

```bash
cargo test
```

Expected output:
```
running 72 tests
........................................................................
test result: ok. 72 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

### Run Specific Test Categories

```bash
# Handler tests only
cargo test handlers::tests

# Route tests only  
cargo test routes::tests

# Validation tests only
cargo test validators::tests

# Database tests only
cargo test utils::tests
```

### Run Tests with Output

```bash
cargo test -- --nocapture
```

## 🗂️ Project Structure

```
rustapi/
├── src/
│   ├── main.rs              # Application entry point
│   ├── handlers/            # HTTP request handlers
│   │   ├── mod.rs
│   │   ├── handlers.rs
│   │   └── handlers.tests.rs
│   ├── routes/              # Route definitions  
│   │   ├── mod.rs
│   │   ├── routes.rs
│   │   └── routes.tests.rs
│   ├── utils/               # Database utilities
│   │   ├── mod.rs
│   │   └── db_utils.rs
│   └── validators/          # Input validation
│       ├── mod.rs
│       ├── order_validator.rs
│       └── order_validator.tests.rs
├── Cargo.toml              # Dependencies and metadata
├── Cargo.lock              # Dependency lock file
├── orders.db               # SQLite database (auto-created)
└── README.md               # This file
```

## 🏗️ Architecture

### **Layer Architecture**

1. **Routes Layer** (`routes/`): HTTP routing and endpoint definitions
2. **Handlers Layer** (`handlers/`): Business logic and request processing  
3. **Validation Layer** (`validators/`): Input validation and error handling
4. **Database Layer** (`utils/`): Database operations and connection management

### **Key Technologies**

- **Axum**: Modern, ergonomic web framework for Rust
- **SQLx**: Async SQL toolkit with compile-time checked queries
- **Tokio**: Async runtime for high-performance networking
- **Serde**: Serialization framework for JSON handling
- **SQLite**: Lightweight, serverless database

## 🔧 Development

### Database Changes

The database schema is automatically created on startup. To modify:

1. Update the `Order` struct in `src/utils/db_utils.rs`
2. Modify the `CREATE TABLE` statement in `init_db()`
3. Update validation rules as needed

### Running in Development Mode

```bash
# Auto-reload on file changes (requires cargo-watch)
cargo install cargo-watch
cargo watch -x run

# Run with debug logging
RUST_LOG=debug cargo run
```

## 🚀 Production Deployment

### Environment Configuration

Create a `.env` file or set environment variables:

```bash
export DATABASE_URL="sqlite:production.db"
export RUST_LOG="info"
export PORT="3000"
```

### Build for Production

```bash
cargo build --release
./target/release/rustapi
```

---

**Built with ❤️ and Rust** 🦀
