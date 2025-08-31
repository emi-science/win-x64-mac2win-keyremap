<div align="center">
 
# Mac Command → Windows CTRL Remapper

[![Rust](https://img.shields.io/badge/Rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Windows](https://img.shields.io/badge/Platform-Windows-blue.svg)](https://www.microsoft.com/windows)
[![Release](https://img.shields.io/badge/Release-v0.1.0-green.svg)](#)

Seamlessly use Mac keyboard shortcuts on Windows.
 
</div>
---
## Overview

**I created this project because I use a Mac keyboard on my Windows PC and got tired of the muscle memory confusion.** After years of using Mac shortcuts like `Cmd+C`, `Cmd+V`, and `Cmd+Z`, switching to Windows where these become `Ctrl+C`, `Ctrl+V`, and `Ctrl+Z` was incredibly frustrating. My fingers automatically went to the Command key position, but Windows expected the Control key.

This lightweight Rust application runs silently in the background, intercepting Mac Command key presses (Windows key on PC keyboards) and converting them to Windows CTRL key events. Perfect for Mac users who work on Windows machines but prefer their familiar keyboard shortcuts.

### Why This Matters

As someone who made the switch from Mac to Windows (or uses both platforms), I know firsthand how disruptive it can be to relearn keyboard shortcuts. This tool eliminates that friction by making your Mac keyboard behave exactly like it does on macOS, even when you're on Windows.

### Key Benefits

- **Seamless remapping**: Command → Ctrl
- **Zero learning curve**: Use your existing Mac muscle memory
- **Low latency and minimal overhead (Rust)**
- **Lightweight: minimal CPU and memory footprint
- **Zero configuration: works out of the box
---
## Quick Start
### Requirements

- Windows x64 operating system
- Rust compiler (latest stable version) if building from source

### Installation

#### Option 1: Download Binary (Recommended)

1. Download the [latest release](https://github.com/mac2win-keyremap/mac2win-keyremap/releases)
2. Unzip and run the executable
 
#### Option 2: Build from Source
 
```bash
# Clone the repository
git clone https://github.com/mac2win-keyremap/mac2win-keyremap.git
 
# Navigate to project directory
cd mac2win-keyremap
 
# Build in release mode
cargo build --release
 
# The executable will be available at:
# target/release/mac2win-keyremap.exe
```

#### Option 3: Run Directly

```bash
# Run in development mode
cargo run
```

---
## Usage

1. **Run the application**: Double-click the executable or run from command line
2. **System Tray**: The application runs in the background with a system tray icon
3. **Test the remapping**: Try common Mac shortcuts like:
   - `Cmd+C` (Windows key + C) → Copies to clipboard
   - `Cmd+V` (Windows key + V) → Pastes from clipboard
   - `Cmd+Z` (Windows key + Z) → Undo
   - `Cmd+S` (Windows key + S) → Save

4. **Exit**: Right-click the system tray icon and select "Exit"

---
## Technical Details

### How It Works

This application uses Windows low-level keyboard hooks to intercept key events at the system level. When the Windows key (Command key on Mac keyboards) is pressed, it:

1. **Intercepts** the Windows key press
2. **Suppresses** the original Windows key event
3. **Generates** a new CTRL key event
4. **Maintains** key state (press/release) for proper behavior

### Key Features

- **Low-level keyboard hook**: Uses `WH_KEYBOARD_LL` for system-wide interception
- **Real-time processing**: Minimal latency between key press and remapping
- **State management**: Properly handles key up/down states
- **Graceful shutdown**: Clean hook removal on exit
- **Error handling**: Robust error handling and logging

### Performance

- **Memory usage**: ~2-5 MB RAM
- **CPU usage**: <0.1% when idle
- **Latency**: <1ms key processing time

---
## Development

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest stable)
- Windows 10/11 x64

### Building

```bash
# Clone the repository
git clone https://github.com/mac2win-keyremap/mac2win-keyremap.git
cd mac2win-keyremap

# Build for development
cargo build

# Build for release
cargo build --release

# Run tests
cargo test

# Check code formatting
cargo fmt --check

# Run clippy for linting
cargo clippy
```

### Project Structure

```
mac2win-keyremap/
├── src/
│   └── main.rs          # Main application logic
├── .github/
│   └── workflows/
│       └── release.yml  # GitHub Actions for releases
├── Cargo.toml          # Rust package configuration
├── Cargo.lock          # Dependency lock file
└── README.md           # This file
```

---
## Troubleshooting

### Common Issues

#### Application won't start
- Ensure you're running on Windows x64
- Check if antivirus software is blocking the executable
- Run as administrator if needed

#### Remapping not working
- Verify the application is running (check system tray)
- Ensure no other keyboard remapping software is running
- Restart the application

#### High CPU usage
- This is typically caused by rapid key presses
- Restart the application to reset the hook

### Debug Mode

Run with debug logging:

```bash
# Set log level to debug
$env:RUST_LOG="debug"
cargo run
```

---
## Contributing

We welcome contributions! Please see our [Contributing Guide](CONTRIBUTING.md) for details.

### Development Setup

1. Fork the repository
2. Create a feature branch: `git checkout -b feature-name`
3. Make your changes
4. Add tests if applicable
5. Run the test suite: `cargo test`
6. Submit a pull request

---
## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---
## Acknowledgments

- Built with [Rust](https://www.rust-lang.org/) and [windows-rs](https://github.com/microsoft/windows-rs)
- Inspired by the need for seamless cross-platform keyboard experience
- Thanks to the open-source community for tools and libraries

---

**Maintained by [KeyRemap Team](https://github.com/mac2win-keyremap)**

</div>
