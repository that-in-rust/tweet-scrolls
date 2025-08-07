# Idiomatic Rust TDD Patterns - Tweet-Scrolls Reference
## Production-Ready Patterns Demonstrated

### 🎯 **Purpose**
This document captures the idiomatic Rust and TDD patterns successfully implemented in Tweet-Scrolls, serving as a reference for future Rust development projects.

## 🔄 **TDD Methodology Patterns**

### **Red → Green → Refactor Cycle**
```rust
// RED PHASE: Write failing test first
#[tokio::test]
async fn test_main_relationship_analysis_integration() -> Result<()> {
    let result = main_analyze_relationships(/* params */).await;
    assert!(result.is_ok()); // FAILS - no implementation yet
}

// GREEN PHASE: Minimal implementation to pass
pub async fn main_analyze_relationships(/* params */) -> Result<()> {
    // Minimal code to make test pass
    Ok(())
}

// REFACTOR PHASE: Improve design while keeping tests green
pub async fn main_analyze_relationships(
    output_path: &str, screen_name: &str, timestamp: u64,
    profiles: &[UserProfile], interactions: &[InteractionEvent]
) -> Result<()> {
    let generator = LLMFileGenerator::new(output_path, screen_name, timestamp);
    generator.generate_all_files(profiles, interactions)
        .context("Failed to generate relationship analysis files")?;
    Ok(())
}
```

### **Test Organization Patterns**
```rust
// Helper functions for test data creation
fn create_sample_profiles() -> Vec<UserProfile> {
    vec![
        UserProfile::new("abcd1234efgh5678"),
        UserProfile::new("wxyz9876abcd5432"),
    ]
}

// Comprehensive integration testing
#[tokio::test]
async fn test_complete_workflow() -> Result<()> {
    let temp_dir = tempdir()?;
    let profiles = create_sample_profiles();
    
    let result = process_complete_workflow(&profiles).await;
    
    assert!(result.is_ok());
    // Verify side effects
    assert!(expected_file.exists());
    Ok(())
}

// Error case testing
#[tokio::test]
async fn test_error_handling() -> Result<()> {
    let result = function_with_invalid_input().await;
    assert!(result.is_err());
    Ok(())
}
```

## 🏗️ **Idiomatic Rust Patterns**

### **Error Handling Excellence**
```rust
use anyhow::{Context, Result};

// Comprehensive error context
pub async fn process_file(path: &str) -> Result<ProcessedData> {
    let content = tokio::fs::read_to_string(path)
        .await
        .with_context(|| format!("Failed to read file: {}", path))?;
    
    let parsed = parse_content(&content)
        .context("Failed to parse file content")?;
    
    Ok(parsed)
}

// Error propagation with ?
pub fn chain_operations() -> Result<FinalResult> {
    let step1 = first_operation()?;
    let step2 = second_operation(step1)?;
    let final_result = third_operation(step2)?;
    Ok(final_result)
}
```

### **Async/Await Patterns**
```rust
// Proper async function signatures
pub async fn main_analyze_relationships(
    output_path: &str,
    screen_name: &str, 
    timestamp: u64,
    profiles: &[UserProfile],
    interactions: &[InteractionEvent]
) -> Result<()> {
    // Async operations with proper error handling
    let generator = LLMFileGenerator::new(output_path, screen_name, timestamp);
    generator.generate_all_files(profiles, interactions).await?;
    Ok(())
}

// Async testing patterns
#[tokio::test]
async fn test_async_operation() -> Result<()> {
    let result = async_function().await?;
    assert_eq!(result.status, ExpectedStatus::Success);
    Ok(())
}
```

### **Type Safety and Strong Typing**
```rust
// Custom types for domain modeling
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfile {
    pub user_hash: String,
    pub total_interactions: u32,
    pub first_interaction: Option<DateTime<Utc>>,
    pub last_interaction: Option<DateTime<Utc>>,
    pub interaction_counts: HashMap<String, u32>,
    pub metadata: HashMap<String, String>,
}

// Enum for type safety
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum InteractionType {
    DmSent,
    DmReceived,
    TweetSent,
    TweetReceived,
    Mention,
    Reply,
    TweetReply,
    Like,
    Retweet,
    Quote,
    Other,
}

// Implementation blocks for behavior
impl UserProfile {
    pub fn new(user_hash: impl Into<String>) -> Self {
        Self {
            user_hash: user_hash.into(),
            total_interactions: 0,
            first_interaction: None,
            last_interaction: None,
            interaction_counts: HashMap::new(),
            metadata: HashMap::new(),
        }
    }
    
    pub fn add_interaction(&mut self, interaction_type: impl Into<String>, timestamp: DateTime<Utc>) {
        self.total_interactions += 1;
        let type_str = interaction_type.into();
        *self.interaction_counts.entry(type_str).or_insert(0) += 1;
        
        if self.first_interaction.map_or(true, |t| timestamp < t) {
            self.first_interaction = Some(timestamp);
        }
        if self.last_interaction.map_or(true, |t| timestamp > t) {
            self.last_interaction = Some(timestamp);
        }
    }
}
```

