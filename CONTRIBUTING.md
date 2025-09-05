# Contributing to NX Health Checker

Thank you for your interest in contributing to NX Health Checker! This document provides guidelines and information for contributors.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Setup](#development-setup)
- [Contributing Process](#contributing-process)
- [Coding Standards](#coding-standards)
- [Testing](#testing)
- [Documentation](#documentation)
- [Release Process](#release-process)

## Code of Conduct

This project follows the [Contributor Covenant Code of Conduct](CODE_OF_CONDUCT.md). By participating, you agree to uphold this code.

## Getting Started

### Prerequisites

- **Rust 1.87+**: [Install Rust](https://rustup.rs/)
- **Docker** (optional): [Install Docker](https://docs.docker.com/get-docker/)
- **Git**: [Install Git](https://git-scm.com/)

### Fork and Clone

1. Fork the repository on GitHub
2. Clone your fork locally:
   ```bash
   git clone https://github.com/your-username/nx-health-checker.git
   cd nx-health-checker
   ```
3. Add the upstream repository:
   ```bash
   git remote add upstream https://github.com/next-era/nx-health-checker.git
   ```

## Development Setup

### Local Development

1. **Install Dependencies**:
   ```bash
   cargo build
   ```

2. **Run the Application**:
   ```bash
   cargo run
   ```

3. **Run Tests**:
   ```bash
   cargo test
   ```

4. **Format Code**:
   ```bash
   cargo fmt
   ```

5. **Lint Code**:
   ```bash
   cargo clippy
   ```

### Docker Development

1. **Build Docker Image**:
   ```bash
   docker build -t nx-health-checker:dev .
   ```

2. **Run with Docker Compose**:
   ```bash
   docker-compose up -d
   ```

3. **View Logs**:
   ```bash
   docker-compose logs -f
   ```

## Contributing Process

### 1. Create a Feature Branch

```bash
git checkout -b feature/your-feature-name
```

### 2. Make Your Changes

- Write clean, readable code
- Follow the coding standards
- Add tests for new functionality
- Update documentation as needed

### 3. Test Your Changes

```bash
# Run all tests
cargo test

# Run specific tests
cargo test test_name

# Test with Docker
docker-compose up -d
curl http://localhost:3030/health
```

### 4. Commit Your Changes

```bash
git add .
git commit -m "feat: add new feature description"
```

**Commit Message Format:**
- `feat:` New feature
- `fix:` Bug fix
- `docs:` Documentation changes
- `style:` Code style changes
- `refactor:` Code refactoring
- `test:` Adding tests
- `chore:` Maintenance tasks

### 5. Push and Create Pull Request

```bash
git push origin feature/your-feature-name
```

Then create a Pull Request on GitHub.

## Coding Standards

### Rust Code Style

1. **Use `cargo fmt`** for consistent formatting
2. **Use `cargo clippy`** for linting
3. **Follow Rust naming conventions**:
   - `snake_case` for variables and functions
   - `PascalCase` for types and traits
   - `SCREAMING_SNAKE_CASE` for constants

### Code Organization

1. **Module Structure**:
   - Keep related functionality together
   - Use clear module names
   - Document public APIs

2. **Error Handling**:
   - Use `Result<T, E>` for fallible operations
   - Provide meaningful error messages
   - Use `anyhow` for application errors

3. **Async Code**:
   - Use `async/await` consistently
   - Handle errors properly in async functions
   - Use appropriate async traits

### Example Code Structure

```rust
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Service {
    pub id: i64,
    pub name: String,
    pub url: String,
}

impl Service {
    pub async fn create(pool: &SqlitePool, name: String, url: String) -> Result<Self> {
        // Implementation
    }
}
```

## Testing

### Unit Tests

Write unit tests for individual functions and methods:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_service_creation() {
        // Test implementation
    }
}
```

### Integration Tests

Create integration tests in the `tests/` directory:

```rust
// tests/integration_test.rs
use nx_health_checker::*;

#[tokio::test]
async fn test_api_endpoint() {
    // Test implementation
}
```

### Test Coverage

- Aim for high test coverage
- Test both success and failure cases
- Test edge cases and error conditions
- Mock external dependencies when appropriate

## Documentation

### Code Documentation

1. **Document Public APIs**:
   ```rust
   /// Creates a new service with the given name and URL.
   /// 
   /// # Arguments
   /// 
   /// * `name` - The service name
   /// * `url` - The health check URL
   /// 
   /// # Returns
   /// 
   /// Returns a `Result` containing the created service or an error.
   pub async fn create_service(name: String, url: String) -> Result<Service> {
       // Implementation
   }
   ```

2. **Update README.md** for user-facing changes
3. **Update API.md** for API changes
4. **Add inline comments** for complex logic

### Documentation Standards

- Use clear, concise language
- Provide examples where helpful
- Keep documentation up-to-date
- Use proper markdown formatting

## Release Process

### Version Numbering

We follow [Semantic Versioning](https://semver.org/):
- **MAJOR**: Breaking changes
- **MINOR**: New features (backward compatible)
- **PATCH**: Bug fixes (backward compatible)

### Release Checklist

1. **Update Version**:
   - Update `Cargo.toml` version
   - Update `CHANGELOG.md`
   - Update documentation

2. **Test Release**:
   - Run all tests
   - Test Docker build
   - Test deployment

3. **Create Release**:
   - Create Git tag
   - Push to repository
   - Create GitHub release

## Issue Reporting

### Bug Reports

When reporting bugs, please include:

1. **Environment Information**:
   - OS and version
   - Rust version
   - Docker version (if applicable)

2. **Steps to Reproduce**:
   - Clear, numbered steps
   - Expected vs actual behavior
   - Error messages or logs

3. **Additional Context**:
   - Screenshots if applicable
   - Related issues or discussions

### Feature Requests

When requesting features, please include:

1. **Problem Description**:
   - What problem does this solve?
   - Why is this feature needed?

2. **Proposed Solution**:
   - How should this feature work?
   - Any design considerations?

3. **Alternatives Considered**:
   - What other solutions were considered?
   - Why is this approach preferred?

## Community Guidelines

### Communication

- Be respectful and inclusive
- Use clear, constructive language
- Help others learn and grow
- Follow the Code of Conduct

### Getting Help

- Check existing issues and discussions
- Ask questions in GitHub Discussions
- Join our community channels (if available)
- Contact us at [nxera.solutions@gmail.com](mailto:nxera.solutions@gmail.com)

## Recognition

Contributors will be recognized in:

- **CONTRIBUTORS.md** file
- **Release notes** for significant contributions
- **GitHub contributors** page

## License

By contributing to this project, you agree that your contributions will be licensed under the MIT License.

## Contact

- **Website**: [nxera.top](https://nxera.top)
- **Email**: [nxera.solutions@gmail.com](mailto:nxera.solutions@gmail.com)
- **GitHub**: [Next Era](https://github.com/next-era)

Thank you for contributing to NX Health Checker! 🚀
