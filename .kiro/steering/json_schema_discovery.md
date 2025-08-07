# JSON Schema Discovery & Flexible Parsing

## Overview
When dealing with large, complex JSON files with unknown or varying field structures (like Twitter exports), we need robust methods to discover all field variations and create flexible parsing structures.

## Problem Statement
Real-world JSON data often has:
- Fields that can be strings, numbers, or objects
- Optional fields that may or may not exist
- Arrays that might contain mixed types
- Nested structures with varying depths
- Large file sizes (100MB+) requiring streaming

## Solution: Schema Discovery Approach

### 1. Schema Discovery Tool
Analyze a sample of JSON to discover all possible field variations:

```rust
use serde_json::Value;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
pub struct FieldInfo {
    pub types_seen: HashSet<String>,
    pub is_optional: bool,
    pub sample_values: Vec<String>,
}

pub struct SchemaDiscovery {
    pub fields: HashMap<String, FieldInfo>,
    pub total_items_analyzed: usize,
}

impl SchemaDiscovery {
    pub fn new() -> Self {
        Self {
            fields: HashMap::new(),
            total_items_analyzed: 0,
        }
    }

    pub fn analyze_json_sample(&mut self, json_content: &str, sample_size: usize) -> Result<()> {
        // Remove JavaScript prefix if present
        let clean_json = self.clean_javascript_prefix(json_content)?;
        
        // Parse as generic JSON Value
        let data: Vec<Value> = serde_json::from_str(&clean_json)?;
        
        // Analyze sample
        for (i, item) in data.iter().enumerate() {
            if i >= sample_size { break; }
            self.analyze_value_recursive(item, "");
            self.total_items_analyzed += 1;
        }
        
        // Mark fields as optional if they don't appear in all items
        self.calculate_optional_fields();
        
        Ok(())
    }

    fn clean_javascript_prefix(&self, content: &str) -> Result<String> {
        if let Some(start) = content.find('[') {
            if let Some(end) = content.rfind(']') {
                return Ok(content[start..=end].to_string());
            }
        }
        anyhow::bail!("Invalid JSON format: missing brackets");
    }

    fn analyze_value_recursive(&mut self, value: &Value, path: &str) {
        match value {
            Value::Object(map) => {
                for (key, val) in map {
                    let field_path = if path.is_empty() { 
                        key.clone() 
                    } else { 
                        format!("{}.{}", path, key) 
                    };
                    
                    let type_name = self.get_type_name(val);
                    let sample_value = self.get_sample_value(val);
                    
                    let field_info = self.fields.entry(field_path.clone()).or_insert_with(|| FieldInfo {
                        types_seen: HashSet::new(),
                        is_optional: false,
                        sample_values: Vec::new(),
                    });
                    
                    field_info.types_seen.insert(type_name);
                    if field_info.sample_values.len() < 3 {
                        field_info.sample_values.push(sample_value);
                    }
                    
                    // Recurse into nested structures
                    self.analyze_value_recursive(val, &field_path);
                }
            }
            Value::Array(arr) => {
                // Analyze first few array items to understand structure
                for (i, item) in arr.iter().enumerate().take(5) {
                    let array_path = format!("{}[{}]", path, i);
                    self.analyze_value_recursive(item, &array_path);
                }
            }
            _ => {}
        }
    }

    fn get_type_name(&self, value: &Value) -> String {
        match value {
            Value::String(_) => "String".to_string(),
            Value::Number(n) => {
                if n.is_i64() { "Integer" } else { "Float" }
            }.to_string(),
            Value::Bool(_) => "Boolean".to_string(),
            Value::Array(_) => "Array".to_string(),
            Value::Object(_) => "Object".to_string(),
            Value::Null => "Null".to_string(),
        }
    }

    fn get_sample_value(&self, value: &Value) -> String {
        match value {
            Value::String(s) => s.chars().take(50).collect(),
            Value::Number(n) => n.to_string(),
            Value::Bool(b) => b.to_string(),
            Value::Array(arr) => format!("[{} items]", arr.len()),
            Value::Object(obj) => format!("{{{}fields}}", obj.len()),
            Value::Null => "null".to_string(),
        }
    }

    fn calculate_optional_fields(&mut self) {
        let total = self.total_items_analyzed as f64;
        for field_info in self.fields.values_mut() {
            // If field appears in less than 95% of items, mark as optional
            field_info.is_optional = (field_info.sample_values.len() as f64 / total) < 0.95;
        }
    }

    pub fn generate_report(&self) -> String {
        let mut report = String::new();
        report.push_str("# JSON Schema Discovery Report\n\n");
        report.push_str(&format!("**Total items analyzed:** {}\n\n", self.total_items_analyzed));
        
        let mut sorted_fields: Vec<_> = self.fields.iter().collect();
        sorted_fields.sort_by_key(|(path, _)| path.as_str());
        
        for (path, info) in sorted_fields {
            report.push_str(&format!("## Field: `{}`\n", path));
            report.push_str(&format!("- **Types seen:** {:?}\n", info.types_seen));
            report.push_str(&format!("- **Optional:** {}\n", info.is_optional));
            report.push_str(&format!("- **Sample values:** {:?}\n\n", info.sample_values));
        }
        
        report
    }
}
```

