.PHONY: install build-rust build-android build-electron run-android run-ios test lint clean help

# Default target
help:
	@echo "Flip7 MVP - Available targets:"
	@echo "  install       - Install all dependencies (pnpm + cargo)"
	@echo "  build-rust    - Build Rust crates"
	@echo "  build-android - Build Android APK"
	@echo "  build-electron- Build desktop Electron app"
	@echo "  run-android   - Run on Android device/emulator"
	@echo "  run-ios       - Run on iOS device/simulator"
	@echo "  test          - Run all tests"
	@echo "  lint          - Run linting on all code"
	@echo "  clean         - Clean build artifacts"
	@echo "  help          - Show this help message"

# Install dependencies
install:
	@echo "Installing dependencies..."
	cd app && pnpm install
	cd rust/game_core && cargo fetch
	cd rust/net && cargo fetch

# Build Rust crates
build-rust:
	@echo "Building Rust crates..."
	cd rust/game_core && cargo build --release
	cd rust/net && cargo build --release

# Build Android APK
build-android: build-rust
	@echo "Building Android APK..."
	cd app && pnpm install
	cd app && npx react-native bundle --platform android --dev false --entry-file index.js --bundle-output android/app/src/main/assets/index.android.bundle
	cd app/android && ./gradlew assembleRelease
	@echo "APK built: app/android/app/build/outputs/apk/release/"

# Build Electron desktop app
build-electron: build-rust
	@echo "Building Electron app..."
	@if [ ! -d "electron" ]; then \
		mkdir electron; \
		cd electron; \
		echo '{"name":"flip7-desktop","version":"1.0.0","main":"main.js","scripts":{"start":"electron .","build":"electron-builder","dist":"electron-builder --publish=never"},"devDependencies":{"electron":"^25.0.0","electron-builder":"^24.0.0"}}' > package.json; \
		echo 'const {app,BrowserWindow}=require("electron");function createWindow(){const win=new BrowserWindow({width:1200,height:800,webPreferences:{nodeIntegration:true,contextIsolation:false}});win.loadURL("http://localhost:8081");}app.whenReady().then(createWindow);app.on("window-all-closed",()=>{if(process.platform!=="darwin")app.quit();});app.on("activate",()=>{if(BrowserWindow.getAllWindows().length===0)createWindow();});' > main.js; \
	fi
	cd electron && pnpm install
	cd electron && pnpm run build

# Run on Android
run-android:
	cd app && pnpm run android

# Run on iOS
run-ios:
	cd app && pnpm run ios

# Run tests
test:
	@echo "Running Rust tests..."
	cd rust/game_core && cargo test
	cd rust/net && cargo test
	@echo "Running React Native tests..."
	cd app && pnpm test

# Run linting
lint:
	@echo "Running Rust linting..."
	cd rust/game_core && cargo fmt --check && cargo clippy -- -D warnings
	cd rust/net && cargo fmt --check && cargo clippy -- -D warnings
	@echo "Running React Native linting..."
	cd app && pnpm run lint
	cd app && npx tsc --noEmit

# Clean build artifacts
clean:
	@echo "Cleaning build artifacts..."
	cd rust/game_core && cargo clean
	cd rust/net && cargo clean
	cd app && rm -rf node_modules android/app/build ios/build
	rm -rf electron/dist electron/node_modules

# Development server targets
dev-server:
	cd rust/net && cargo run

dev-app:
	cd app && pnpm start

# Multi-instance testing
run-multi-instances:
	@echo "Starting multiple server instances for testing..."
	@for i in 1 2 3; do \
		echo "Starting instance $$i on port $$((8080 + $$i - 1))"; \
		cd rust/net && cargo run -- --port $$((8080 + $$i - 1)) & \
	done; \
	echo "Press Ctrl+C to stop all instances"; \
	wait