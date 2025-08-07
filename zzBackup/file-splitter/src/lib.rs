//! A library for splitting files into smaller chunks with various strategies.

use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::{Read, Write};
use thiserror::Error;
use std::fmt;

/// Custom error type for file splitting operations
#[derive(Error, Debug)]
pub enum SplitError {
    /// I/O error occurred
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    
    /// Invalid chunk size specified
    #[error("Invalid chunk size: {0}")]
    InvalidChunkSize(String),
    
    /// Invalid input path
    #[error("Invalid input path: {0}")]
    InvalidInputPath(String),
    
    /// Invalid output directory
    #[error("Invalid output directory: {0}")]
    InvalidOutputDir(String),
}

/// Result type for file splitting operations
pub type Result<T> = std::result::Result<T, SplitError>;

/// Configuration for file splitting
#[derive(Debug, Clone)]
pub struct SplitConfig {
    /// Path to the input file
    pub input_path: String,
    
    /// Directory to output chunks (defaults to same as input file)
    pub output_dir: Option<String>,
    
    /// Size of each chunk in bytes
    pub chunk_size: u64,
    
    /// Prefix for output chunk filenames (defaults to input filename)
    pub prefix: Option<String>,
    
    /// Number of digits to use in chunk numbering (default: 3)
    pub digits: u8,
}

impl Default for SplitConfig {
    fn default() -> Self {
        Self {
            input_path: String::new(),
            output_dir: None,
            chunk_size: 1024 * 1024, // 1MB default chunk size
            prefix: None,
            digits: 3,
        }
    }
}

/// Represents a chunk of a file
#[derive(Debug)]
pub struct FileChunk {
    /// The path to the chunk file
    pub path: PathBuf,
    /// The size of the chunk in bytes
    pub size: u64,
}

impl fmt::Display for FileChunk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f, 
            "{} ({} bytes)", 
            self.path.display(),
            self.size
        )
    }
}

/// Result of a file split operation
#[derive(Debug)]
pub struct SplitResult {
    /// The original file path
    pub input_path: PathBuf,
    /// The output directory
    pub output_dir: PathBuf,
    /// The size of each chunk
    pub chunk_size: u64,
    /// Information about each created chunk
    pub chunks: Vec<FileChunk>,
    /// Total number of chunks created
    pub total_chunks: usize,
    /// Total size of the original file in bytes
    pub total_size: u64,
}

impl fmt::Display for SplitResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Split '{}' into {} chunks:", 
            self.input_path.display(), 
            self.total_chunks
        )?;
        
        for (i, chunk) in self.chunks.iter().enumerate() {
            writeln!(f, "  {:03}: {}", i + 1, chunk)?;
        }
        
        writeln!(f, "Total size: {} bytes ({} chunks)", 
            self.total_size, 
            self.total_chunks
        )
    }
}

