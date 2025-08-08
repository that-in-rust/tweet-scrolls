#!/bin/bash
# Data Safety Check Script for Tweet-Scrolls
# Ensures no private data accidentally gets committed to git

echo "🛡️  TWEET-SCROLLS DATA SAFETY CHECK"
echo "=================================="

# Check if REALDATA folder exists
if [ -d "REALDATA" ]; then
    echo "✅ REALDATA folder found"
    echo "📊 Contents: $(find REALDATA -name "*.js" | wc -l) JavaScript files"
else
    echo "ℹ️  No REALDATA folder found (this is fine)"
fi

# Check git status for any private data files
echo ""
echo "🔍 Checking git status for private data..."
PRIVATE_FILES=$(git status --porcelain | grep -E "(REALDATA|tweets\.js|direct-messages\.js|private_data)" || true)

if [ -z "$PRIVATE_FILES" ]; then
    echo "✅ No private data files in git staging area - SAFE!"
else
    echo "⚠️  WARNING: Private data files detected in git:"
    echo "$PRIVATE_FILES"
    echo ""
    echo "🚨 DANGER: Run 'git reset' to unstage these files!"
fi

# Check git tracking for any private data files
echo ""
echo "🔍 Checking git tracking for private data..."
TRACKED_FILES=$(git ls-files | grep -E "(REALDATA|tweets\.js|direct-messages\.js|private_data)" || true)

if [ -z "$TRACKED_FILES" ]; then
    echo "✅ No private data files tracked by git - SAFE!"
else
    echo "🚨 CRITICAL: Private data files are being tracked by git:"
    echo "$TRACKED_FILES"
    echo ""
    echo "🛠️  To fix: git rm --cached <filename>"
fi

# Check .gitignore coverage
echo ""
echo "🔍 Checking .gitignore protection..."
if grep -q "REALDATA" .gitignore && grep -q "tweets.js" .gitignore; then
    echo "✅ .gitignore properly configured for private data"
else
    echo "⚠️  .gitignore may need updates for private data protection"
fi

# Final safety summary
echo ""
echo "🎯 SAFETY SUMMARY"
echo "================"
if [ -z "$PRIVATE_FILES" ] && [ -z "$TRACKED_FILES" ]; then
    echo "🟢 ALL CLEAR: Your private data is safe from git commits!"
else
    echo "🔴 ACTION REQUIRED: Private data protection needs attention!"
fi

echo ""
echo "📋 Remember:"
echo "  • Never commit REALDATA/ folder"
echo "  • Never commit *.js files from Twitter archives"
echo "  • Always run this script before pushing to remote"
echo "  • Your privacy is protected by local-only processing"