//! File splitter CLI binary for Tweet-Scrolls
//! 
//! A command-line utility to split large files into manageable chunks.
//! Particularly useful for splitting large Twitter archive files.

use anyhow::{Context, Result, bail};
use std::env;
use std::path::PathBuf;
use tweet_scrolls::utils::file_splitter::{split_file, parse_size_string, SplitConfig};

/// Simple argument parsing structure
#[derive(Debug)]
struct Args {
    input: PathBuf,
    output_dir: Option<PathBuf>,
    chunk_size: String,
    prefix: Option<String>,
    digits: u8,
    verbose: bool,
}

impl Args {
    fn parse() -> Result<Self> {
        let args: Vec<String> = env::args().collect();
        
        if args.len() < 2 {
            print_usage();
            bail!("Missing required argument: input file");
        }
        
        let mut input = None;
        let mut output_dir = None;
        let mut chunk_size = "1M".to_string();
        let mut prefix = None;
        let mut digits = 3;
        let mut verbose = false;
        
        let mut i = 1;
        while i < args.len() {
            match args[i].as_str() {
                "-i" | "--input" => {
                    i += 1;
                    if i >= args.len() {
                        bail!("Missing value for input argument");
                    }
                    input = Some(PathBuf::from(&args[i]));
                }
                "-o" | "--output-dir" => {
                    i += 1;
                    if i >= args.len() {
                        bail!("Missing value for output-dir argument");
                    }
                    output_dir = Some(PathBuf::from(&args[i]));
                }
                "-s" | "--chunk-size" => {
                    i += 1;
                    if i >= args.len() {
                        bail!("Missing value for chunk-size argument");
                    }
                    chunk_size = args[i].clone();
                }
                "-p" | "--prefix" => {
                    i += 1;
                    if i >= args.len() {
                        bail!("Missing value for prefix argument");
                    }
                    prefix = Some(args[i].clone());
                }
                "-d" | "--digits" => {
                    i += 1;
                    if i >= args.len() {
                        bail!("Missing value for digits argument");
                    }
                    digits = args[i].parse()
                        .with_context(|| format!("Invalid digits value: {}", args[i]))?;
                }
                "-v" | "--verbose" => {
                    verbose = true;
                }
                "-h" | "--help" => {
                    print_usage();
                    std::process::exit(0);
                }
                arg if !arg.starts_with('-') => {
                    // Positional argument - treat as input file if not set
                    if input.is_none() {
                        input = Some(PathBuf::from(arg));
                    } else {
                        bail!("Unexpected argument: {}", arg);
                    }
                }
                _ => {
                    bail!("Unknown argument: {}", args[i]);
                }
            }
            i += 1;
        }
        
        let input = input.ok_or_else(|| anyhow::anyhow!("Input file is required"))?;
        
        Ok(Args {
            input,
            output_dir,
            chunk_size,
            prefix,
            digits,
            verbose,
        })
    }
}

fn print_usage() {
    println!("🔧 File Splitter - Tweet-Scrolls Utility");
    println!("========================================");
    println!();
    println!("USAGE:");
    println!("    file-splitter [OPTIONS] <INPUT_FILE>");
    println!("    file-splitter --input <INPUT_FILE> [OPTIONS]");
    println!();
    println!("ARGUMENTS:");
    println!("    <INPUT_FILE>    Input file to split");
    println!();
    println!("OPTIONS:");
    println!("    -i, --input <FILE>        Input file to split");
    println!("    -o, --output-dir <DIR>    Output directory for chunks");
    println!("    -s, --chunk-size <SIZE>   Size of each chunk in MB (e.g., 1M=1MB, 500K, 2G) [default: 1M]");
    println!("    -p, --prefix <PREFIX>     Prefix for chunk filenames");
    println!("    -d, --digits <DIGITS>     Number of digits in chunk numbers [default: 3]");
    println!("    -v, --verbose             Show verbose output");
    println!("    -h, --help                Show this help message");
    println!();
    println!("EXAMPLES:");
    println!("    file-splitter large_file.json");
    println!("    file-splitter -i tweets.js -s 5M -o chunks/");
    println!("    file-splitter --input direct-messages.js --chunk-size 1G --verbose");
}

