# ChronoSub

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-2024-orange.svg)](https://www.rust-lang.org/)
[![Build](https://img.shields.io/badge/build-passing-brightgreen.svg)]()

> An intuitive CLI tool for synchronizing subtitle files with millisecond precision

**ChronoSub** is a command-line utility that helps you fix out-of-sync subtitle files by shifting timestamps forward or backward. Whether your subtitles appear too early or too late, ChronoSub makes it easy to synchronize them perfectly with your video content.

## ✨ Features

- **🎯 Precise Timestamp Adjustment** - Shift subtitles with millisecond accuracy
- **📁 Multiple Format Support** - Works with both `.srt` (SubRip) and `.vtt` (WebVTT) files
- **🔀 Bidirectional Shifting** - Delay subtitles (shift forward) or speed them up (shift backward)
- **🌐 Encoding Preservation** - Automatically detects and preserves original file encoding:
  - UTF-8
  - UTF-16 LE (with BOM)
  - Windows-1252
- **💻 Interactive Interface** - User-friendly terminal UI with guided prompts
- **📝 Flexible Output Options** - Choose how to name your adjusted files:
  - Add suffix or prefix
  - Replace original file
  - Specify custom filename
- **✅ Input Validation** - Real-time validation with helpful error messages
- **🔍 File Browser** - Built-in filesystem navigation for easy file selection

## 🎬 Use Cases

ChronoSub is perfect for situations like:

- **Movie/TV Subtitles** - Fix timing issues when downloaded subtitles don't match your video file
- **Video Editing** - Adjust subtitles after editing video content (cuts, trims, etc.)
- **Content Creation** - Fine-tune subtitle timing for your own video projects

## 📦 Installation

### From Source

```bash
# Clone the repository
git clone https://github.com/irwnda/chrono-sub.git
cd chrono-sub

# Build the project
cargo build --release

# The binary will be available at target/release/chrono-sub
```

## 🚀 Usage

ChronoSub features an interactive interface that guides you through the adjustment process:

```bash
chrono-sub
```

### Step-by-Step Walkthrough

**1. Select your subtitle file**
```
? How would you like to select your subtitle file?
> Use current directory
  Enter a path manually
  Browse filesystem
```

**2. Choose adjustment direction**
```
? Select adjustment direction:
> Slower (Delay/Shift Forward/+Time)
  Faster (Speed up/Shift Backward/-Time)
```

**3. Enter the time adjustment**
```
? Enter time to adjust (format: hh:mm:ss,ms or hh:mm:ss.ms): 00:00:02,500
```

**4. Choose output naming**
```
? Select output file naming:
> Add suffix (_adjusted)
  Add prefix (adjusted_)
  Replace original file
  Custom name
```

### Examples

#### Example 1: Delaying Subtitles
Your subtitles appear 2.5 seconds too early:
- Direction: **Slower** (Shift Forward)
- Time: `00:00:02,500`
- Result: All timestamps shifted forward by 2.5 seconds

#### Example 2: Speeding Up Subtitles
Your subtitles appear 1 second too late:
- Direction: **Faster** (Shift Backward)
- Time: `00:00:01,000`
- Result: All timestamps shifted backward by 1 second

#### Example 3: Micro-adjustments
Fine-tune subtitle sync with millisecond precision:
- Time: `00:00:00,350` (350 milliseconds)
- Result: Precise 350ms shift

### Supported Time Formats

- **SRT format**: `hh:mm:ss,mmm` (e.g., `00:01:23,456`)
- **VTT format**: `hh:mm:ss.mmm` (e.g., `00:01:23.456`)

Both formats use 24-hour notation with:
- Hours: 00-23
- Minutes: 00-59
- Seconds: 00-59
- Milliseconds: 000-999

## 🛠️ Development

### Prerequisites

- Rust Edition 2024 or later
- Cargo (Rust package manager)

### Setting Up Development Environment

```bash
# Clone the repository
git clone https://github.com/irwnda/chrono-sub.git
cd chrono-sub

# Install dependencies
cargo build

# Run tests
cargo test

# Run with debug output
cargo run
```

### Running Tests

The project includes comprehensive unit tests:

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test module
cargo test subtitle::tests
```

### Project Structure

```
ChronoSub/
├── src/
│   ├── main.rs          # Entry point and file selection UI
│   ├── lib.rs           # Library module declarations
│   ├── io.rs            # File browsing and selection logic
│   └── subtitle.rs      # Core timestamp manipulation
├── tests/               # Integration tests (if any)
├── Cargo.toml           # Project configuration
├── Cargo.lock           # Dependency lock file
└── README.md            # This file
```

### Key Dependencies

- `crossterm` - Terminal styling and colors
- `encoding_rs` - Character encoding detection
- `encoding_rs_io` - I/O operations with encoding support
- `inquire` - Interactive command-line prompts
- `regex` - Regular expression pattern matching

## 🤝 Contributing

Contributions are welcome! Here's how to get started:

1. **Fork the repository**
2. **Create a feature branch**
   ```bash
   git checkout -b feature/amazing-feature
   ```
3. **Make your changes**
   - Write clean, idiomatic Rust code
   - Add tests for new functionality
   - Ensure all tests pass: `cargo test`
4. **Commit your changes**
   ```bash
   git commit -m "feat: add amazing feature"
   ```
5. **Push to your branch**
   ```bash
   git push origin feature/amazing-feature
   ```
6. **Open a Pull Request**

### Development Guidelines

- Follow Rust coding conventions (use `cargo fmt` and `cargo clippy`)
- Write tests for new features
- Update documentation as needed
- Keep commits atomic and well-described
- Ensure backward compatibility for file formats

### Areas for Contribution

Potential areas for enhancement:
- [ ] Edit only some parts of the subtitle file
- [ ] Batch processing support for multiple files
- [ ] Support for additional subtitle formats (ASS, SSA, etc.)
- [ ] Preview mode showing affected subtitles
- [ ] Command-line argument mode for scripting
- [ ] Configuration file support

## 📄 License

This project is licensed under the MIT License - see the LICENSE file for details.

```
Copyright (c) 2025 ChronoSub Contributors

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```

## 🙏 Acknowledgments

- Built with [Rust](https://www.rust-lang.org/)
- Uses excellent open-source libraries: [crossterm](https://github.com/crossterm-rs/crossterm), [inquire](https://github.com/mikaelmello/inquire), [encoding_rs](https://github.com/hsivonen/encoding_rs)
- Inspired by the need for simple, precise subtitle synchronization tools

## 📞 Support

- **Issues**: [GitHub Issues](https://github.com/Irwnda/chrono-sub/issues)
- **Discussions**: [GitHub Discussions](https://github.com/Irwnda/chrono-sub/discussions)

---

Made with ❤️ by subtitle enthusiasts everywhere