### 2. Flexible Serde Structures
Create maximally flexible structures based on discovery results:

```rust
use serde::{Deserialize, Deserializer};
use serde_json::Value;

// Custom deserializer that handles any type and converts to string
fn flexible_string<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Value::deserialize(deserializer)?;
    match value {
        Value::String(s) => Ok(Some(s)),
        Value::Number(n) => Ok(Some(n.to_string())),
        Value::Bool(b) => Ok(Some(b.to_string())),
        Value::Null => Ok(None),
        Value::Object(_) | Value::Array(_) => Ok(Some(value.to_string())),
    }
}

// Custom deserializer for arrays that might contain mixed types
fn flexible_array<'de, D>(deserializer: D) -> Result<Vec<Value>, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Value::deserialize(deserializer)?;
    match value {
        Value::Array(arr) => Ok(arr),
        Value::Null => Ok(Vec::new()),
        other => Ok(vec![other]), // Wrap single values in array
    }
}

// Template for flexible structure
#[derive(Debug, Deserialize)]
pub struct FlexibleStruct {
    // Required fields (appear in >95% of items)
    pub id: String,
    
    // Optional fields with flexible types
    #[serde(deserialize_with = "flexible_string")]
    pub optional_field: Option<String>,
    
    // Arrays that might be objects or strings
    #[serde(default, deserialize_with = "flexible_array")]
    pub flexible_array: Vec<Value>,
    
    // Catch any unknown fields
    #[serde(flatten)]
    pub extra_fields: HashMap<String, Value>,
}
```

### 3. Streaming Parser for Large Files
Handle large files efficiently:

```rust
use serde_json::Deserializer;
use std::fs::File;
use std::io::{BufReader, Read};

pub fn parse_large_json_streaming<T>(
    file_path: &str,
    mut processor: impl FnMut(T) -> Result<()>
) -> Result<usize>
where
    T: for<'de> Deserialize<'de>,
{
    let file = File::open(file_path)?;
    let mut reader = BufReader::new(file);
    
    // Read and clean JavaScript prefix
    let mut content = String::new();
    reader.read_to_string(&mut content)?;
    
    let json_start = content.find('[').context("Missing opening bracket")?;
    let json_end = content.rfind(']').context("Missing closing bracket")?;
    let json_content = &content[json_start..=json_end];
    
    // Stream parse
    let stream = Deserializer::from_str(json_content).into_iter::<T>();
    
    let mut success_count = 0;
    let mut error_count = 0;
    
    for (i, item) in stream.enumerate() {
        match item {
            Ok(parsed_item) => {
                if let Err(e) = processor(parsed_item) {
                    eprintln!("Processing error at item {}: {}", i, e);
                    error_count += 1;
                } else {
                    success_count += 1;
                }
            }
            Err(e) => {
                eprintln!("Parse error at item {}: {}", i, e);
                error_count += 1;
            }
        }
        
        // Progress reporting for large files
        if i % 10000 == 0 && i > 0 {
            println!("Processed {} items ({} success, {} errors)", i, success_count, error_count);
        }
    }
    
    println!("Final: {} successful, {} errors", success_count, error_count);
    Ok(success_count)
}
```

## Usage Workflow

### Step 1: Discover Schema
```rust
let mut discovery = SchemaDiscovery::new();
discovery.analyze_json_sample(&file_content, 1000)?;
let report = discovery.generate_report();
println!("{}", report);
```

### Step 2: Create Flexible Structures
Based on the discovery report, create Rust structs with appropriate flexibility.

### Step 3: Implement Streaming Parser
Use the flexible structures with streaming parser for production use.

## Best Practices

1. **Sample Size**: Use 1000-5000 items for discovery (balance between accuracy and speed)
2. **Error Handling**: Always continue processing on individual item failures
3. **Memory Management**: Use streaming for files >50MB
4. **Field Flexibility**: Make fields optional if they appear in <95% of items
5. **Type Flexibility**: Use custom deserializers for fields with multiple types
6. **Progress Reporting**: Show progress for long-running operations
7. **Documentation**: Generate and save schema reports for future reference

## Performance Considerations

- **Memory Usage**: Streaming keeps memory usage constant regardless of file size
- **CPU Usage**: Schema discovery is CPU-intensive, run once and cache results
- **I/O Optimization**: Use buffered readers for large files
- **Error Recovery**: Graceful degradation when individual items fail to parse

## Common Patterns

### Twitter Export Files
- JavaScript prefixes: `window.YTD.tweets.part0 = [`
- Mixed field types: IDs as strings or numbers
- Optional fields: edit_info, reply fields
- Nested arrays: entities, user_mentions

### API Response Files
- Wrapper objects: `{ "data": [...] }`
- Pagination metadata: `{ "items": [...], "next_page": "..." }`
- Timestamp variations: ISO8601, Unix timestamps, custom formats

### Log Files
- Mixed log levels and formats
- Optional fields based on log type
- Nested error objects
- Variable message structures

## Tools and Utilities

Create helper tools for common tasks:
- Schema discovery CLI tool
- Flexible struct generator
- JSON validator with detailed error reporting
- Performance profiler for large file processing

This approach ensures robust parsing of any JSON structure while maintaining performance and providing detailed insights into data variations.