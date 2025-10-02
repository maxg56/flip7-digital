# Flip7 MVP Architecture

## Overview

Flip7 is a cross-platform game built with React Native for the frontend and Rust for the game logic and networking backend.

## Project Structure

```
flip7-digital/
├─ .github/workflows/     # CI/CD pipelines
├─ app/                   # React Native application
├─ rust/                  # Rust backend crates
├─ scripts/               # Build and utility scripts
└─ docs/                  # Documentation
```

## Components

### React Native App (`app/`)

- **Language**: TypeScript
- **Framework**: React Native 0.72.x
- **Purpose**: Cross-platform mobile interface
- **Key Features**:
  - Game UI and user interactions
  - Network communication with Rust backend
  - Native bridge integration

### Game Core (`rust/game_core/`)

- **Language**: Rust
- **Purpose**: Core game logic and state management
- **Key Features**:
  - Game state representation
  - Move validation
  - Player management
  - Board logic

### Network Layer (`rust/net/`)

- **Language**: Rust
- **Framework**: Tokio (async runtime)
- **Purpose**: Multiplayer networking and game server
- **Key Features**:
  - Async message handling
  - Game session management
  - Client-server communication

## Architecture Patterns

### Game State Management

- Centralized game state in Rust
- Immutable state updates
- Serialization via Serde for cross-language communication

### Network Protocol

- JSON-based message protocol
- Async/await pattern for non-blocking operations
- Error handling with typed responses

### Cross-Platform Strategy

- React Native for mobile (iOS/Android)
- Electron wrapper for desktop
- Shared Rust core for consistency

## Development Workflow

1. **Rust Development**: Core logic and networking
2. **React Native Development**: UI and user experience
3. **Integration**: Bridge between platforms
4. **Testing**: Unit tests for Rust, integration tests for RN
5. **Build**: Platform-specific build scripts

## Deployment Strategy

- **Mobile**: Native app store distribution
- **Desktop**: Electron-based application
- **Server**: Containerized Rust server deployment

## Security Considerations

- Input validation in Rust core
- Secure client-server communication
- Player authentication and session management