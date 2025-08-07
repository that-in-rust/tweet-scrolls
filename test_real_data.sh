#!/bin/bash

echo "🧪 Testing Tweet-Scrolls with REAL DATA"
echo "========================================"
echo "📊 Data Scale:"
echo "  - Tweets: $(wc -l < private_data/REALDATA/tweets.js) lines"
echo "  - DMs: $(wc -l < private_data/REALDATA/direct-messages.js) lines"
echo "  - Total: $(wc -l private_data/REALDATA/*.js | tail -1 | awk '{print $1}') lines"
echo ""

echo "🚀 Starting Tweet-Scrolls with real data..."
echo "⏱️  Start time: $(date)"
echo ""

# Test with automated input:
# 1. tweets.js file path
# 2. screen name: realuser
# 3. DM file path
# 4. Relationship analysis: yes
echo -e "private_data/REALDATA/tweets.js\nrealuser\nprivate_data/REALDATA/direct-messages.js\ny" | timeout 600 ./target/release/tweet-scrolls

echo ""
echo "⏱️  End time: $(date)"
echo "🎯 Test completed!"

# Check if output was generated
if [ -d "private_data/REALDATA/output_realuser_"* ]; then
    echo "✅ Output directory created successfully"
    echo "📁 Generated files:"
    ls -la private_data/REALDATA/output_realuser_*/
    
    # Check if relationship analysis files were created
    if [ -d "private_data/REALDATA/output_realuser_"*/relationship_profiles_* ]; then
        echo "✅ Relationship intelligence files generated"
        echo "📊 Relationship analysis files:"
        ls -la private_data/REALDATA/output_realuser_*/relationship_profiles_*/
    else
        echo "⚠️  No relationship analysis files found"
    fi
else
    echo "❌ No output directory found"
fi

echo ""
echo "🏁 Real data test complete!"