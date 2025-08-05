---
inclusion: always
---

# Rust Idiomatic Patterns for Test Coverage Implementation

This document provides guidance on applying idiomatic Rust patterns specifically for implementing comprehensive test coverage in the Interview Irodov toolkit. These patterns ensure our tests are not only thorough but also maintainable, performant, and follow Rust best practices.

## Core Testing Philosophy

Following Rust's philosophy of safety, performance, and concurrency, our testing approach should:

1. **Safety First**: Tests should catch bugs at compile time when possible, use type-safe assertions, and avoid unsafe code
2. **Zero-Cost Abstractions**: Test utilities should compile to efficient code without runtime overhead
3. **Explicit Error Handling**: Test failures should provide clear, actionable error messages using `Result` types
4. **Ownership Clarity**: Test data should have clear ownership semantics to avoid lifetime issues

## Essential Patterns for Test Implementation

### 1. Error Handling in Tests (Pattern 2.1-2.10)

**Use `anyhow` for test functions:**
```rust
use anyhow::Result;

#[test]
fn test_archive_creation() -> Result<()> {
    let config = ArchiveConfig::default();
    let archiver = CodeArchiver::new(config)?;
    let archive = archiver.create_archive()?;
    
    assert!(!archive.is_empty(), "Archive should contain files");
    Ok(())
}
```

**Add context to test failures:**
```rust
#[test]
fn test_file_filtering() -> Result<()> {
    let result = process_files(&config)
        .context("Failed to process files with filtering config")?;
    
    assert_eq!(result.len(), 5, "Expected 5 files after filtering");
    Ok(())
}
```

### 2. Resource Management (Pattern 4.1-4.10, 11.4)

**Use RAII for test fixtures:**
```rust
pub struct TestEnvironment {
    pub temp_dir: TempDir,
    _cleanup: CleanupGuard,
}

impl TestEnvironment {
    pub fn new() -> Result<Self> {
        let temp_dir = TempDir::new()?;
        let cleanup = CleanupGuard::new(&temp_dir);
        
        Ok(Self {
            temp_dir,
            _cleanup: cleanup,
        })
    }
}

impl Drop for TestEnvironment {
    fn drop(&mut self) {
        // Automatic cleanup when test completes
    }
}
```

### 3. Builder Pattern for Test Configuration (Pattern 3.1-3.10)

**Create fluent test builders:**
```rust
pub struct TestConfigBuilder {
    extensions: Option<Vec<String>>,
    max_size: Option<u64>,
    git_enabled: bool,
}

impl TestConfigBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn with_extensions(mut self, extensions: Vec<String>) -> Self {
        self.extensions = Some(extensions);
        self
    }
    
    pub fn with_max_size(mut self, size: u64) -> Self {
        self.max_size = Some(size);
        self
    }
    
    pub fn with_git(mut self) -> Self {
        self.git_enabled = true;
        self
    }
    
    pub fn build(self) -> ArchiveConfig {
        ArchiveConfig {
            extensions: self.extensions,
            max_size: self.max_size,
            include_git_status: self.git_enabled,
            ..Default::default()
        }
    }
}
```

### 4. Data Handling: Owned vs Borrowed (Pattern 3.1-3.3)

**Test functions should accept slices:**
```rust
fn assert_files_match(expected: &[&str], actual: &[FileEntry]) {
    let actual_paths: Vec<&str> = actual.iter()
        .map(|entry| entry.path.as_str())
        .collect();
    
    assert_eq!(expected, actual_paths.as_slice());
}
```

**Test fixtures should own their data:**
```rust
pub struct TestProject {
    pub name: String,           // Owned
    pub files: Vec<TestFile>,   // Owned
    pub directories: Vec<PathBuf>, // Owned
}
```

### 5. Option and Null Safety (Pattern 31.1-31.10)

**Use combinators over explicit matching:**
```rust
#[test]
fn test_optional_git_status() -> Result<()> {
    let entry = create_test_entry()?;
    
    // Good: Use combinators
    let status_string = entry.git_status
        .map(|s| s.to_string())
        .unwrap_or_else(|| "No status".to_string());
    
    // Avoid: Explicit matching for simple cases
    // let status_string = match entry.git_status {
    //     Some(s) => s.to_string(),
    //     None => "No status".to_string(),
    // };
    
    assert!(!status_string.is_empty());
    Ok(())
}
```

### 6. Smart Pointers for Complex Test Scenarios (Pattern 4.1-4.4)

**Use `Arc` for shared test data in concurrent tests:**
```rust
#[test]
fn test_concurrent_processing() -> Result<()> {
    let shared_config = Arc::new(create_test_config());
    let handles: Vec<_> = (0..4)
        .map(|i| {
            let config = Arc::clone(&shared_config);
            thread::spawn(move || {
                process_with_config(&config, i)
            })
        })
        .collect();
    
    for handle in handles {
        handle.join().unwrap()?;
    }
    
    Ok(())
}
```

### 7. Testing Patterns (Pattern 11.1-11.10)

