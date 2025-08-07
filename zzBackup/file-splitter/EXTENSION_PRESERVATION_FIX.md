# File Extension Preservation Fix

## Problem Statement

The original file-splitter implementation had a significant usability issue with its naming convention:

**Before (Problematic):**
- `document.txt` → `document.001`, `document.002`, `document.003`
- `archive.tar.gz` → `archive.001`, `archive.002`, `archive.003`
- Lost file extension information, making it unclear what type of files the chunks were

## Solution Implemented

Following TDD principles and idiomatic Rust patterns, we implemented an improved naming convention:

**After (Fixed):**
- `document.txt` → `document-001.txt`, `document-002.txt`, `document-003.txt`
- `archive.tar.gz` → `archive-001.tar.gz`, `archive-002.tar.gz`, `archive-003.tar.gz`
- `README` → `README-001`, `README-002`, `README-003` (no extension preserved correctly)

## Key Implementation Details

### 1. Filename Parsing Logic
```rust
let (base_name, extension) = match &config.prefix {
    Some(p) => (p.clone(), String::new()), // Custom prefix, no extension
    None => {
        let file_name = input_path.file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("chunk");
        
        // Handle complex extensions like .tar.gz by finding the first dot
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
```

### 2. Chunk Path Generation
```rust
let chunk_path = output_dir.join(format!(
    "{}-{:0width$}{}",
    base_name,
    chunk_num + 1,
    extension,
    width = config.digits as usize
));
```

## Test-Driven Development Approach

### Tests Added
1. `test_extension_preservation()` - Basic extension preservation
2. `test_complex_extension_preservation()` - Complex extensions like `.tar.gz`
3. `test_no_extension_file()` - Files without extensions
4. Updated existing tests to expect the new format

### Test Results
```
running 16 tests
test tests::test_extension_preservation ... ok
test tests::test_complex_extension_preservation ... ok
test tests::test_no_extension_file ... ok
test tests::test_number_padding ... ok
test tests::test_default_filename_prefix ... ok
[... all other tests ...]
test result: ok. 16 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

## Idiomatic Rust Patterns Applied

- **Pattern 2.1-2.10**: Proper error handling with `Result` types and context
- **Pattern 3.1-3.3**: Owned vs borrowed types (String vs &str)
- **Pattern 11.1-11.10**: Comprehensive test coverage with TDD approach
- **Pattern 0A.11**: Clean build pattern (attempted, but hit dependency issues)

## Benefits of the New Implementation

1. **Preserves File Type**: `document.txt` → `document-001.txt`
2. **Clear Relationship**: Dash separator clearly shows these are chunks
3. **Tool Compatibility**: Other tools can still recognize file types by extension
4. **User Friendly**: Users immediately know what type of data is in each chunk
5. **Handles Complex Extensions**: `archive.tar.gz` → `archive-001.tar.gz`
6. **Backward Compatible**: Custom prefix behavior unchanged

## Examples

| Original File | Old Format (Problematic) | New Format (Fixed) |
|---------------|---------------------------|---------------------|
| `document.txt` | `document.001`, `document.002` | `document-001.txt`, `document-002.txt` |
| `archive.tar.gz` | `archive.001`, `archive.002` | `archive-001.tar.gz`, `archive-002.tar.gz` |
| `data.json` | `data.001`, `data.002` | `data-001.json`, `data-002.json` |
| `README` | `README.001`, `README.002` | `README-001`, `README-002` |
| `script.sh` | `script.001`, `script.002` | `script-001.sh`, `script-002.sh` |

## Conclusion

This fix significantly improves the user experience of the file-splitter tool by making the output files immediately recognizable and maintaining compatibility with other tools that rely on file extensions. The implementation follows idiomatic Rust patterns and was developed using TDD principles to ensure correctness and maintainability.