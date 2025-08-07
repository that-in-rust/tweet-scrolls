#!/usr/bin/env cargo
//! Schema Discovery Tool for JSON Files
//! 
//! Analyzes large JSON files to discover field variations and generate
//! flexible parsing structures.

use anyhow::Result;
use std::fs;
use tweet_scrolls::utils::schema_discovery::SchemaDiscovery;

#[tokio::main]
async fn main() -> Result<()> {
    println!("🔍 JSON Schema Discovery Tool");
    println!("=============================");
    
    // Get file path from command line or prompt
    let args: Vec<String> = std::env::args().collect();
    let file_path = if args.len() > 1 {
        args[1].clone()
    } else {
        println!("📁 Enter path to JSON file to analyze:");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        input.trim().to_string()
    };
    
    println!("📂 Analyzing file: {}", file_path);
    
    // Read the file
    let content = fs::read_to_string(&file_path)?;
    println!("📊 File size: {} bytes", content.len());
    
    // Run schema discovery
    let mut discovery = SchemaDiscovery::new();
    discovery.analyze_json_sample(&content, 1000)?;
    
    // Show summary only
    println!("\n📊 DISCOVERY SUMMARY:");
    println!("Total fields found: {}", discovery.fields.len());
    println!("Items analyzed: {}", discovery.total_items_analyzed);
    
    // Show problematic fields that need special handling
    let problematic = discovery.get_problematic_fields();
    if !problematic.is_empty() {
        println!("🚨 FIELDS REQUIRING SPECIAL HANDLING:");
        println!("=====================================");
        for (path, info) in problematic {
            println!("• `{}` - Types: {:?}", path, info.types_seen);
            if info.types_seen.contains("Object") {
                println!("  ⚠️  Contains objects - needs flexible_string deserializer");
            }
            if info.types_seen.len() > 1 {
                println!("  ⚠️  Mixed types - needs flexible deserializer");
            }
        }
    }
    
    // Save report to file
    let report = discovery.generate_report();
    let report_path = format!("{}_schema_report.md", file_path.replace('/', "_"));
    fs::write(&report_path, &report)?;
    println!("\n📄 Report saved to: {}", report_path);
    
    Ok(())
}