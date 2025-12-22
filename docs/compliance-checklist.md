# Compliance Checklist

This document contains mandatory rules that MUST be followed when working on this project.

## Code Structure Rules

### No Functions in lib.rs or mod.rs

**RULE**: No functions may be defined in `lib.rs` or `mod.rs` files anywhere in this project.

These files should ONLY contain:
- Module declarations (`pub mod foo;`)
- Re-exports (`pub use foo::bar;`)
- Documentation comments

**Correct structure:**
```
src/
├── lib.rs          # Only: pub mod video; pub use video::duration::get_duration;
└── video/
    ├── mod.rs      # Only: pub mod duration; pub mod frames;
    ├── duration.rs # Functions go here
    └── frames.rs   # Functions go here
```

**Wrong:**
```rust
// lib.rs - WRONG
pub fn my_function() { ... }  // NO! Move to a dedicated file
```

### No Disabling Lint Checks

**RULE**: Never disable clippy or lint checks with `#[allow(...)]` annotations.

- Fix the underlying issue instead of suppressing warnings
- If code is unused, remove it
- If there's dead code, don't add it until needed

## Scripts Usage

### Build, Check, Format, Run

Always use the scripts in `yt-rs/scripts/` for project operations:

- `scripts/build-all.sh` - Build all components
- `scripts/check-all.sh` - Run clippy on all components
- `scripts/format-all.sh` - Format all components
- `scripts/run.sh` - Run the server

**Do NOT use:**
- `cargo run` directly
- `trunk serve`
- Python or other servers

## Pre-Commit Checklist

Before committing, ensure:

1. [ ] `scripts/format-all.sh` passes
2. [ ] `scripts/check-all.sh` passes (no clippy warnings)
3. [ ] No functions in any `lib.rs` or `mod.rs` files
4. [ ] No `#[allow(...)]` annotations added
5. [ ] No dead/unused code added
