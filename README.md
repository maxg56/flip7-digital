# Flip7 MVP

A cross-platform multiplayer game built with React Native and Rust.

## Architecture

- **Frontend**: React Native (TypeScript)
- **Backend**: Rust (game logic + networking)
- **Platforms**: Mobile (iOS/Android) + Desktop (Electron)

## Quick Start

### Prerequisites

- Node.js 18+
- pnpm 8+
- Rust 1.70+
- React Native CLI
- Android Studio (for Android builds)
- Xcode (for iOS builds, macOS only)

### Development

1. **Install dependencies**:
   ```bash
   make install
   ```

2. **Build Rust crates**:
   ```bash
   make build-rust
   ```

3. **Run on mobile**:
   ```bash
   make run-android  # or make run-ios
   ```

### Make Targets

- `make install` - Install all dependencies
- `make build-rust` - Build Rust crates
- `make build-android` - Build Android APK
- `make build-electron` - Build desktop app
- `make test` - Run all tests
- `make lint` - Run linting
- `make clean` - Clean build artifacts

## Project Structure

```
flip7-digital/
├─ app/                    # React Native app
│  ├─ src/
│  │  ├─ App.tsx          # Main app component
│  │  ├─ screens/         # Game screens
│  │  ├─ components/      # Reusable components
│  │  └─ services/        # Native bridge wrappers
│  ├─ package.json
│  └─ tsconfig.json
├─ rust/
│  ├─ game_core/          # Core game logic
│  └─ net/                # Networking layer
├─ scripts/               # Build utilities
└─ docs/                  # Documentation
```

## Development Workflow

1. Implement game logic in `rust/game_core`
2. Add networking features in `rust/net`
3. Create React Native UI in `app/src`
4. Test with build scripts
5. Deploy via CI/CD pipelines

## CI/CD

- **Rust CI**: Automated testing, formatting, linting
- **React Native CI**: TypeScript checking, testing, building

## Contributing

1. Follow existing code conventions
2. Add tests for new features
3. Update documentation as needed
4. Use provided build scripts for consistency