**Organize tests by functionality:**
```rust
#[cfg(test)]
mod directory_scanning_tests {
    use super::*;
    
    #[test]
    fn test_recursive_traversal() -> Result<()> {
        // Test implementation
        Ok(())
    }
    
    #[test]
    fn test_symlink_handling() -> Result<()> {
        // Test implementation
        Ok(())
    }
}

#[cfg(test)]
mod file_filtering_tests {
    use super::*;
    
    #[test]
    fn test_extension_filtering() -> Result<()> {
        // Test implementation
        Ok(())
    }
}
```

**Use property-based testing for complex scenarios:**
```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_size_filtering_property(
        file_size in 0u64..1_000_000,
        max_size in 0u64..1_000_000
    ) {
        let should_include = file_size <= max_size;
        let result = size_filter_includes(file_size, max_size);
        prop_assert_eq!(should_include, result);
    }
}
```

### 8. Custom Assertions (Pattern 11.9)

**Create domain-specific assertions:**
```rust
pub trait ArchiveAssertions {
    fn assert_contains_file(&self, path: &str);
    fn assert_file_count(&self, expected: usize);
    fn assert_git_status(&self, path: &str, expected: GitStatus);
}

impl ArchiveAssertions for Vec<FileEntry> {
    fn assert_contains_file(&self, path: &str) {
        assert!(
            self.iter().any(|entry| entry.path.ends_with(path)),
            "Archive should contain file: {}",
            path
        );
    }
    
    fn assert_file_count(&self, expected: usize) {
        assert_eq!(
            self.len(),
            expected,
            "Expected {} files, found {}",
            expected,
            self.len()
        );
    }
    
    fn assert_git_status(&self, path: &str, expected: GitStatus) {
        let entry = self.iter()
            .find(|e| e.path.ends_with(path))
            .expect(&format!("File not found: {}", path));
            
        assert_eq!(
            entry.git_status,
            Some(expected),
            "Git status mismatch for {}: expected {:?}, got {:?}",
            path,
            expected,
            entry.git_status
        );
    }
}
```

### 9. Async Testing Patterns (Pattern 40.1-40.10)

**For future async functionality:**
```rust
#[tokio::test]
async fn test_async_file_processing() -> Result<()> {
    let config = TestConfigBuilder::new()
        .with_extensions(vec!["rs".to_string()])
        .build();
    
    let result = process_files_async(&config).await?;
    
    result.assert_file_count(3);
    Ok(())
}
```

### 10. Performance Testing (Pattern 13.1-13.10)

**Use criterion for benchmarks:**
```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn benchmark_directory_scanning(c: &mut Criterion) {
    let test_env = TestEnvironment::new().unwrap();
    create_large_directory_structure(&test_env.temp_dir).unwrap();
    
    c.bench_function("scan_large_directory", |b| {
        b.iter(|| {
            let config = ArchiveConfig::default();
            let archiver = CodeArchiver::new(config).unwrap();
            black_box(archiver.create_archive().unwrap())
        })
    });
}

criterion_group!(benches, benchmark_directory_scanning);
criterion_main!(benches);
```

## Anti-Patterns to Avoid

### 1. Don't Use `unwrap()` in Tests
```rust
// Bad
let result = risky_operation().unwrap();

// Good
let result = risky_operation()
    .context("Failed to perform risky operation")?;
```

### 2. Don't Ignore Resource Cleanup
```rust
// Bad - resources might leak
#[test]
fn test_something() {
    let temp_dir = TempDir::new().unwrap();
    // Test code...
    // No explicit cleanup
}

// Good - RAII ensures cleanup
#[test]
fn test_something() -> Result<()> {
    let _env = TestEnvironment::new()?;
    // Test code...
    Ok(()) // Cleanup happens automatically
}
```

### 3. Don't Use String Literals for Paths
```rust
// Bad - platform-specific
assert!(path.ends_with("/src/main.rs"));

// Good - platform-agnostic
assert!(path.ends_with(&format!("{}src{}main.rs", 
    std::path::MAIN_SEPARATOR, 
    std::path::MAIN_SEPARATOR)));
```

## Workspace-Specific Guidelines

### Clean Build Pattern (Pattern 0A.11)
Always run `cargo clean` before:
- Running comprehensive test suites
- Switching between test configurations
- After updating workspace dependencies
- When encountering mysterious test failures

### Dependency Management (Pattern 0A.1-0A.4)
- Use workspace-level dependency declarations for test utilities
- Share test dependencies across all crates
- Use feature flags for optional test dependencies (e.g., `criterion` for benchmarks)

### Module Organization (Pattern 16.1-16.10)
```rust
// tests/common/mod.rs - Shared test utilities
pub mod assertions;
pub mod fixtures;
pub mod environment;

// tests/unit/ - Unit tests organized by module
// tests/integration/ - Integration tests
// tests/benchmarks/ - Performance tests
```

## Implementation Priority

When implementing tests, follow this priority order based on Rust idioms:

1. **Safety First**: Implement error handling and resource management
2. **Core Functionality**: Test the ownership and borrowing patterns
3. **Edge Cases**: Use property-based testing for complex scenarios
4. **Performance**: Add benchmarks for critical paths
5. **Integration**: Test component interactions
6. **Documentation**: Ensure tests serve as usage examples

This approach ensures our test suite not only provides comprehensive coverage but also demonstrates idiomatic Rust patterns for future maintainers.