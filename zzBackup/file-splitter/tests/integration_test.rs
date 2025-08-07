use assert_cmd::prelude::*;
use assert_fs::prelude::*;
use std::fs;
use std::process::Command;

#[test]
fn test_cli_split_file() -> Result<(), Box<dyn std::error::Error>> {
    // Create a temporary directory for testing
    let temp_dir = assert_fs::TempDir::new()?;
    
    // Create a test file with known content
    let input_file = temp_dir.child("test.txt");
    let content = "This is a test file with some content that we'll split into chunks";
    input_file.write_str(content)?;
    
    // Create output directory
    let output_dir = temp_dir.child("output");
    
    // Run the CLI command
    let mut cmd = Command::cargo_bin("file-splitter")?;
    
    cmd
        .arg("--input")
        .arg(input_file.path())
        .arg("--output-dir")
        .arg(output_dir.path())
        .arg("--chunk-size")
        .arg("10")
        .arg("--verbose")
        .assert()
        .success();
    
    // Verify output files were created and collect them
    let mut output_files: Vec<_> = fs::read_dir(output_dir.path())?
        .filter_map(Result::ok)
        .filter(|e| e.file_name().to_string_lossy().starts_with("test"))
        .collect();
    
    assert!(!output_files.is_empty(), "No output files were created");
    
    // Sort files by name to ensure correct order
    output_files.sort_by_key(|e| e.path());
    
    // Verify the content of the chunks
    let mut combined = String::new();
    for entry in output_files {
        let chunk_content = fs::read_to_string(entry.path())?;
        combined.push_str(&chunk_content);
    }
    
    assert_eq!(combined, content);
    
    Ok(())
}
