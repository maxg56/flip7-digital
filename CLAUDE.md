# CLAUDE.md - AI Assistant Guide for Flip7 Digital

This document provides comprehensive guidance for AI assistants working on the Flip7 Digital codebase.

## Table of Contents

1. [Project Overview](#project-overview)
2. [Repository Structure](#repository-structure)
3. [Technology Stack](#technology-stack)
4. [Development Workflows](#development-workflows)
5. [Key Conventions](#key-conventions)
6. [Testing Strategy](#testing-strategy)
7. [Common Tasks](#common-tasks)
8. [FFI Bridge Architecture](#ffi-bridge-architecture)
9. [Code Locations Guide](#code-locations-guide)
10. [Tips for AI Assistants](#tips-for-ai-assistants)

---

## Project Overview

**Flip7 Digital** is a cross-platform multiplayer card game built with:
- **Frontend**: React Native (TypeScript) for mobile (iOS/Android) and desktop (Electron)
- **Backend**: Rust for game logic and networking
- **Architecture**: Monorepo with clear separation between Rust core and React Native UI

### Game Concept

Flip7 is a card game where:
- **Deck**: 79 cards total (1×0, 1×1, 2×2, 3×3, ..., 12×12)
- **Objective**: Get closest to 21 without busting (> 21)
- **Special Rule**: "Flip7" = exactly 7 points earns 21 points bonus
- **Gameplay**: Turn-based, players draw or stay until bust/stay
- **Multiplayer**: P2P networking with host-authoritative game state

### Current Development Status

**Completed**:
- ✅ Core game logic in Rust
- ✅ FFI bindings for React Native integration
- ✅ CLI tool for testing and debugging
- ✅ Networking layer foundation
- ✅ CI/CD pipelines for Rust and React Native
- ✅ Build automation via Makefile

**In Progress**:
- 🚧 React Native UI implementation (minimal placeholder exists)
- 🚧 Integration between React Native and Rust FFI
- 🚧 P2P multiplayer gameplay

**Planned**:
- 📋 Game screens (lobby, gameplay, scoreboard)
- 📋 Cross-device multiplayer testing
- 📋 Mobile app store deployment

---

## Repository Structure

```
flip7-digital/
├── .github/
│   └── workflows/
│       ├── ci-rust.yml         # Rust CI pipeline
│       └── ci-rn.yml           # React Native CI pipeline
├── app/                        # React Native application
│   ├── src/
│   │   ├── App.tsx            # Main app entry point (31 lines)
│   │   ├── screens/           # Game screens (planned)
│   │   ├── components/        # Reusable UI components (planned)
│   │   └── services/          # FFI wrapper services (planned)
│   ├── android/               # Android native code
│   ├── ios/                   # iOS native code
│   ├── package.json           # pnpm dependencies
│   └── tsconfig.json          # Strict TypeScript config
├── rust/
│   ├── game_core/             # Core game logic + FFI
│   │   ├── src/
│   │   │   ├── lib.rs        # Main game logic (648 lines)
│   │   │   ├── main.rs       # Demo binary (102 lines)
│   │   │   └── ffi_test.rs   # FFI tests (79 lines)
│   │   └── Cargo.toml        # Dependencies + crate config
│   ├── net/                   # Networking layer
│   │   ├── src/
│   │   │   └── lib.rs        # Game server (190 lines)
│   │   └── Cargo.toml
│   └── cli/                   # Command-line interface
│       ├── src/
│       │   └── main.rs       # CLI tool (253 lines)
│       └── Cargo.toml
├── docs/
│   ├── arch.md               # Architecture documentation
│   └── Types_de_commit.md   # Commit conventions (French)
├── Makefile                  # Build automation (98 lines)
├── README.md                 # Main documentation
└── LICENSE                   # Project license
```

### Key File Purposes

| File | Lines | Purpose |
|------|-------|---------|
| `rust/game_core/src/lib.rs` | 648 | Core game logic, data structures, FFI exports |
| `rust/cli/src/main.rs` | 253 | CLI for testing game scenarios |
| `rust/net/src/lib.rs` | 190 | Async game server with Tokio |
| `rust/game_core/src/main.rs` | 102 | Demo binary showing game usage |
| `rust/game_core/src/ffi_test.rs` | 79 | FFI integration tests |
| `app/src/App.tsx` | 31 | React Native app entry (minimal) |

---

## Technology Stack

### Backend (Rust)

**Language**: Rust 2021 Edition
**Build System**: Cargo
**Crate Types**: `cdylib` (for FFI), `rlib` (for Rust linking)

**Dependencies**:
```toml
serde = { version = "1.0", features = ["derive"] }  # Serialization
serde_json = "1.0"                                  # JSON support
rand_chacha = "0.3"                                 # Deterministic RNG
libc = "0.2"                                        # C FFI
tokio = { version = "1", features = ["full"] }      # Async runtime
uuid = "1.0"                                        # Unique IDs
clap = { version = "4.0", features = ["derive"] }   # CLI parsing
```

**Key Rust Features Used**:
- `OnceLock<Mutex<HashMap>>` for thread-safe global FFI state
- `Arc<RwLock>` for concurrent game state access
- Deterministic shuffling with `ChaCha8Rng` and seeds
- C-compatible FFI with `#[no_mangle]` and `extern "C"`
- Async/await with Tokio for networking

### Frontend (React Native)

**Language**: TypeScript 4.8.4
**Framework**: React Native 0.72.0
**Package Manager**: pnpm 8.15.0

**Dependencies**:
```json
"react": "18.2.0"
"react-native": "0.72.0"
"@react-native-community/netinfo": "^9.4.1"
```

**TypeScript Config**:
- Strict mode enabled
- All strict compiler options turned on
- Extends `@tsconfig/react-native`

**Dev Tools**:
- ESLint with `@react-native/eslint-config`
- Jest for testing
- Prettier for formatting
- Babel for transpilation

### Build Tools

- **Makefile**: Primary build automation
- **Cargo**: Rust builds and testing
- **pnpm**: React Native dependency management
- **Gradle**: Android builds
- **Xcode**: iOS builds (macOS only)

---

## Development Workflows

### Initial Setup

```bash
# 1. Install all dependencies (Rust + React Native)
make install

# 2. Build Rust crates in release mode
make build-rust

# 3. Run tests to verify setup
make test
```

### Common Development Tasks

#### Working on Rust Core Logic

```bash
# Navigate to game core
cd rust/game_core

# Run tests with verbose output
cargo test -- --nocapture

# Check formatting
cargo fmt --check

# Run clippy linting
cargo clippy -- -D warnings

# Build in release mode
cargo build --release

# Run demo binary
cargo run --bin demo
```

#### Working on CLI Tool

```bash
cd rust/cli

# Run CLI with state persistence
cargo run -- new --players 3 --seed 12345
cargo run -- draw player_1
cargo run -- stay player_1
cargo run -- state

# Simulate game from script
cargo run -- simulate game_script.txt
```

#### Working on Networking

```bash
cd rust/net

# Run tests
cargo test

# Start development server
cargo run

# Multi-instance testing
make run-multi-instances
```

#### Working on React Native UI

```bash
cd app

# Start Metro bundler
pnpm start

# Run on Android (separate terminal)
pnpm run android

# Run on iOS (macOS only)
pnpm run ios

# Type checking
npx tsc --noEmit

# Linting
pnpm run lint

# Run tests
pnpm test
```

### Building for Production

```bash
# Android APK
make build-android
# Output: app/android/app/build/outputs/apk/release/

# Desktop (Electron)
make build-electron
# Output: electron/dist/

# Clean all build artifacts
make clean
```

### Testing Workflows

```bash
# Run all tests (Rust + React Native)
make test

# Rust tests only
cd rust/game_core && cargo test
cd rust/net && cargo test

# React Native tests only
cd app && pnpm test

# Linting all code
make lint
```

---

## Key Conventions

### Commit Messages

The project follows **Conventional Commits** (French documentation in `docs/Types_de_commit.md`):

| Type | Usage | Example |
|------|-------|---------|
| `feat` | New feature | `feat(auth): add OAuth login` |
| `fix` | Bug fix | `fix(form): correct validation error` |
| `docs` | Documentation | `docs(readme): update install procedure` |
| `style` | Formatting (no logic change) | `style: reformat with rustfmt` |
| `refactor` | Code restructuring | `refactor: simplify auth logic` |
| `perf` | Performance improvement | `perf: optimize deck shuffling` |
| `test` | Add/modify tests | `test: add Flip7 detection tests` |
| `build` | Build system changes | `build: update Node.js version` |
| `ci` | CI/CD changes | `ci: fix deployment script` |
| `chore` | Maintenance tasks | `chore: clean temp files` |
| `revert` | Revert previous commit | `revert: rollback commit [hash]` |

### Code Style

#### Rust

- **Formatting**: Use `cargo fmt` (enforced in CI)
- **Linting**: Pass `cargo clippy -- -D warnings` (enforced in CI)
- **Naming**:
  - `snake_case` for functions, variables, modules
  - `PascalCase` for structs, enums, traits
  - `SCREAMING_SNAKE_CASE` for constants
- **Documentation**: Use `///` for public APIs
- **Error Handling**: Prefer `Result<T, E>` over panics
- **Testing**: Place tests in `#[cfg(test)]` modules

#### TypeScript

- **Strict Mode**: All strict compiler options enabled
- **Formatting**: Use Prettier (2 spaces, no semicolons in JSX)
- **Linting**: ESLint with React Native config
- **Naming**:
  - `camelCase` for variables, functions
  - `PascalCase` for components, types
  - `UPPER_CASE` for constants
- **Components**: Functional components with hooks
- **Types**: Explicit types for all function parameters and returns

### File Organization

- **No large files**: Keep files under 500 lines when possible
- **Single responsibility**: Each module/component has one clear purpose
- **Tests co-located**: Tests near the code they test
- **Clear exports**: Export public APIs explicitly

---

## Testing Strategy

### Rust Tests

#### Unit Tests (`rust/game_core/src/lib.rs`)

Located in `#[cfg(test)]` module with 5 key tests:

```rust
#[test]
fn test_deck_card_counts()        // Validates 79-card deck composition
#[test]
fn test_bust_detection()          // Tests bust logic (total > 21)
#[test]
fn test_flip7_detection()         // Tests Flip7 combinations (sum = 7)
#[test]
fn test_scoring_accuracy()        // Validates scoring with Flip7 bonus
#[test]
fn test_game_flow()               // Integration test for full game
```

#### FFI Tests (`rust/game_core/src/ffi_test.rs`)

```rust
#[test]
fn test_ffi_new_game()            // Tests FFI game creation
#[test]
fn test_ffi_full_game_flow()      // End-to-end FFI integration test
```

#### Network Tests (`rust/net/src/lib.rs`)

```rust
#[test]
fn test_join_new_game()           // Tests joining game
#[test]
fn test_start_game()              // Tests starting game
```

### React Native Tests

- **Framework**: Jest with `react-native` preset
- **Current Status**: Configuration in place, no tests written yet
- **Planned**: Component tests, integration tests, snapshot tests

### CI/CD Testing

#### Rust CI (`.github/workflows/ci-rust.yml`)

- Triggered on push/PR to main
- Builds all Rust crates
- Runs `cargo test --verbose`
- Checks `cargo fmt --check`
- Runs `cargo clippy -- -D warnings`

#### React Native CI (`.github/workflows/ci-rn.yml`)

- Sets up Node.js 18 and pnpm 8
- Installs dependencies
- Runs TypeScript checks (`tsc --noEmit`)
- Runs ESLint
- Runs Jest tests
- Attempts production build

---

## Common Tasks

### Adding a New Game Feature

1. **Implement logic in Rust**:
   ```bash
   cd rust/game_core
   # Edit src/lib.rs
   cargo test
   cargo fmt
   cargo clippy
   ```

2. **Expose via FFI** (if needed for React Native):
   ```rust
   #[no_mangle]
   pub extern "C" fn flip7_new_feature(...) -> *mut c_char {
       // Implementation
   }
   ```

3. **Add FFI tests**:
   ```bash
   # Edit src/ffi_test.rs
   cargo test
   ```

4. **Test with CLI**:
   ```bash
   cd ../cli
   # Add CLI command in src/main.rs
   cargo run -- <your-command>
   ```

5. **Update React Native** (when ready):
   ```typescript
   // app/src/services/GameService.ts
   // Add wrapper for FFI function
   ```

### Adding a Test

**Rust**:
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_your_feature() {
        // Arrange
        let mut game = GameState::new();

        // Act
        let result = game.your_feature();

        // Assert
        assert_eq!(result, expected);
    }
}
```

**TypeScript**:
```typescript
import { render } from '@testing-library/react-native';
import YourComponent from './YourComponent';

describe('YourComponent', () => {
  it('should render correctly', () => {
    const { getByText } = render(<YourComponent />);
    expect(getByText('Expected Text')).toBeTruthy();
  });
});
```

### Debugging Game State

**Using CLI**:
```bash
cd rust/cli

# Start new game
cargo run -- new --players 3 --seed 12345

# Check state
cargo run -- state | jq '.'

# Simulate specific scenario
echo "draw player_1
stay player_1
draw player_2" > scenario.txt
cargo run -- simulate scenario.txt
```

**Using Demo Binary**:
```bash
cd rust/game_core
cargo run --bin demo
# Outputs JSON game state to console
```

### Updating Dependencies

**Rust**:
```bash
cd rust/game_core
cargo update
cargo test
```

**React Native**:
```bash
cd app
pnpm update
pnpm test
```

---

## FFI Bridge Architecture

### Overview

The FFI (Foreign Function Interface) bridge allows React Native to call Rust functions. This is the critical integration point between frontend and backend.

### Key Components

#### 1. Global Game State Storage

Located in `rust/game_core/src/lib.rs`:

```rust
use std::sync::{OnceLock, Mutex};
use std::collections::HashMap;

static GAMES: OnceLock<Mutex<HashMap<String, GameState>>> = OnceLock::new();

fn get_games() -> &'static Mutex<HashMap<String, GameState>> {
    GAMES.get_or_init(|| Mutex::new(HashMap::new()))
}
```

**Why**: FFI functions are stateless; we need persistent storage across calls.

#### 2. FFI Functions

All FFI functions follow this pattern:

```rust
#[no_mangle]
pub extern "C" fn flip7_function_name(param: *const c_char) -> *mut c_char {
    // 1. Convert C string to Rust string
    let input = unsafe { CStr::from_ptr(param).to_str().unwrap() };

    // 2. Parse JSON input
    let data: InputType = serde_json::from_str(input).unwrap();

    // 3. Perform operation
    let result = perform_operation(data);

    // 4. Serialize result to JSON
    let json = serde_json::to_string(&result).unwrap();

    // 5. Convert to C string
    CString::new(json).unwrap().into_raw()
}
```

**Memory Management**: React Native must call `flip7_free_string()` to free returned strings.

#### 3. Current FFI API

| Function | Input | Output | Purpose |
|----------|-------|--------|---------|
| `flip7_new_game(players)` | JSON: `{game_id, players: [{id, name}]}` | JSON: `{game_id, status: "created"}` | Initialize new game |
| `flip7_get_state(game_id)` | JSON: `{game_id}` | JSON: Full `GameState` | Get current state |
| `flip7_draw(game_id, player_id)` | JSON: `{game_id, player_id}` | JSON: Updated `GameState` | Player draws card |
| `flip7_stay(game_id, player_id)` | JSON: `{game_id, player_id}` | JSON: Updated `GameState` | Player stays |
| `flip7_free_string(ptr)` | C pointer | None | Free allocated string |

### FFI Data Flow

```
React Native (TypeScript)
    ↓ JSON string
[FFI Boundary]
    ↓ Parse JSON
Rust (game_core)
    ↓ Execute logic
    ↓ Serialize result
[FFI Boundary]
    ↑ Return JSON string
React Native (TypeScript)
    ↑ Parse JSON
```

### Example FFI Usage (from tests)

```rust
// Create game
let input = r#"{"game_id":"test-1","players":[{"id":"p1","name":"Alice"},{"id":"p2","name":"Bob"}]}"#;
let input_c = CString::new(input).unwrap();
let result = flip7_new_game(input_c.as_ptr());

// Get state
let state_input = r#"{"game_id":"test-1"}"#;
let state_c = CString::new(state_input).unwrap();
let state = flip7_get_state(state_c.as_ptr());

// Draw card
let draw_input = r#"{"game_id":"test-1","player_id":"p1"}"#;
let draw_c = CString::new(draw_input).unwrap();
let new_state = flip7_draw(draw_c.as_ptr());

// Always free!
flip7_free_string(result);
flip7_free_string(state);
flip7_free_string(new_state);
```

### FFI Best Practices

1. **Always validate input**: Check JSON parsing, validate IDs
2. **Handle errors gracefully**: Return error JSON instead of panicking
3. **Free memory**: React Native must call `flip7_free_string()`
4. **Use thread-safe storage**: `Mutex<HashMap>` for game state
5. **Test thoroughly**: Use `ffi_test.rs` for integration tests

---

## Code Locations Guide

### When modifying game rules or logic

**File**: `rust/game_core/src/lib.rs`
**Key structs**: `Card`, `Deck`, `Hand`, `Player`, `GameState`
**Key methods**:
- `GameState::new()` - Initialize game
- `GameState::start_round()` - Begin round
- `GameState::player_draw()` - Player draws card
- `GameState::player_stay()` - Player stays
- `GameState::compute_scores()` - Calculate scores

### When adding/modifying FFI exports

**File**: `rust/game_core/src/lib.rs` (bottom section)
**Pattern**: Follow `flip7_*` naming convention
**Test file**: `rust/game_core/src/ffi_test.rs`

### When working on CLI commands

**File**: `rust/cli/src/main.rs`
**Framework**: Clap with derive macros
**State file**: `game_state.json` (auto-created)

### When working on networking

**File**: `rust/net/src/lib.rs`
**Key structs**: `GameServer`, `GameMessage`, `GameResponse`
**Runtime**: Tokio async/await

### When building React Native UI

**Main entry**: `app/src/App.tsx`
**Future locations**:
- `app/src/screens/` - Game screens (lobby, game, scores)
- `app/src/components/` - Reusable UI components
- `app/src/services/` - FFI wrapper services

### When updating build process

**File**: `Makefile`
**Targets**: See `make help` for all available commands

### When modifying CI/CD

**Files**:
- `.github/workflows/ci-rust.yml` - Rust pipeline
- `.github/workflows/ci-rn.yml` - React Native pipeline

---

## Tips for AI Assistants

### Understanding Context

1. **Read recent commits**: Check git log to understand recent changes
   ```bash
   git log --oneline -10
   ```

2. **Check current branch**: Verify which branch you're on
   ```bash
   git branch --show-current
   ```

3. **Understand issue context**: If working on a GitHub issue, read related PRs and comments

### Code Changes Best Practices

1. **Test before committing**: Always run tests after changes
   ```bash
   make test
   make lint
   ```

2. **Follow conventions**: Use appropriate commit type (feat/fix/docs/etc.)

3. **Keep changes focused**: One logical change per commit

4. **Update tests**: Add/update tests for new functionality

5. **Check FFI compatibility**: When modifying game logic, ensure FFI exports still work

### Common Pitfalls to Avoid

1. **Don't break FFI**: Changes to `GameState` structure affect FFI serialization
2. **Don't skip tests**: CI will fail if tests don't pass
3. **Don't ignore warnings**: Clippy warnings must be resolved
4. **Don't forget memory management**: FFI strings must be freed
5. **Don't hardcode values**: Use configuration or constants

### Useful Commands for Investigation

```bash
# Find where a function is defined
cd rust && rg "fn player_draw"

# Find all FFI exports
cd rust && rg "#\[no_mangle\]"

# Check game state structure
cd rust/game_core && cargo doc --open

# Find TODO comments
rg "TODO|FIXME"

# Check dependency tree
cd rust/game_core && cargo tree

# See what changed recently
git diff HEAD~5..HEAD --stat
```

### When Stuck

1. **Check demo binary**: See how game is used in `rust/game_core/src/main.rs`
2. **Use CLI for testing**: Quickly test game scenarios without React Native
3. **Read tests**: Tests show expected behavior
4. **Check architecture docs**: Review `docs/arch.md` for design decisions

### Performance Considerations

1. **Deterministic RNG**: Use seeds for reproducible games (testing/debugging)
2. **JSON serialization**: Can be slow; consider binary formats for large data
3. **Mutex contention**: FFI game storage uses locks; minimize lock time
4. **React Native bridge**: FFI calls have overhead; batch when possible

### Security Considerations

1. **Input validation**: Always validate JSON input in FFI functions
2. **Player authentication**: Network layer needs auth (not implemented yet)
3. **Game state integrity**: Validate moves are legal before applying
4. **Memory safety**: Rust prevents most issues, but FFI requires care

---

## Additional Resources

- **Main README**: `/home/user/flip7-digital/README.md`
- **Architecture Doc**: `/home/user/flip7-digital/docs/arch.md`
- **Commit Guidelines**: `/home/user/flip7-digital/docs/Types_de_commit.md`
- **Work Plan**: `/home/user/flip7-digital/Plan De Travail — Flip7 Mvp (attaque Rapide).md`

---

## Quick Reference Card

### Most Common Commands

| Task | Command |
|------|---------|
| Install dependencies | `make install` |
| Build Rust | `make build-rust` |
| Run all tests | `make test` |
| Run linting | `make lint` |
| Clean build | `make clean` |
| Run CLI | `cd rust/cli && cargo run -- <cmd>` |
| Run Android | `make run-android` |
| Check TypeScript | `cd app && npx tsc --noEmit` |

### Key File Line Counts (for context)

| File | Lines | Complexity |
|------|-------|------------|
| `rust/game_core/src/lib.rs` | 648 | High - Core logic |
| `rust/cli/src/main.rs` | 253 | Medium - CLI interface |
| `rust/net/src/lib.rs` | 190 | Medium - Networking |
| `rust/game_core/src/main.rs` | 102 | Low - Demo |
| `rust/game_core/src/ffi_test.rs` | 79 | Low - Tests |
| `app/src/App.tsx` | 31 | Low - Placeholder |

### Project Metrics

- **Total Rust Code**: ~1,091 lines (main files)
- **Total TypeScript Code**: ~31 lines (minimal)
- **Test Coverage**: 7 test functions across 3 files
- **Dependencies**: 11 Rust crates, 15+ npm packages

---

**Last Updated**: 2025-11-14
**Codebase Version**: Based on commit `a830ddc` and earlier
