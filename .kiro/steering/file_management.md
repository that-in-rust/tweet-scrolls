---
inclusion: always
---

# File Management and Code Organization Guidelines

## Critical File Size Requirements

### Maximum File Size Limits
- **Absolute Maximum**: 600 lines per file
- **Optimal Target**: 400-500 lines per file
- **Reasoning**: LLM context window limitations and maintainability

### Enforcement Strategy
- Use Minto Pyramid Principle for systematic decomposition
- Single responsibility per module
- Extract functions/structs when approaching limits
- Prefer multiple focused files over monolithic implementations

### File Size Monitoring
```bash
# Check file sizes regularly
find src -name "*.rs" -exec wc -l {} + | sort -nr

# Alert if any file exceeds 600 lines
find src -name "*.rs" -exec wc -l {} + | awk '$1 > 600 {print "WARNING: " $2 " has " $1 " lines (exceeds 600 limit)"}'
```

## Private Data Handling - CRITICAL SECURITY

### Private Data Location
- **Path**: `/home/amuldotexe/Desktop/GitHub202410/tweet-scrolls/private_data/REALDATA/`
- **Content**: Actual private Twitter archive data
- **Status**: ⚠️ **NEVER COMMIT TO VERSION CONTROL**

### Usage Guidelines
1. **For Test Data Modeling**: 
   - Examine first 200 and last 200 lines only
   - Create anonymized sample data based on structure
   - Never copy actual content or personal identifiers

2. **For Final Testing**:
   - Use for end-to-end validation of processing pipeline
   - Verify anonymization works correctly
   - Test performance with real data volumes

3. **Security Requirements**:
   - Always use relative paths in code
   - Never hardcode absolute paths to private data
   - Ensure .gitignore excludes private_data/ directory
   - Use anonymized hashes in any logs or outputs

### .gitignore Requirements
```
# Private data - NEVER COMMIT
private_data/
REALDATA/
*.twitter-archive
*.json.personal
*_private_*
```

## Code Organization Principles

### Module Structure
- Each module should have single responsibility
- Clear boundaries between processing, models, services
- Re-export commonly used types at module level
- Comprehensive documentation for public APIs

### Testing Strategy
- Test files can exceed 600 lines (testing is different)
- Use helper functions to reduce test code duplication
- Separate integration tests from unit tests
- Mock private data for CI/CD pipelines

### Performance Considerations
- Modular design should not impact runtime performance
- Use appropriate visibility (pub vs private)
- Minimize cross-module dependencies
- Profile memory usage with large datasets