fn main() -> Result<()> {
    let args = Args::parse()?;
    
    if args.verbose {
        println!("🔧 File Splitter - Tweet-Scrolls Utility");
        println!("========================================");
        println!("📁 Input file: {}", args.input.display());
        if let Some(ref output_dir) = args.output_dir {
            println!("📂 Output directory: {}", output_dir.display());
        }
        println!("📏 Chunk size: {}", args.chunk_size);
        println!();
    }
    
    // Parse chunk size
    let chunk_size = parse_size_string(&args.chunk_size)
        .with_context(|| format!("Invalid chunk size: {}", args.chunk_size))?;
    
    // Validate digits
    if args.digits == 0 || args.digits > 10 {
        anyhow::bail!("Digits must be between 1 and 10, got: {}", args.digits);
    }
    
    // Build configuration
    let config = SplitConfig {
        input_path: args.input,
        output_dir: args.output_dir,
        chunk_size,
        prefix: args.prefix,
        digits: args.digits,
    };
    
    // Perform the split
    println!("🚀 Starting file split operation...");
    let result = split_file(&config)
        .context("Failed to split file")?;
    
    // Display results
    println!("✅ Split operation completed successfully!\n");
    println!("{}", result);
    
    if args.verbose {
        println!("\n💡 Tips:");
        println!("  • Use larger chunk sizes for better performance");
        println!("  • Chunks preserve the original file extension");
        println!("  • Use custom prefixes to organize different splits");
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    // Helper function to simulate command line arguments
    fn parse_args_from_vec(args: Vec<&str>) -> Result<Args> {
        // Note: We can't easily mock env::args() in tests, so we simulate parsing
        
        // We can't easily mock env::args(), so we'll test the parsing logic directly
        // by creating a mock implementation for testing
        let mut input = None;
        let mut output_dir = None;
        let mut chunk_size = "1M".to_string();
        let mut prefix = None;
        let mut digits = 3;
        let mut verbose = false;
        
        let mut i = 1; // Skip program name
        while i < args.len() {
            match args[i] {
                "-i" | "--input" => {
                    i += 1;
                    if i >= args.len() {
                        bail!("Missing value for input argument");
                    }
                    input = Some(PathBuf::from(args[i]));
                }
                "-o" | "--output-dir" => {
                    i += 1;
                    if i >= args.len() {
                        bail!("Missing value for output-dir argument");
                    }
                    output_dir = Some(PathBuf::from(args[i]));
                }
                "-s" | "--chunk-size" => {
                    i += 1;
                    if i >= args.len() {
                        bail!("Missing value for chunk-size argument");
                    }
                    chunk_size = args[i].to_string();
                }
                "-p" | "--prefix" => {
                    i += 1;
                    if i >= args.len() {
                        bail!("Missing value for prefix argument");
                    }
                    prefix = Some(args[i].to_string());
                }
                "-d" | "--digits" => {
                    i += 1;
                    if i >= args.len() {
                        bail!("Missing value for digits argument");
                    }
                    digits = args[i].parse()
                        .with_context(|| format!("Invalid digits value: {}", args[i]))?;
                }
                "-v" | "--verbose" => {
                    verbose = true;
                }
                arg if !arg.starts_with('-') => {
                    if input.is_none() {
                        input = Some(PathBuf::from(arg));
                    } else {
                        bail!("Unexpected argument: {}", arg);
                    }
                }
                _ => {
                    bail!("Unknown argument: {}", args[i]);
                }
            }
            i += 1;
        }
        
        let input = input.ok_or_else(|| anyhow::anyhow!("Input file is required"))?;
        
        Ok(Args {
            input,
            output_dir,
            chunk_size,
            prefix,
            digits,
            verbose,
        })
    }
    
    #[test]
    fn test_args_parsing_minimal() -> Result<()> {
        let args = parse_args_from_vec(vec!["file-splitter", "--input", "test.txt"])?;
        assert_eq!(args.input, PathBuf::from("test.txt"));
        assert_eq!(args.chunk_size, "1M");
        assert_eq!(args.digits, 3);
        assert!(!args.verbose);
        Ok(())
    }
    
    #[test]
    fn test_args_parsing_full() -> Result<()> {
        let args = parse_args_from_vec(vec![
            "file-splitter",
            "--input", "large_file.json",
            "--output-dir", "/tmp/chunks",
            "--chunk-size", "5M",
            "--prefix", "chunk",
            "--digits", "4",
            "--verbose"
        ])?;
        
        assert_eq!(args.input, PathBuf::from("large_file.json"));
        assert_eq!(args.output_dir, Some(PathBuf::from("/tmp/chunks")));
        assert_eq!(args.chunk_size, "5M");
        assert_eq!(args.prefix, Some("chunk".to_string()));
        assert_eq!(args.digits, 4);
        assert!(args.verbose);
        Ok(())
    }
    
    #[test]
    fn test_args_parsing_short_flags() -> Result<()> {
        let args = parse_args_from_vec(vec![
            "file-splitter",
            "-i", "test.txt",
            "-o", "output",
            "-s", "1G",
            "-p", "part",
            "-d", "2",
            "-v"
        ])?;
        
        assert_eq!(args.input, PathBuf::from("test.txt"));
        assert_eq!(args.output_dir, Some(PathBuf::from("output")));
        assert_eq!(args.chunk_size, "1G");
        assert_eq!(args.prefix, Some("part".to_string()));
        assert_eq!(args.digits, 2);
        assert!(args.verbose);
        Ok(())
    }
    
    #[test]
    fn test_args_parsing_positional() -> Result<()> {
        let args = parse_args_from_vec(vec!["file-splitter", "test.txt"])?;
        assert_eq!(args.input, PathBuf::from("test.txt"));
        assert_eq!(args.chunk_size, "1M");
        assert_eq!(args.digits, 3);
        assert!(!args.verbose);
        Ok(())
    }
    
    #[test]
    fn test_args_parsing_missing_input() {
        let result = parse_args_from_vec(vec!["file-splitter"]);
        assert!(result.is_err());
    }
}