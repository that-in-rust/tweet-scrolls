//! JSON Schema Discovery Tool
//! 
//! Analyzes large JSON files to discover all field variations and types,
//! enabling creation of flexible parsing structures.

use anyhow::{Context, Result};
use serde_json::Value;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
/// Information about a field discovered in the JSON schema
pub struct FieldInfo {
    /// Set of unique types seen for this field
    pub types_seen: HashSet<String>,
    /// Whether this field is optional (not present in all records)
    pub is_optional: bool,
    /// Sample values seen for this field
    pub sample_values: Vec<String>,
    /// Number of times this field has been seen
    pub occurrence_count: usize,
}

/// Schema discovery utility for JSON data
pub struct SchemaDiscovery {
    /// Map of field names to their discovered information
    pub fields: HashMap<String, FieldInfo>,
    /// Total number of items analyzed
    pub total_items_analyzed: usize,
}

impl Default for SchemaDiscovery {
    fn default() -> Self {
        Self::new()
    }
}

impl SchemaDiscovery {
    /// Creates a new schema discovery instance
    pub fn new() -> Self {
        Self {
            fields: HashMap::new(),
            total_items_analyzed: 0,
        }
    }

    /// Analyzes a JSON sample to discover field types and patterns
    /// 
    /// # Arguments
    /// * `json_content` - The raw JSON content to analyze
    /// * `sample_size` - Maximum number of items to analyze
    pub fn analyze_json_sample(&mut self, json_content: &str, sample_size: usize) -> Result<()> {
        println!("🔍 Starting schema discovery analysis...");
        
        // Remove JavaScript prefix if present
        let clean_json = self.clean_javascript_prefix(json_content)?;
        
        // Parse as generic JSON Value
        let data: Vec<Value> = serde_json::from_str(&clean_json)
            .context("Failed to parse JSON for schema discovery")?;
        
        println!("📊 Analyzing {} items (sample size: {})", data.len().min(sample_size), sample_size);
        
        // Analyze sample
        for (i, item) in data.iter().enumerate() {
            if i >= sample_size { break; }
            self.analyze_value_recursive(item, "");
            self.total_items_analyzed += 1;
            
            if i % 100 == 0 && i > 0 {
                println!("  Processed {} items...", i);
            }
        }
        
        // Mark fields as optional if they don't appear in all items
        self.calculate_optional_fields();
        
        println!("✅ Schema discovery complete! Analyzed {} items, found {} unique fields", 
            self.total_items_analyzed, self.fields.len());
        
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
                        occurrence_count: 0,
                    });
                    
                    field_info.types_seen.insert(type_name);
                    field_info.occurrence_count += 1;
                    
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
            Value::Object(obj) => format!("{{{} fields}}", obj.len()),
            Value::Null => "null".to_string(),
        }
    }

    fn calculate_optional_fields(&mut self) {
        let total = self.total_items_analyzed as f64;
        for field_info in self.fields.values_mut() {
            // If field appears in less than 95% of items, mark as optional
            field_info.is_optional = (field_info.occurrence_count as f64 / total) < 0.95;
        }
    }

    /// Generates a human-readable report of the discovered schema
    pub fn generate_report(&self) -> String {
        let mut report = String::new();
        report.push_str("# JSON Schema Discovery Report\n\n");
        report.push_str(&format!("**Total items analyzed:** {}\n\n", self.total_items_analyzed));
        
        let mut sorted_fields: Vec<_> = self.fields.iter().collect();
        sorted_fields.sort_by_key(|(path, _)| path.as_str());
        
        // Summary statistics
        let optional_count = self.fields.values().filter(|f| f.is_optional).count();
        let required_count = self.fields.len() - optional_count;
        let mixed_type_count = self.fields.values().filter(|f| f.types_seen.len() > 1).count();
        
        report.push_str("**Summary:**\n");
        report.push_str(&format!("- Total fields: {}\n", self.fields.len()));
        report.push_str(&format!("- Required fields: {}\n", required_count));
        report.push_str(&format!("- Optional fields: {}\n", optional_count));
        report.push_str(&format!("- Mixed-type fields: {}\n\n", mixed_type_count));
        
        // Detailed field analysis
        report.push_str("## Field Details\n\n");
        for (path, info) in sorted_fields {
            report.push_str(&format!("### `{}`\n", path));
            report.push_str(&format!("- **Types:** {:?}\n", info.types_seen));
            report.push_str(&format!("- **Optional:** {} ({}/{} items)\n", 
                info.is_optional, info.occurrence_count, self.total_items_analyzed));
            report.push_str(&format!("- **Samples:** {:?}\n\n", info.sample_values));
        }
        
        report
    }

    /// Returns fields that have multiple types or other problematic patterns
    pub fn get_problematic_fields(&self) -> Vec<(String, &FieldInfo)> {
        self.fields.iter()
            .filter(|(_, info)| info.types_seen.len() > 1 || info.types_seen.contains("Object"))
            .map(|(path, info)| (path.clone(), info))
            .collect()
    }
}