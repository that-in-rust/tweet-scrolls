# Test Specification: LLM-Ready File Generation

## Test Strategy
This document follows Test-Driven Development (TDD) principles to ensure robust implementation of LLM-ready file generation.

## Unit Tests

### 1. Test File Naming and Structure
```rust
#[test]
fn test_llm_file_naming() {
    let user_id = "12345";
    let hashed_id = hash_user_id(user_id);
    let expected = format!("user_{}.md", hashed_id);
    assert_eq!(generate_profile_filename(user_id), expected);
}
```

### 2. Test Markdown Generation
```rust
#[test]
fn test_markdown_generation() {
    let profile = create_sample_profile();
    let markdown = generate_profile_markdown(&profile);
    assert!(markdown.contains("# User Profile"));
    assert!(markdown.contains(&profile.user_id_hash));
    assert!(markdown.contains(&format!("Total Interactions: {}", profile.total_interactions)));
}
```

### 3. Test JSONL Serialization
```rust
#[test]
fn test_jsonl_serialization() {
    let events = create_sample_interactions();
    let jsonl = generate_jsonl(&events);
    let lines: Vec<&str> = jsonl.lines().collect();
    assert_eq!(lines.len(), events.len());
    
    // Verify first line is valid JSON
    let parsed: serde_json::Value = serde_json::from_str(lines[0]).unwrap();
    assert!(parsed.get("timestamp").is_some());
    assert!(parsed.get("type").is_some());
}
```

## Integration Tests

### 1. Test End-to-End File Generation
```rust
#[tokio::test]
async fn test_end_to_end_file_generation() {
    let temp_dir = tempfile::tempdir().unwrap();
    let output_path = temp_dir.path().to_path_buf();
    
    let generator = LLMFileGenerator::new(output_path.clone());
    let result = generator.generate_files(&test_data()).await;
    
    assert!(result.is_ok());
    
    // Verify files were created
    let profile_file = output_path.join("profiles/user_abc123.md");
    assert!(profile_file.exists());
    
    let timeline_file = output_path.join("timelines/activity.jsonl");
    assert!(timeline_file.exists());
}
```

### 2. Test Privacy Preservation
```rust
#[test]
fn test_privacy_preservation() {
    let sensitive_data = "user@example.com";
    let processed = process_for_privacy(sensitive_data);
    
    assert_ne!(sensitive_data, processed);
    assert!(!processed.contains('@'));
}
```

## Performance Tests

### 1. Test Large File Processing
```rust
#[tokio::test]
async fn test_large_file_performance() {
    let large_dataset = generate_large_test_dataset(1_000_000); // 1M records
    let start = Instant::now();
    
    let result = process_batch(&large_dataset).await;
    
    let duration = start.elapsed();
    assert!(duration < Duration::from_secs(30), "Processing took too long: {:?}", duration);
    assert_eq!(result.processed_count, large_dataset.len());
}
```

## Test Data Generation

### Sample Test Data Helper
```rust
fn create_sample_profile() -> UserProfile {
    UserProfile {
        user_id_hash: hash_user_id("12345"),
        first_interaction: Utc::now() - Duration::days(30),
        last_interaction: Utc::now(),
        total_interactions: 42,
        // ... other fields
    }
}

fn create_sample_interactions() -> Vec<InteractionEvent> {
    vec![
        InteractionEvent::new("msg1", Utc::now(), "dm_sent", "user1", "Hello"),
        InteractionEvent::new("msg2", Utc::now(), "dm_received", "user2", "Hi there"),
    ]
}
```

## Test Execution
Run tests with:
```bash
cargo test llm_file_generation -- --nocapture
```
