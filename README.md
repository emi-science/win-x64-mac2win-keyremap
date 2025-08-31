<div align="center">

# Mac to Windows Key Remapper

[![Rust](https://img.shields.io/badge/Rust-1.70%2B-orange.svg)](https://www.rust-lang.org/) [![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT) [![Windows](https://img.shields.io/badge/Platform-Windows-blue.svg)](https://www.microsoft.com/windows) [![Release](https://img.shields.io/badge/Release-v0.1.0-green.svg)](#)

**Seamlessly use Mac keyboard shortcuts on Windows by remapping the `Command` key to `Ctrl`.**

</div>

---

## Overview

This lightweight, background utility is for Mac users who work on Windows. If you're accustomed to `Cmd+C`, `Cmd+V`, and other classic Mac shortcuts, this tool eliminates the need to switch to the `Ctrl` key on Windows. It intercepts the `Command` key (Windows key on PC keyboards) and translates it to a `Ctrl` key press, preserving your muscle memory and workflow.

## Features

- **Seamless Remapping**: `Command` key is intelligently remapped to `Ctrl`.
- **Zero Configuration**: Works out-of-the-box with no setup required.
- **High Performance**: Built in Rust for minimal CPU and memory usage (~2-5 MB RAM).
- **System Tray Icon**: Manages the application lifecycle quietly from the system tray.

## Quick Start

### 1. Get the Application

**Option A: Download (Recommended)**

Download the latest executable from the [Releases](https://github.com/mac2win-keyremap/mac2win-keyremap/releases) page.

**Option B: Build from Source**

```bash
# Clone the repository
git clone https://github.com/mac2win-keyremap/mac2win-keyremap.git
cd mac2win-keyremap

# Build for release
cargo build --release
# The executable will be in `target/release/`
```

### 2. Run the Application

Double-click `mac2win-keyremap.exe`. The application will run in the background and appear in your system tray.

### 3. Usage

Your Mac shortcuts will now work as expected:

- `Cmd+C` → `Ctrl+C` (Copy)
- `Cmd+V` → `Ctrl+V` (Paste)
- `Cmd+Z` → `Ctrl+Z` (Undo)
- `Cmd+S` → `Ctrl+S` (Save)

To exit, right-click the system tray icon and select **Exit**.

---

## Development

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest stable)
- Windows 10/11 x64

### Commands

```bash
# Build for development
cargo build

# Run tests
cargo test

# Check formatting
cargo fmt --check

# Run linter
cargo clippy
```

## How It Works

This tool uses the Windows `WH_KEYBOARD_LL` hook to intercept low-level keyboard events. It suppresses the original `WinKey` event and injects a `Ctrl` key event, ensuring system-wide compatibility with minimal latency.

## Contributing

Contributions are welcome! Please fork the repository, create a feature branch, and submit a pull request.

## License

This project is licensed under the MIT License.
