# Contributing to ChronoSub

Thank you for your interest in contributing to ChronoSub! We welcome contributions from everyone, whether you're fixing bugs, adding features, improving documentation, or reporting issues.

## 📋 Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Workflow](#development-workflow)
- [Coding Standards](#coding-standards)
- [Testing Guidelines](#testing-guidelines)
- [Documentation Standards](#documentation-standards)
- [Submitting Changes](#submitting-changes)
- [Reporting Bugs](#reporting-bugs)
- [Suggesting Features](#suggesting-features)

## 🤝 Code of Conduct

We are committed to providing a welcoming and inclusive environment. Please:

- Be respectful and constructive
- Welcome newcomers and help them learn
- Focus on what is best for the community
- Show empathy towards other community members

## 🚀 Getting Started

### Prerequisites

- **Rust**: Edition 2024 or later ([Install Rust](https://www.rust-lang.org/tools/install))
- **Git**: For version control
- **GitHub Account**: For submitting pull requests

### Setting Up Your Development Environment

```bash
# 1. Fork the repository on GitHub

# 2. Clone your fork
git clone https://github.com/YOUR_USERNAME/chrono-sub.git
cd chrono-sub

# 3. Add the original repository as upstream
git remote add upstream https://github.com/Irwnda/chrono-sub.git

# 4. Install dependencies
cargo build

# 5. Run tests to ensure everything works
cargo test

# 6. Create a new branch for your work
git checkout -b feature/your-feature-name
```

## 🔧 Development Workflow

### 1. Find Something to Work On

Check out our [Good First Issues](https://github.com/Irwnda/chrono-sub/issues?q=is%3Aissue+is%3Aopen+label%3A%22good+first+issue%22) or [Enhancement Ideas](https://github.com/Irwnda/chrono-sub/issues?q=is%3Aissue+is%3Aopen+label%3Aenhancement).

### 2. Write Code

Follow our [Coding Standards](#coding-standards) and make sure your code passes all tests.

### 3. Test Your Changes

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run clippy to catch common mistakes
cargo clippy -- -D warnings

# Format your code
cargo fmt
```

### 4. Commit Your Changes

We follow the [Conventional Commits](https://www.conventionalcommits.org/) specification:

```
<type>(<scope>): <description>

[optional body]

[optional footer(s)]
```

**Types:**
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting, etc.)
- `refactor`: Code refactoring
- `test`: Adding or updating tests
- `chore`: Maintenance tasks

**Examples:**
```bash
git commit -m "feat(subtitle): add support for ASS subtitle format"
git commit -m "fix(io): handle file paths with spaces correctly"
git commit -m "docs: update installation instructions"
```

### 5. Push and Create Pull Request

```bash
# Push to your fork
git push origin feature/your-feature-name

# Create a pull request on GitHub
```

## 📐 Coding Standards

### Rust Best Practices

1. **Follow Rust Conventions**
   - Use `cargo fmt` for consistent formatting
   - Use `cargo clippy` to catch common mistakes
   - Follow [The Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)

2. **Error Handling**
   - Use `Result` types for functions that can fail
   - Provide meaningful error messages
   - Use appropriate error types (e.g., `io::Error`, custom error types)

3. **Naming Conventions**
   - Functions and variables: `snake_case`
   - Types and structs: `PascalCase`
   - Constants: `SCREAMING_SNAKE_CASE`

4. **Documentation**
   - Document public APIs with `///` or `//!`
   - Include examples in documentation comments
   - Use `#[deprecated]` for deprecated items

### Code Organization

- **Modular Structure**: Keep modules focused and cohesive
- **Separation of Concerns**: Separate business logic from I/O operations
- **DRY Principle**: Don't repeat yourself - extract common functionality

Example:
```rust
/// Represents a subtitle timestamp with hours, minutes, seconds, and milliseconds
///
/// # Examples
/// ```
/// let time = SubTime::new(0, 1, 30, 500);
/// assert_eq!(time.to_string(), "00:01:30,500");
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct SubTime {
    pub hours: u64,
    pub minutes: u64,
    pub seconds: u64,
    pub milliseconds: u64,
}
```

## 🧪 Testing Guidelines

### Writing Tests

1. **Unit Tests**: Test individual functions and methods
2. **Integration Tests**: Test module interactions
3. **Edge Cases**: Test boundary conditions and error cases

### Test Organization

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subtime_add() {
        let time = SubTime::new(0, 0, 30, 0);
        let added = time.add(&SubTime::new(0, 0, 20, 0)).unwrap();
        assert_eq!(added.seconds, 50);
    }

    #[test]
    fn test_subtime_overflow() {
        let time = SubTime::new(23, 59, 59, 999);
        let result = time.add(&SubTime::new(0, 0, 0, 1));
        assert!(result.is_err());
    }
}
```

### Test Coverage Goals

- Aim for >80% code coverage
- Test all public APIs
- Test error conditions
- Test edge cases (empty files, malformed timestamps, etc.)

## 📚 Documentation Standards

### Code Documentation

- **Public Functions**: Must have doc comments
- **Complex Logic**: Add inline comments explaining the "why"
- **Examples**: Include runnable examples in doc comments

### README Updates

When adding features, update:
- Feature list
- Usage examples
- Installation instructions (if needed)

## 📤 Submitting Changes

### Pull Request Checklist

Before submitting, ensure:

- [ ] Code compiles without warnings (`cargo build`)
- [ ] All tests pass (`cargo test`)
- [ ] Code is formatted (`cargo fmt`)
- [ ] Clippy produces no warnings (`cargo clippy -- -D warnings`)
- [ ] New features include tests
- [ ] Documentation is updated
- [ ] Commit messages follow conventions

### Pull Request Template

When creating a PR, use this template:

```markdown
## Description
Brief description of changes

## Type of Change
- [ ] Bug fix
- [ ] New feature
- [ ] Breaking change
- [ ] Documentation update

## Testing
Describe testing performed

## Checklist
- [ ] Tests pass locally
- [ ] Documentation updated
- [ ] No warnings from clippy
```

## 🐛 Reporting Bugs

### Before Reporting

1. Check existing issues to avoid duplicates
2. Confirm the bug still exists in the latest version
3. Gather necessary information

### Bug Report Template

```markdown
## Description
Clear description of the bug

## To Reproduce
Steps to reproduce the behavior:
1. Run '...'
2. Select file '...'
3. Adjust time by '...'
4. See error

## Expected Behavior
What should happen

## Actual Behavior
What actually happens

## Environment
- OS: [e.g., macOS 14, Ubuntu 22.04]
- ChronoSub version: [e.g., 1.0.0]
- Rust version: [e.g., 1.75.0]

## Additional Context
Logs, screenshots, or other relevant info
```

## 💡 Suggesting Features

### Feature Request Guidelines

- Clearly describe the feature and its use case
- Explain why it would be useful
- Consider if it fits the project scope
- Suggest an API/design if possible

### Feature Request Template

```markdown
## Feature Description
Detailed description of the desired feature

## Use Case
Explain the scenario where this would be useful

## Proposed Solution
Suggested implementation approach (optional)

## Alternatives
Other approaches considered (optional)

## Additional Context
Examples, mockups, or references
```

## 🎯 Areas Needing Help

Looking for contributions in these areas:

- [ ] Batch processing for multiple files
- [ ] Support for additional subtitle formats (ASS, SSA, SUB, etc.)
- [ ] Command-line argument mode for scripting
- [ ] Preview mode showing affected subtitles
- [ ] Performance improvements for large files
- [ ] Better error messages and user guidance
- [ ] Internationalization (i18n)
- [ ] Configuration file support

## 📢 Getting Help

- **GitHub Issues**: For bugs and feature requests
- **GitHub Discussions**: For questions and general discussion
- **Discord/Slack**: (If available)

## 🎉 Recognition

Contributors will be recognized in:
- CONTRIBUTORS.md file
- Release notes
- Project documentation

Thank you for contributing to ChronoSub! 🙏
