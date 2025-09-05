# NX Health Checker

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.87+-orange.svg)](https://www.rust-lang.org/)
[![Docker](https://img.shields.io/badge/docker-ready-blue.svg)](https://www.docker.com/)
[![Prometheus](https://img.shields.io/badge/prometheus-enabled-red.svg)](https://prometheus.io/)

A high-performance, containerized health monitoring system built with Rust and Axum. Monitor your services' health status, track response times, and collect metrics with Prometheus integration.

## 🚀 Features

- **Real-time Health Monitoring**: Monitor multiple services with customizable check intervals
- **RESTful API**: Complete CRUD operations for service management
- **Prometheus Integration**: Built-in metrics collection and export
- **Docker Ready**: Multi-stage Docker build with health checks
- **Web Dashboard**: Modern, responsive UI for service management
- **Session Authentication**: Secure login system with password management
- **SQLite Database**: Lightweight, embedded database for data persistence
- **Background Collectors**: Automated health checks with configurable intervals

## 📋 Table of Contents

- [Installation](#installation)
- [Quick Start](#quick-start)
- [Configuration](#configuration)
- [API Documentation](#api-documentation)
- [Docker Deployment](#docker-deployment)
- [Monitoring](#monitoring)
- [Development](#development)
- [Contributing](#contributing)
- [License](#license)

## 🛠 Installation

### Prerequisites

- **Rust 1.87+**: [Install Rust](https://rustup.rs/)
- **Docker** (optional): [Install Docker](https://docs.docker.com/get-docker/)
- **SQLite3**: Usually pre-installed on most systems

### From Source

```bash
# Clone the repository
git clone https://github.com/next-era/nx-health-checker.git
cd nx-health-checker

# Build the application
cargo build --release

# Run the application
cargo run
```

### Using Docker

```bash
# Build the Docker image
docker build -t nx-health-checker:latest .

# Run with Docker Compose
docker-compose up -d
```

## 🚀 Quick Start

### 1. Start the Application

```bash
# Using Cargo
cargo run

# Using Docker Compose
docker-compose up -d
```

### 2. Access the Dashboard

Open your browser and navigate to:
- **Web Interface**: http://localhost:3030
- **Health Check**: http://localhost:3030/health
- **Prometheus Metrics**: http://localhost:3030/metrics

### 3. Default Login

- **Username**: `admin`
- **Password**: `admin`

> ⚠️ **Security Note**: Change the default credentials in production!

### 4. Add Your First Service

1. Click "Add Service" in the dashboard
2. Fill in the service details:
   - **Service Name**: `My API`
   - **Health Check URL**: `https://api.example.com/health`
   - **Check Interval**: `30` (seconds)
3. Click "Save Changes"

## ⚙️ Configuration

### Environment Variables

Create a `config.env` file or set environment variables:

```bash
# Database Configuration
DATABASE_URL=sqlite:health_check.db?mode=rwc

# Default User Credentials
DEFAULT_USERNAME=admin
DEFAULT_PASSWORD=admin

# Server Configuration
HOST=0.0.0.0
PORT=3030

# Prometheus Configuration
PROMETHEUS_URL=http://localhost:9090
PROMETHEUS_ENABLED=true
PROMETHEUS_SCRAPE_INTERVAL=15

# Logging Configuration
RUST_LOG=info

# Session Configuration
SESSION_SECRET=your-secret-key-here-change-in-production
SESSION_TIMEOUT=3600

# Health Check Configuration
HEALTH_CHECK_TIMEOUT=30
HEALTH_CHECK_RETRIES=3
```

### Docker Configuration

The application supports Docker deployment with the following features:

- **Multi-stage build** for optimized image size
- **Non-root user** (`nxera`) for security
- **Health checks** for container monitoring
- **Volume mounting** for persistent data
- **Environment variable** configuration

## 📚 API Documentation

### Authentication

All API endpoints (except health checks) require authentication via session cookies.

### Service Management

#### Get All Services
```http
GET /api/services
```

#### Create Service
```http
POST /api/services
Content-Type: application/json

{
  "service_name": "My API",
  "healthcheck_url": "https://api.example.com/health",
  "healthcheck_duration_seconds": 30
}
```

#### Update Service
```http
PUT /api/services/{id}
Content-Type: application/json

{
  "service_name": "Updated API",
  "healthcheck_url": "https://api.example.com/health",
  "healthcheck_duration_seconds": 60
}
```

#### Delete Service
```http
DELETE /api/services/{id}
```

### Health Checks

#### Check Single Service
```http
GET /api/services/{id}/health
```

#### Check All Services
```http
GET /api/services/health
```

#### Application Health Check
```http
GET /health
```

### Metrics

#### Prometheus Metrics
```http
GET /metrics
```

#### Restart Collectors
```http
POST /api/metrics/restart
```

### User Management

#### Change Password
```http
POST /api/change-password
Content-Type: application/json

{
  "current_password": "old_password",
  "new_password": "new_password",
  "confirm_password": "new_password"
}
```

## 🐳 Docker Deployment

### Using Docker Compose

```bash
# Start the application
docker-compose up -d

# Start with Prometheus monitoring
docker-compose --profile monitoring up -d

# View logs
docker-compose logs -f nx-health-checker

# Stop the application
docker-compose down
```

### Using Docker

```bash
# Build the image
docker build -t nx-health-checker:latest .

# Run the container
docker run -d \
  --name nx-health-checker \
  -p 3030:3030 \
  -v $(pwd)/data:/app/data \
  -e RUST_LOG=info \
  -e DATABASE_URL=sqlite:/app/data/health_check.db?mode=rwc \
  -e DEFAULT_USERNAME=admin \
  -e DEFAULT_PASSWORD=admin \
  nx-health-checker:latest
```

### Docker Features

- **Health Checks**: Automatic container health monitoring
- **Volume Mounting**: Persistent database storage
- **Environment Configuration**: Flexible configuration via environment variables
- **Security**: Non-root user execution
- **Optimization**: Multi-stage build for smaller image size

## 📊 Monitoring

### Prometheus Integration

The application exposes Prometheus metrics at `/metrics`:

- **service_status**: Service health status (0 = down, 1 = up)
- **service_latency_seconds**: Response time for health checks
- **service_checks_total**: Total number of health checks performed

### Grafana Dashboard

Use the provided Prometheus metrics to create Grafana dashboards for:

- Service availability over time
- Response time trends
- Health check success rates
- Service uptime statistics

### Health Check Endpoints

- **Application Health**: `GET /health` - Simple application health check
- **Service Health**: `GET /api/services/health` - Comprehensive service health status
- **Individual Service**: `GET /api/services/{id}/health` - Single service health check

## 🛠 Development

### Project Structure

```
nx_health_checker/
├── src/
│   ├── main.rs              # Application entry point
│   ├── config.rs            # Configuration management
│   ├── database.rs          # Database initialization and management
│   ├── auth.rs              # Authentication and session management
│   ├── handlers.rs          # HTTP request handlers
│   ├── models.rs            # Data models and structures
│   ├── routes.rs            # Route definitions
│   ├── metrics.rs           # Prometheus metrics
│   └── collector.rs         # Background health check collectors
├── static/                  # Web UI files
│   ├── dashboard.html       # Main dashboard
│   ├── login.html           # Login page
│   ├── change-password.html # Password change page
│   └── assets/              # Static assets (CSS, JS, images)
├── Dockerfile               # Docker build configuration
├── docker-compose.yml       # Docker Compose configuration
├── config.env               # Environment configuration
└── Cargo.toml               # Rust dependencies
```

### Building from Source

```bash
# Install dependencies
cargo build

# Run in development mode
cargo run

# Run tests
cargo test

# Build for production
cargo build --release
```

### Adding New Features

1. **New API Endpoints**: Add routes in `src/routes.rs` and handlers in `src/handlers.rs`
2. **Database Changes**: Update `src/database.rs` for schema changes
3. **Frontend Changes**: Modify files in `static/` directory
4. **Configuration**: Add new environment variables in `src/config.rs`

## 🤝 Contributing

We welcome contributions! Please follow these steps:

1. **Fork the repository**
2. **Create a feature branch**: `git checkout -b feature/amazing-feature`
3. **Commit your changes**: `git commit -m 'Add amazing feature'`
4. **Push to the branch**: `git push origin feature/amazing-feature`
5. **Open a Pull Request**

### Development Guidelines

- Follow Rust best practices and conventions
- Add tests for new functionality
- Update documentation for API changes
- Ensure Docker builds successfully
- Test with different environment configurations

### Code Style

- Use `cargo fmt` for code formatting
- Use `cargo clippy` for linting
- Follow the existing code structure and patterns
- Add meaningful comments for complex logic

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🏢 About Next Era

**Next Era** is a technology company focused on building innovative solutions for modern infrastructure and monitoring needs.

- **Website**: [nxera.top](https://nxera.top)
- **Email**: [nxera.solutions@gmail.com](mailto:nxera.solutions@gmail.com)
- **GitHub**: [Next Era](https://github.com/next-era)

### Our Mission

We believe in creating robust, scalable, and user-friendly tools that help organizations maintain healthy and reliable systems. NX Health Checker is part of our commitment to providing enterprise-grade monitoring solutions.

## 🆘 Support

### Getting Help

- **Documentation**: Check this README and inline code comments
- **Issues**: Report bugs and request features on [GitHub Issues](https://github.com/next-era/nx-health-checker/issues)
- **Email**: Contact us at [nxera.solutions@gmail.com](mailto:nxera.solutions@gmail.com)

### Common Issues

#### Application Won't Start
- Check if port 3030 is available
- Verify database permissions
- Check environment variable configuration

#### Health Checks Failing
- Verify service URLs are accessible
- Check network connectivity
- Review health check timeout settings

#### Docker Issues
- Ensure Docker is running
- Check volume mount permissions
- Verify environment variable syntax

## 🎯 Roadmap

### Upcoming Features

- [ ] **Multi-user Support**: Role-based access control
- [ ] **Alerting System**: Email/Slack notifications for service failures
- [ ] **Service Dependencies**: Monitor service relationships
- [ ] **Custom Health Check Scripts**: Support for custom health check logic
- [ ] **API Rate Limiting**: Protect against abuse
- [ ] **Service Discovery**: Automatic service detection
- [ ] **Historical Data**: Long-term health check history
- [ ] **Mobile App**: Native mobile application

### Performance Improvements

- [ ] **Database Optimization**: Query performance improvements
- [ ] **Caching Layer**: Redis integration for better performance
- [ ] **Load Balancing**: Support for multiple instances
- [ ] **Metrics Optimization**: More efficient Prometheus metrics

---

**Built with ❤️ by [Next Era](https://nxera.top)**

For more information, visit our website or contact us at [nxera.solutions@gmail.com](mailto:nxera.solutions@gmail.com).