/// Split a file into smaller chunks based on the provided configuration
pub fn split_file(config: &SplitConfig) -> Result<SplitResult> {
    // Input validation
    if config.chunk_size == 0 {
        return Err(SplitError::InvalidChunkSize("Chunk size must be greater than 0".into()));
    }
    
    // Verify input file exists and is a file
    let input_path = Path::new(&config.input_path).canonicalize()
        .map_err(|e| SplitError::InvalidInputPath(e.to_string()))?;
        
    if !input_path.is_file() {
        return Err(SplitError::InvalidInputPath("Input path is not a file".into()));
    }
    
    // Get or create output directory
    let output_dir = match &config.output_dir {
        Some(dir) => {
            let path = Path::new(dir);
            if !path.exists() {
                std::fs::create_dir_all(path).map_err(SplitError::Io)?;
            }
            path.canonicalize().map_err(|e| 
                SplitError::InvalidOutputDir(e.to_string())
            )?
        }
        None => {
            input_path.parent()
                .unwrap_or_else(|| Path::new("."))
                .canonicalize()
                .map_err(|e| SplitError::InvalidOutputDir(e.to_string()))?
        }
    };
    
    // Get file metadata
    let metadata = std::fs::metadata(&input_path)
        .map_err(|e| SplitError::Io(e))?;
    
    let file_size = metadata.len();
    if file_size == 0 {
        return Err(SplitError::InvalidInputPath("Input file is empty".into()));
    }
    
    // Calculate number of chunks needed
    let total_chunks = ((file_size as f64) / (config.chunk_size as f64)).ceil() as usize;
    
    // Determine the filename base and extension following idiomatic patterns (Pattern 3.1-3.3)
    let (base_name, extension) = match &config.prefix {
        Some(p) => (p.clone(), String::new()), // Custom prefix, no extension
        None => {
            let file_name = input_path.file_name()
                .and_then(|name| name.to_str())
                .unwrap_or("chunk");
            
            // Handle complex extensions like .tar.gz by finding the first dot
            // This preserves the full extension while getting the true base name
            if let Some(dot_pos) = file_name.find('.') {
                let base_name = file_name[..dot_pos].to_string();
                let extension = file_name[dot_pos..].to_string();
                (base_name, extension)
            } else {
                // No extension found
                (file_name.to_string(), String::new())
            }
        }
    };
    
    // Open the input file
    let mut input_file = File::open(&input_path).map_err(SplitError::Io)?;
    
    // Buffer for reading chunks
    let mut buffer = vec![0u8; config.chunk_size as usize];
    let mut chunks = Vec::with_capacity(total_chunks);
    
    // Process each chunk
    for chunk_num in 0..total_chunks {
        let chunk_path = output_dir.join(format!(
            "{}-{:0width$}{}",
            base_name,
            chunk_num + 1,
            extension,
            width = config.digits as usize
        ));
        
        // Read a chunk from the input file
        let bytes_read = input_file.read(&mut buffer).map_err(SplitError::Io)?;
        
        if bytes_read == 0 {
            break; // End of file
        }
        
        // Write the chunk to the output file
        let mut output_file = File::create(&chunk_path).map_err(SplitError::Io)?;
        output_file.write_all(&buffer[..bytes_read]).map_err(SplitError::Io)?;
        
        // Add chunk info to the result
        chunks.push(FileChunk {
            path: chunk_path,
            size: bytes_read as u64,
        });
    }
    
    // Build and return the result
    Ok(SplitResult {
        input_path,
        output_dir,
        chunk_size: config.chunk_size,
        chunks,
        total_chunks,
        total_size: file_size,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use std::fs::{self, File};
    use std::io::Write;
    use std::path::Path;

    // Helper function to create a test file with specified content
    fn create_test_file(dir: &Path, name: &str, content: &[u8]) -> std::io::Result<std::path::PathBuf> {
        let path = dir.join(name);
        let mut file = File::create(&path)?;
        file.write_all(content)?;
        Ok(path)
    }

    // Helper function to count files in directory with given prefix
    fn count_files_with_prefix(dir: &Path, prefix: &str) -> std::io::Result<usize> {
        let count = fs::read_dir(dir)?
            .filter_map(|res| res.ok())
            .filter(|entry| {
                entry.file_name()
                    .to_str()
                    .map(|s| s.starts_with(prefix))
                    .unwrap_or(false)
            })
            .count();
        Ok(count)
    }

    #[test]
    fn test_split_config_default() {
        let config = SplitConfig::default();
        assert_eq!(config.input_path, "");
        assert_eq!(config.chunk_size, 1024 * 1024); // 1MB default
        assert_eq!(config.digits, 3);
        assert!(config.output_dir.is_none());
        assert!(config.prefix.is_none());
    }

    // Test splitting a file into chunks of specified size
    #[test]
    fn test_split_file_into_chunks() -> Result<()> {
        let temp_dir = tempdir()?;
        let input_path = create_test_file(temp_dir.path(), "test.txt", b"1234567890")?;
        
        let config = SplitConfig {
            input_path: input_path.to_str().unwrap().to_string(),
            chunk_size: 3, // Split into chunks of 3 bytes
            ..Default::default()
        };

        let result = split_file(&config)?;
        
        assert_eq!(result.total_chunks, 4); // 10 bytes / 3 bytes per chunk = 4 chunks
        assert_eq!(result.chunks[0].size, 3);
        assert_eq!(result.chunks[1].size, 3);
        assert_eq!(result.chunks[2].size, 3);
        assert_eq!(result.chunks[3].size, 1); // Last chunk has remaining 1 byte
        
        Ok(())
    }

    // Test default chunk size (1MB)
    #[test]
    fn test_default_chunk_size() -> Result<()> {
        let temp_dir = tempdir()?;
        // Create a file slightly larger than 1MB
        let content = vec![b'x'; 1024 * 1024 + 100];
        let input_path = create_test_file(temp_dir.path(), "large.txt", &content)?;
        
        let config = SplitConfig {
            input_path: input_path.to_str().unwrap().to_string(),
            ..Default::default()
        };

        let result = split_file(&config)?;
        assert_eq!(result.total_chunks, 2); // Should be split into 2 chunks
        assert_eq!(result.chunks[0].size, 1024 * 1024); // First chunk is exactly 1MB
        assert_eq!(result.chunks[1].size, 100); // Second chunk has remaining 100 bytes
        
        Ok(())
    }

    // Test custom chunk sizes with different units
    #[test]
    fn test_custom_chunk_sizes() -> Result<()> {
        let temp_dir = tempdir()?;
        let input_path = create_test_file(temp_dir.path(), "test_units.txt", &[b'x'; 3000])?;
        
        // Test with 1K chunks
        let config = SplitConfig {
            input_path: input_path.to_str().unwrap().to_string(),
            chunk_size: 1024, // 1K
            ..Default::default()
        };
        let result = split_file(&config)?;
        assert_eq!(result.total_chunks, 3); // 3000 / 1024 = 3 chunks
        
        Ok(())
    }

    // Test handling of empty input files
    #[test]
    fn test_empty_input_file() -> Result<()> {
        let temp_dir = tempdir()?;
        let input_path = create_test_file(temp_dir.path(), "empty.txt", b"")?;
        
        let config = SplitConfig {
            input_path: input_path.to_str().unwrap().to_string(),
            chunk_size: 1024,
            ..Default::default()
        };
        
        let result = split_file(&config);
        assert!(matches!(result, Err(SplitError::InvalidInputPath(_))));
        
        Ok(())
    }

    // Test default output directory (same as input file)
    #[test]
    fn test_default_output_directory() -> Result<()> {
        let temp_dir = tempdir()?;
        let input_path = create_test_file(temp_dir.path(), "test_default_dir.txt", b"test")?;
        
        let config = SplitConfig {
            input_path: input_path.to_str().unwrap().to_string(),
            chunk_size: 2,
            output_dir: None, // Should default to input file's directory
            ..Default::default()
        };
        
        let result = split_file(&config)?;
        assert_eq!(result.output_dir, input_path.parent().unwrap().canonicalize()?);
        
        Ok(())
    }

    // Test custom output directory
    #[test]
    fn test_custom_output_directory() -> Result<()> {
        let temp_dir = tempdir()?;
        let input_path = create_test_file(temp_dir.path(), "test_custom_dir.txt", b"test")?;
        let output_dir = temp_dir.path().join("custom_output");
        
        let config = SplitConfig {
            input_path: input_path.to_str().unwrap().to_string(),
            chunk_size: 2,
            output_dir: Some(output_dir.to_str().unwrap().to_string()),
            ..Default::default()
        };
        
        let result = split_file(&config)?;
        assert_eq!(result.output_dir, output_dir.canonicalize()?);
        
        Ok(())
    }

    // Test default filename prefix (input filename stem)
    #[test]
    fn test_default_filename_prefix() -> Result<()> {
        let temp_dir = tempdir()?;
        let input_path = create_test_file(temp_dir.path(), "test_prefix.txt", b"test")?;
        
        let config = SplitConfig {
            input_path: input_path.to_str().unwrap().to_string(),
            chunk_size: 2,
            prefix: None, // Should default to input filename stem
            ..Default::default()
        };
        
        let result = split_file(&config)?;
        // New format: test_prefix-001.txt, test_prefix-002.txt
        assert!(result.chunks[0].path.file_name().unwrap().to_str().unwrap().starts_with("test_prefix-"));
        assert!(result.chunks[0].path.file_name().unwrap().to_str().unwrap().ends_with(".txt"));
        
        Ok(())
    }

    // Test custom filename prefix
    #[test]
    fn test_custom_filename_prefix() -> Result<()> {
        let temp_dir = tempdir()?;
        let input_path = create_test_file(temp_dir.path(), "test_custom_prefix.txt", b"test")?;
        
        let config = SplitConfig {
            input_path: input_path.to_str().unwrap().to_string(),
            chunk_size: 2,
            prefix: Some("custom_".to_string()),
            ..Default::default()
        };
        
        let result = split_file(&config)?;
        assert!(result.chunks[0].path.file_name().unwrap().to_str().unwrap().starts_with("custom_"));
        
        Ok(())
    }

    // Test extension preservation with default naming
    #[test]
    fn test_extension_preservation() -> Result<()> {
        let temp_dir = tempdir()?;
        let input_path = create_test_file(temp_dir.path(), "document.txt", b"Hello World!")?; // 12 bytes
        
        let config = SplitConfig {
            input_path: input_path.to_str().unwrap().to_string(),
            chunk_size: 4, // 12 bytes / 4 = 3 chunks exactly
            ..Default::default()
        };
        
        let result = split_file(&config)?;
        
        // Should create document-001.txt, document-002.txt, document-003.txt
        assert_eq!(result.chunks.len(), 3);
        
        let chunk_names: Vec<String> = result.chunks.iter()
            .map(|chunk| chunk.path.file_name().unwrap().to_str().unwrap().to_string())
            .collect();
        
        assert_eq!(chunk_names[0], "document-001.txt");
        assert_eq!(chunk_names[1], "document-002.txt");
        assert_eq!(chunk_names[2], "document-003.txt");
        
        Ok(())
    }

    // Test extension preservation with complex extensions
    #[test]
    fn test_complex_extension_preservation() -> Result<()> {
        let temp_dir = tempdir()?;
        let input_path = create_test_file(temp_dir.path(), "archive.tar.gz", b"compressed data here")?;
        
        let config = SplitConfig {
            input_path: input_path.to_str().unwrap().to_string(),
            chunk_size: 8,
            ..Default::default()
        };
        
        let result = split_file(&config)?;
        
        // Should create archive-001.tar.gz, archive-002.tar.gz, archive-003.tar.gz
        let chunk_names: Vec<String> = result.chunks.iter()
            .map(|chunk| chunk.path.file_name().unwrap().to_str().unwrap().to_string())
            .collect();
        
        assert!(chunk_names[0].ends_with(".tar.gz"));
        assert!(chunk_names[0].starts_with("archive-001"));
        
        Ok(())
    }

    // Test file without extension
    #[test]
    fn test_no_extension_file() -> Result<()> {
        let temp_dir = tempdir()?;
        let input_path = create_test_file(temp_dir.path(), "README", b"This is a readme file")?;
        
        let config = SplitConfig {
            input_path: input_path.to_str().unwrap().to_string(),
            chunk_size: 10,
            ..Default::default()
        };
        
        let result = split_file(&config)?;
        
        // Should create README-001, README-002, README-003
        let chunk_names: Vec<String> = result.chunks.iter()
            .map(|chunk| chunk.path.file_name().unwrap().to_str().unwrap().to_string())
            .collect();
        
        assert_eq!(chunk_names[0], "README-001");
        assert_eq!(chunk_names[1], "README-002");
        assert_eq!(chunk_names[2], "README-003");
        
        Ok(())
    }

    // Test number padding with different digit counts
    #[test]
    fn test_number_padding() -> Result<()> {
        let temp_dir = tempdir()?;
        let input_path = create_test_file(temp_dir.path(), "test_padding.txt", &[b'x'; 30])?;
        
        // Test with 2 digits - new format: test_padding-01.txt
        let config = SplitConfig {
            input_path: input_path.to_str().unwrap().to_string(),
            chunk_size: 10, // Will create 3 chunks
            digits: 2,
            ..Default::default()
        };
        
        let result = split_file(&config)?;
        let chunk_name = result.chunks[0].path.file_name().unwrap().to_str().unwrap();
        assert_eq!(chunk_name, "test_padding-01.txt");
        
        // Test with 4 digits - new format: test_padding-0001.txt
        let config = SplitConfig {
            input_path: input_path.to_str().unwrap().to_string(),
            chunk_size: 10, // Will create 3 chunks
            digits: 4,
            ..Default::default()
        };
        
        let result = split_file(&config)?;
        let chunk_name = result.chunks[0].path.file_name().unwrap().to_str().unwrap();
        assert_eq!(chunk_name, "test_padding-0001.txt");
        
        Ok(())
    }

    // Test handling of non-existent input file
    #[test]
    fn test_nonexistent_input_file() {
        let config = SplitConfig {
            input_path: "nonexistent_file.txt".to_string(),
            chunk_size: 1024,
            ..Default::default()
        };
        
        let result = split_file(&config);
        assert!(matches!(result, Err(SplitError::InvalidInputPath(_))));
    }

    // Test handling of invalid chunk size (zero)
    #[test]
    fn test_zero_chunk_size() {
        let config = SplitConfig {
            input_path: "test.txt".to_string(),
            chunk_size: 0,
            ..Default::default()
        };
        
        let result = split_file(&config);
        assert!(matches!(result, Err(SplitError::InvalidChunkSize(_))));
    }

    // Test handling of invalid output directory (no write permission)
    #[test]
    fn test_invalid_output_directory() -> Result<()> {
        let temp_dir = tempdir()?;
        let input_path = create_test_file(temp_dir.path(), "test_permission.txt", b"test")?;
        
        // On Unix-like systems, we can test write permissions
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            
            // Create a directory without write permissions
            let no_write_dir = temp_dir.path().join("no_write");
            std::fs::create_dir(&no_write_dir)?;
            let mut perms = std::fs::metadata(&no_write_dir)?.permissions();
            perms.set_readonly(true);
            std::fs::set_permissions(&no_write_dir, perms)?;
            
            let config = SplitConfig {
                input_path: input_path.to_str().unwrap().to_string(),
                output_dir: Some(no_write_dir.to_str().unwrap().to_string()),
                chunk_size: 2,
                ..Default::default()
            };
            
            let result = split_file(&config);
            // The function might return either InvalidOutputDir or Io error
            assert!(matches!(
                result,
                Err(SplitError::InvalidOutputDir(_)) | Err(SplitError::Io(_))
            ));
        }
        
        // On non-Unix systems, skip the test
        #[cfg(not(unix))]
        println!("Skipping write permission test on non-Unix system");
        
        Ok(())
    }
}
