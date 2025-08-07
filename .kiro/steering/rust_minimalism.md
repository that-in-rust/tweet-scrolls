# Rust Minimalism Manifesto

## Core Philosophy
Every line of code is a liability. Every function is a commitment. Every module is a responsibility.

## File Size Enforcement
- **Hard Limit**: 600 lines per file
- **Optimal Target**: 300-500 lines
- **Enforcement**: `find src -name "*.rs" -exec wc -l {} + | awk '$1 > 600 && $2 != "total" {print "❌ VIOLATION: " $2 " (" $1 " lines)"}'`

## Function Design Principles
```rust
// ✅ GOOD: Single responsibility, clear intent
fn calculate_interaction_strength(interactions: u32) -> RelationshipStrength {
    match interactions {
        0..=5 => RelationshipStrength::Minimal,
        6..=20 => RelationshipStrength::Low,
        21..=50 => RelationshipStrength::Medium,
        _ => RelationshipStrength::High,
    }
}

// ❌ BAD: Multiple responsibilities, unclear intent
fn process_user_data_and_generate_reports_with_validation(data: &[User]) -> Result<Vec<Report>> {
    // 50+ lines of mixed concerns
}
```

## Minimalist Code Patterns

### 1. Prefer Composition Over Inheritance
```rust
// ✅ GOOD: Composition
struct UserProfile {
    stats: InteractionStats,
    timeline: Timeline,
}

// ❌ BAD: Complex trait hierarchies
trait BaseUser: Clone + Debug + Send + Sync + ... {}
```

### 2. Essential Error Handling Only
```rust
// ✅ GOOD: Context where it matters
.context("Failed to parse critical user data")?

// ❌ BAD: Verbose error chains
.with_context(|| format!("Error in function {} at line {}", function_name, line_number))?
```

### 3. Minimal Dependencies
- Only add dependencies that solve core problems
- Prefer `std` library solutions when performance difference is negligible
- Question every `use` statement

## Code Review Checklist
Before any commit:
- [ ] Can this function be split into smaller functions?
- [ ] Can this logic be simplified without losing clarity?
- [ ] Are all imports actually used?
- [ ] Does this solve exactly one problem?
- [ ] Would a junior developer understand this in 30 seconds?