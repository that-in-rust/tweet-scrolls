#!/bin/bash

# Script to convert all txt files into a single text file with tree structure
# Usage: ./convert_txt_files.sh

RAWDATA_DIR="/home/amuldotexe/Desktop/GitHub202410/rawdata"
OUTPUT_FILE="combined_twitter_convos.txt"

# Check if rawdata directory exists
if [ ! -d "$RAWDATA_DIR" ]; then
    echo "Error: Directory $RAWDATA_DIR does not exist"
    exit 1
fi

# Create output file and add header
echo "=== COMBINED TWITTER CONVERSATIONS ===" > "$OUTPUT_FILE"
echo "Generated on: $(date)" >> "$OUTPUT_FILE"
echo "Source directory: $RAWDATA_DIR" >> "$OUTPUT_FILE"
echo "" >> "$OUTPUT_FILE"

# Add tree structure if tree command is available
if command -v tree &> /dev/null; then
    echo "=== DIRECTORY TREE STRUCTURE ===" >> "$OUTPUT_FILE"
    tree "$RAWDATA_DIR" -a -I '__pycache__|*.pyc|.git' >> "$OUTPUT_FILE"
    echo "" >> "$OUTPUT_FILE"
fi

echo "=== FILE CONTENTS ===" >> "$OUTPUT_FILE"
echo "" >> "$OUTPUT_FILE"

# Find and process all txt files
find "$RAWDATA_DIR" -name "*.txt" -type f | sort | while read -r file; do
    # Get relative path from rawdata directory
    relative_path=$(realpath --relative-to="$RAWDATA_DIR" "$file")
    
    echo "=================================================================================" >> "$OUTPUT_FILE"
    echo "FILE: $RAWDATA_DIR/$relative_path" >> "$OUTPUT_FILE"
    echo "=================================================================================" >> "$OUTPUT_FILE"
    echo "" >> "$OUTPUT_FILE"
    
    # Add file content
    cat "$file" >> "$OUTPUT_FILE"
    echo "" >> "$OUTPUT_FILE"
    echo "" >> "$OUTPUT_FILE"
done

echo "✅ Combined file created: $OUTPUT_FILE"
echo "📊 Total files processed: $(find "$RAWDATA_DIR" -name "*.txt" -type f | wc -l)"
echo "📁 Output file size: $(du -h "$OUTPUT_FILE" | cut -f1)"