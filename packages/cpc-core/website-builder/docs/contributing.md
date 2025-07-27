# Contributing to the Website Builder Module

This document explains how to contribute to the website builder module.

## Getting Started

1. Fork the repository
2. Clone your fork
3. Create a new branch for your feature or bug fix
4. Make your changes
5. Write tests for your changes
6. Run the tests to ensure they pass
7. Commit your changes
8. Push to your fork
9. Create a pull request

## Code Structure

The module follows a hexagonal architecture with the following directories:

- `src/domain/` - Core business logic and entities
- `src/application/` - Use cases and business logic orchestration
- `src/infrastructure/` - Technical implementations
- `src/web/` - Web interface adapters

## Coding Standards

- Follow the Rust coding standards and best practices
- Write idiomatic Rust code
- Use descriptive names for variables, functions, and types
- Write documentation for public APIs
- Write tests for new functionality

## Testing

- Write unit tests for domain logic
- Write integration tests for repository layer
- Write tests for GraphQL resolvers
- Ensure all tests pass before submitting a pull request

To run the tests:

```bash
cargo test -p cpc-website-builder
```

## Documentation

- Update the README.md file if you add new features
- Add documentation for new public APIs
- Update the usage guide if needed
- Add examples for new functionality

## Pull Request Process

1. Ensure your code follows the coding standards
2. Ensure all tests pass
3. Ensure your commit messages are clear and descriptive
4. Create a pull request with a clear description of your changes
5. Request a review from the maintainers

## Reporting Issues

If you find a bug or have a feature request, please create an issue on the GitHub repository with a clear description of the problem or feature.

## Code of Conduct

Please follow the project's code of conduct when contributing to this module.