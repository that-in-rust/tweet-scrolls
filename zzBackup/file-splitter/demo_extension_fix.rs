#!/usr/bin/env rust-script

//! Demonstration of the file extension preservation fix
//! 
//! This script shows how the new file-splitter naming convention works:
//! - document.txt → document-001.txt, document-002.txt, document-003.txt
//! - archive.tar.gz → archive-001.tar.gz, archive-002.tar.gz
//! - README → README-001, README-002

use std::path::Path;

fn demonstrate_filename_parsing(input_filename: &str) {
    println!("Input: {}", input_filename);
    
    let input_path = Path::new(input_filename);
    let file_name = input_path.file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("chunk");
    
    // Handle complex extensions like .tar.gz by finding the first dot
    let (base_name, extension) = if let Some(dot_pos) = file_name.find('.') {
        let base_name = file_name[..dot_pos].to_string();
        let extension = file_name[dot_pos..].to_string();
        (base_name, extension)
    } else {
        // No extension found
        (file_name.to_string(), String::new())
    };
    
    // Show what the chunk names would be
    for i in 1..=3 {
        let chunk_name = format!("{}-{:03}{}", base_name, i, extension);
        println!("  Chunk {}: {}", i, chunk_name);
    }
    println!();
}

fn main() {
    println!("🔧 File-Splitter Extension Preservation Demo");
    println!("============================================\n");
    
    println!("✅ NEW IMPROVED NAMING CONVENTION:");
    println!("Preserves file extensions and uses dash separator\n");
    
    // Test cases
    demonstrate_filename_parsing("document.txt");
    demonstrate_filename_parsing("archive.tar.gz");
    demonstrate_filename_parsing("data.json");
    demonstrate_filename_parsing("README");
    demonstrate_filename_parsing("script.sh");
    demonstrate_filename_parsing("image.png");
    
    println!("🎯 Benefits:");
    println!("• File type is immediately recognizable");
    println!("• Tools can still identify file types");
    println!("• Clear relationship between chunks and original");
    println!("• Handles complex extensions like .tar.gz correctly");
}