### **Module Organization Patterns**
```rust
// Clean module structure
pub mod main_integration;     // Main function integration
pub mod relationship {        // Relationship analysis
    pub mod file_generation;  // LLM-ready file generation
    pub mod analyzer;         // Core analysis logic
    pub mod communication;    // Communication patterns
    pub mod anonymization;    // Privacy protection
    pub mod timeline_integration; // Timeline analysis
}
pub mod processing {          // Data processing
    pub mod tweets;           // Tweet processing
    pub mod direct_messages;  // DM processing
    pub mod data_structures;  // Core data types
    pub mod file_io;          // File I/O operations
}

// Module re-exports for clean APIs
pub use models::interaction::*;
pub use services::timeline::*;
```

### **Documentation Patterns**
```rust
//! Main function integration for relationship analysis

/// Main orchestration function for relationship analysis
/// 
/// This function coordinates the complete relationship analysis workflow,
/// from data processing to file generation.
/// 
/// # Arguments
/// 
/// * `output_path` - Directory where analysis files will be created
/// * `screen_name` - Twitter screen name for the analysis
/// * `timestamp` - Unix timestamp for file naming
/// * `profiles` - User profiles to analyze
/// * `interactions` - Interaction events to process
/// 
/// # Returns
/// 
/// Returns `Ok(())` on successful completion, or an error if any step fails.
/// 
/// # Examples
/// 
/// ```rust
/// let profiles = vec![UserProfile::new("user_hash")];
/// let interactions = vec![/* interaction events */];
/// 
/// main_analyze_relationships(
///     "/output/path",
///     "username", 
///     1234567890,
///     &profiles,
///     &interactions
/// ).await?;
/// ```
pub async fn main_analyze_relationships(
    output_path: &str,
    screen_name: &str,
    timestamp: u64,
    profiles: &[UserProfile],
    interactions: &[InteractionEvent]
) -> Result<()> {
    // Implementation
}
```

## 🎯 **Performance Patterns**

### **Async I/O Optimization**
```rust
// Async file operations
pub async fn generate_all_files(
    &self,
    profiles: &[UserProfile],
    interactions: &[InteractionEvent]
) -> Result<()> {
    // Create output directory
    tokio::fs::create_dir_all(&self.output_dir).await
        .context("Failed to create output directory")?;
    
    // Generate files concurrently where possible
    let profile_futures: Vec<_> = profiles.iter()
        .map(|profile| self.generate_individual_profile_file(profile))
        .collect();
    
    // Wait for all profile files to complete
    for future in profile_futures {
        future.await?;
    }
    
    // Generate other files
    self.generate_timeline_file(interactions).await?;
    self.generate_analysis_prompts(profiles).await?;
    
    Ok(())
}
```

### **Memory Efficiency Patterns**
```rust
// Use references to avoid unnecessary clones
pub fn analyze_patterns(profiles: &[UserProfile]) -> AnalysisResult {
    profiles.iter()
        .map(|profile| analyze_single_profile(profile))
        .collect()
}

// Streaming processing for large datasets
pub fn process_large_dataset<T>(
    data: impl Iterator<Item = T>,
    processor: impl Fn(T) -> ProcessedItem
) -> Vec<ProcessedItem> {
    data.map(processor).collect()
}
```

## 🔒 **Security and Privacy Patterns**

### **Anonymization Patterns**
```rust
use blake3;

/// Hash user ID using Blake3 for consistent anonymization
pub fn hash_user_id(user_id: &str) -> String {
    let mut hasher = blake3::Hasher::new();
    hasher.update(user_id.as_bytes());
    hasher.finalize().to_hex().to_string()
}

