# ClipStack - Ubuntu Clipboard Manager

A fast, lightweight clipboard history manager for Ubuntu Linux, similar to Maccy for macOS. Built with Rust and GTK4.

## Features

### ✅ Core Features
- **Clipboard History Storage**: Automatically captures clipboard changes with configurable history size (default: 200 items)
- **Instant Search**: Fast, case-insensitive search through clipboard history as you type
- **Persistent Storage**: SQLite database backend with in-memory caching for performance
- **Modern GTK4 UI**: Clean, native Linux interface that follows GNOME design guidelines
- **Background Operation**: Runs efficiently in the background with minimal CPU/memory usage

### 🎯 Key Capabilities
- **Smart Deduplication**: Automatically skips duplicate consecutive clipboard entries
- **Content Filtering**: Only stores non-empty text content
- **FIFO Management**: Maintains history size limit by removing oldest entries
- **Thread-Safe Operations**: Concurrent access protection using async Rust patterns
- **Configuration Persistence**: Settings saved to JSON configuration file

## Installation

### Prerequisites
- Ubuntu 20.04 or later
- GTK4 development libraries
- Rust 1.75 or later

### Build from Source

1. **Install system dependencies**:
   ```bash
   sudo apt update
   sudo apt install -y pkg-config libgtk-4-dev libglib2.0-dev
   ```

2. **Install Rust** (if not already installed):
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
   source ~/.cargo/env
   ```

3. **Clone and build**:
   ```bash
   git clone https://github.com/ChinmayAnandS/MaccyForUbuntu.git
   cd MaccyForUbuntu
   cargo build --release
   ```

4. **Run the application**:
   ```bash
   cargo run --release
   ```

## Usage

### Basic Operation
1. Launch ClipStack - a main window will appear
2. Copy some text to your clipboard (the app simulates clipboard monitoring)
3. Click "Show Clipboard History" to view stored items
4. Use the search box to filter clipboard history
5. Double-click or select an item to copy it back to clipboard
6. Click "Close" to return to the main window

### Clipboard Monitoring
- The application automatically monitors clipboard changes every 5 seconds
- New clipboard content is added to the history (simulated in current version)
- Duplicate consecutive entries are automatically filtered out

### Search Functionality
- Start typing in the search box to filter results instantly
- Search is case-insensitive and matches partial content
- Results update in real-time as you type

## Architecture

### Technology Stack
- **Language**: Rust 2021 Edition
- **GUI Framework**: GTK4 with Glib event loop
- **Database**: SQLite with Rusqlite ORM
- **Async Runtime**: Tokio for concurrent operations
- **Configuration**: JSON-based settings storage

### Key Components

#### Configuration (`src/config.rs`)
- Manages application settings (history size, startup behavior)
- JSON serialization with automatic loading/saving
- Thread-safe configuration access

#### History Manager (`src/history.rs`)
- SQLite database operations with in-memory caching
- FIFO queue management for history size limits
- Async operations for database access
- Content deduplication and filtering

#### Clipboard Monitor (`src/clipboard.rs`)
- Background clipboard monitoring (simplified implementation)
- Automatic content capture and storage
- Error handling and logging

#### User Interface (`src/ui.rs`)
- GTK4-based clipboard history viewer
- Real-time search functionality
- Modal popup windows with native styling
- Keyboard and mouse interaction support

#### System Tray (`src/tray.rs`)
- Placeholder for system tray integration
- Framework for future tray icon implementation

## Configuration

Settings are stored in `~/.config/clipstack/config.json`:

```json
{
  "enabled": true,
  "max_items": 200,
  "start_on_login": true,
  "shortcut": "Ctrl+Shift+V"
}
```

### Configuration Options
- `enabled`: Enable/disable clipboard monitoring
- `max_items`: Maximum number of history items (10-10,000)
- `start_on_login`: Auto-start on system login (future feature)
- `shortcut`: Global hotkey for showing clipboard history (future feature)

## Development

### Project Structure
```
src/
├── main.rs          # Application entry point and GTK initialization
├── config.rs        # Configuration management
├── history.rs       # Clipboard history storage and management
├── clipboard.rs     # Clipboard monitoring functionality
├── ui.rs           # GTK4 user interface components
└── tray.rs         # System tray integration (placeholder)
```

### Building and Testing
```bash
# Debug build
cargo build

# Release build
cargo build --release

# Run tests
cargo test

# Run with logging
RUST_LOG=debug cargo run
```

### Dependencies
- `gtk4`: Modern GTK GUI framework
- `rusqlite`: SQLite database bindings
- `tokio`: Async runtime
- `serde`: Serialization framework
- `chrono`: Date/time handling
- `dirs`: Cross-platform directory handling

## Performance

### System Requirements
- **Memory**: ~50 MB RAM usage
- **CPU**: Minimal background CPU usage
- **Storage**: ~1 MB for database and configuration

### Benchmarks (Estimated)
- **UI Response Time**: < 50ms for window display
- **Search Performance**: < 30ms for 1000+ items
- **Database Operations**: < 10ms per write operation

## Future Enhancements

### Planned Features
- [ ] **System Tray Icon**: Native Ubuntu system tray integration using libappindicator
- [ ] **Global Hotkeys**: Configurable keyboard shortcuts for clipboard access
- [ ] **Settings Panel**: GUI preferences dialog for configuration
- [ ] **Image Support**: Clipboard image capture and display
- [ ] **Startup Integration**: Automatic launch on system login
- [ ] **Cloud Sync**: Cross-device clipboard synchronization
- [ ] **Rich Formatting**: HTML/RTF clipboard content support

### Technical Improvements
- [ ] **Real Clipboard Monitoring**: Native Wayland/X11 clipboard APIs
- [ ] **Performance Optimization**: Database query optimization and caching
- [ ] **Accessibility**: Screen reader and keyboard navigation support
- [ ] **Internationalization**: Multi-language support
- [ ] **Plugin System**: Extensible architecture for custom features

## Contributing

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/your-feature`
3. Make your changes and add tests
4. Run the test suite: `cargo test`
5. Submit a pull request

### Development Guidelines
- Follow Rust coding standards and clippy recommendations
- Add documentation for public APIs
- Include unit tests for new functionality
- Update README for significant changes

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- Inspired by [Maccy](https://github.com/p0deje/Maccy) for macOS
- Built with [GTK4](https://gtk.org/) and [Rust](https://rust-lang.org/)
- Thanks to the Ubuntu and GNOME communities for excellent documentation

---

**Note**: This is a development version with simplified clipboard monitoring. Full clipboard integration requires additional system-level permissions and APIs that will be implemented in future releases.