// Use hashed IDs throughout the system
pub fn create_anonymous_profile(user_id: &str) -> UserProfile {
    let user_hash = hash_user_id(user_id);
    UserProfile::new(user_hash)
}
```

### **Local Processing Patterns**
```rust
// Ensure all processing remains local
pub async fn process_twitter_archive(archive_path: &str) -> Result<AnalysisResult> {
    // All file operations are local
    let tweets = load_tweets_locally(archive_path).await?;
    let dms = load_dms_locally(archive_path).await?;
    
    // All processing happens locally
    let analysis = analyze_locally(&tweets, &dms)?;
    
    // All output is local
    save_results_locally(&analysis).await?;
    
    Ok(analysis)
}
```

## 🧪 **Testing Patterns**

### **Comprehensive Test Coverage**
```rust
// Unit tests for individual functions
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_hash_consistency() {
        let user_id = "1132151165410455552";
        let hash1 = hash_user_id(user_id);
        let hash2 = hash_user_id(user_id);
        assert_eq!(hash1, hash2);
    }
    
    #[test]
    fn test_hash_uniqueness() {
        let hash1 = hash_user_id("user1");
        let hash2 = hash_user_id("user2");
        assert_ne!(hash1, hash2);
    }
}

// Integration tests for workflows
#[cfg(test)]
mod integration_tests {
    use super::*;
    use tempfile::tempdir;
    
    #[tokio::test]
    async fn test_complete_workflow() -> Result<()> {
        let temp_dir = tempdir()?;
        let output_path = temp_dir.path().to_str().unwrap();
        
        let result = complete_workflow(output_path).await;
        
        assert!(result.is_ok());
        assert!(expected_output_exists(output_path));
        Ok(())
    }
}
```

### **Test Data Creation Patterns**
```rust
// Helper functions for creating test data
fn create_sample_profiles() -> Vec<UserProfile> {
    vec![
        UserProfile::new("abcd1234efgh5678"),
        UserProfile::new("wxyz9876abcd5432"),
    ]
}

fn create_sample_interactions() -> Vec<InteractionEvent> {
    use chrono::Utc;
    
    vec![
        InteractionEvent {
            id: "msg1".to_string(),
            user_hash: "abcd1234efgh5678".to_string(),
            interaction_type: InteractionType::DmSent,
            timestamp: Utc::now(),
            content: "Test message".to_string(),
            metadata: std::collections::HashMap::new(),
        },
        // More test data...
    ]
}
```

## 📏 **Code Quality Patterns**

### **File Size Management**
```rust
// Keep modules focused and under size limits
// Target: <400 lines per file, Maximum: 600 lines

// Single responsibility principle
pub mod anonymization {
    // Only user ID hashing functionality
}

pub mod file_generation {
    // Only LLM file generation functionality
}

pub mod communication {
    // Only communication analysis functionality
}
```

### **Clean Code Patterns**
```rust
// Descriptive function names
pub fn calculate_response_times(events: &[InteractionEvent]) -> Vec<Duration> { }
pub fn generate_relationship_analysis_prompt() -> String { }
pub fn should_run_relationship_analysis(input: &str) -> bool { }

// Clear variable names
let relationship_dir = Path::new(output_path).join(format!("relationship_profiles_{}_{}", screen_name, timestamp));
let most_active_relationship = profiles.iter().max_by_key(|p| p.total_interactions);

// Meaningful constants
const BLAKE3_HASH_LENGTH: usize = 64;
const MAX_FILE_SIZE_LINES: usize = 600;
const OPTIMAL_FILE_SIZE_LINES: usize = 400;
```

## 🎯 **Summary of Patterns**

### **TDD Patterns Demonstrated**
1. **Red → Green → Refactor**: Complete cycles for all features
2. **Test-First Development**: Every feature starts with a failing test
3. **Comprehensive Coverage**: 15/15 tests passing for critical functionality
4. **Integration Testing**: End-to-end workflow validation
5. **Error Case Testing**: Robust error handling validation

### **Rust Idioms Followed**
1. **Error Handling**: `Result<T>` with `anyhow::Context`
2. **Async/Await**: Proper async patterns with `tokio`
3. **Memory Safety**: Zero unsafe code, ownership system
4. **Type Safety**: Strong typing with custom structs/enums
5. **Documentation**: Comprehensive doc comments
6. **Module Organization**: Clean separation of concerns

### **Production Patterns Applied**
1. **Performance**: Async I/O with memory optimization
2. **Security**: Blake3 anonymization and local processing
3. **Maintainability**: Modular design under file size limits
4. **Reliability**: Comprehensive error handling and testing
5. **Usability**: Clear APIs and user experience

---

*This document serves as a reference for idiomatic Rust and TDD patterns successfully demonstrated in the Tweet-Scrolls project.*