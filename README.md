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

### Option 1: Install from crates.io (Recommended)

The easiest way to install ChronoSub is directly from crates.io:

```bash
cargo install chrono-sub
```

This will:
- Download the latest stable release (v1.0.0)
- Compile the binary optimized for your system
- Install it globally in your Cargo binary directory (~/.cargo/bin/)

**After installation:**
1. Make sure `~/.cargo/bin` is in your PATH
2. Run `chrono-sub` from anywhere

**To update to the latest version:**
```bash
cargo install chrono-sub --force
```

---

### Option 2: Install from Source

If you prefer to build from source or need the development version:

```bash
# Clone the repository
git clone https://github.com/irwnda/chrono-sub.git
cd chrono-sub

# Build the project
cargo build --release

# The binary will be available at target/release/chrono-sub

# Optional: Install globally
cargo install --path .
```

---

### Verifying Installation

After installation, verify that ChronoSub is installed correctly:

```bash
chrono-sub --version
```

Or simply run:
```bash
chrono-sub
```

This will start the interactive interface if installation was successful.

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

## 📖 Complete Guide

### Understanding Subtitle Synchronization

Subtitle timing issues typically fall into two categories:

1. **Subtitles appear too early** → Use "Slower" (Shift Forward) to add time
2. **Subtitles appear too late** → Use "Faster" (Shift Backward) to subtract time

### How to Determine the Adjustment Amount

**Method 1: Visual Check**
1. Play your video with subtitles
2. Find a clear reference point (e.g., dialogue, sound effect)
3. Note when the subtitle appears vs. when it should appear
4. Use that difference as your adjustment value

**Method 2: Trial and Error**
1. Start with a small adjustment (±500ms)
2. Test the result
3. Adjust incrementally until synchronized

### Common Scenarios

#### Scenario 1: Movie Downloaded from Internet

**Problem:** Downloaded subtitles appear 2-3 seconds early

**Solution:**
```
Direction: Slower (Shift Forward)
Time: 00:00:02,500
```

**Why:** Downloaded subtitles may be from different versions (theatrical vs. director's cut) or have been encoded with different timing offsets.

---

#### Scenario 2: Edited Video

**Problem:** After cutting intro from video, all subtitles are 45 seconds late

**Solution:**
```
Direction: Faster (Shift Backward)
Time: 00:00:45,000
```

**Why:** Removing content from the beginning shifts all timestamps forward.

---

#### Scenario 3: Frame Rate Conversion

**Problem:** Converted video from 24fps to 25fps, subtitles drift progressively

**Solution:** This requires percentage-based adjustment. For each hour of video:
- 24fps → 25fps: Subtitles play ~4% faster
- Adjust approximately: 00:01:30,000 per hour (slower)

**Note:** ChronoSub applies fixed time adjustments. For frame rate issues, you may need specialized tools or calculate cumulative adjustments.

---

#### Scenario 4: Encoding Issues

**Problem:** Subtitles have strange characters or won't load

**Solution:** ChronoSub automatically detects and preserves encoding (UTF-8, UTF-16 LE, Windows-1252). If issues persist:
1. Check source file encoding with: `file -I subtitle.srt`
2. Ensure your video player supports the encoding
3. Convert to UTF-8 if needed using: `iconv`

---

#### Scenario 5: Multiple Adjustments

**Problem:** First attempt wasn't perfect, need to fine-tune

**Solution:**
1. Start with your best estimate
2. Apply adjustment to create `subtitle_adjusted.srt`
3. Test with your video
4. If still off, adjust `subtitle_adjusted.srt` again
5. Repeat until perfect

**Tip:** Keep copies at each stage so you can backtrack if needed

---

### Time Format Quick Reference

| Component | Range | Format |
|-----------|-------|--------|
| Hours | 00-23 | 2 digits |
| Minutes | 00-59 | 2 digits |
| Seconds | 00-59 | 2 digits |
| Milliseconds | 000-999 | 3 digits |

**Separators:**
- **SRT files**: Use comma (`00:01:23,456`)
- **VTT files**: Use period (`00:01:23.456`)

**Common Mistakes:**
- ❌ `0:01:23,456` (single digit hours)
- ❌ `00:1:23,456` (single digit minutes)
- ❌ `00:01:23,45` (2 digit milliseconds)
- ✅ `00:01:23,456` (correct!)

---

### Troubleshooting

#### Issue: "Invalid time format" error

**Check:**
- [ ] All components have correct digit counts
- [ ] Separator matches file type (comma for SRT, period for VTT)
- [ ] No extra spaces or characters
- [ ] Values are within valid ranges

---

#### Issue: Adjusted file has wrong encoding

**Solution:** ChronoSub preserves original encoding. If you see issues:
1. Verify original file encoding
2. Ensure your text editor supports that encoding
3. Consider converting source to UTF-8 first

---

#### Issue: Subtitles still don't sync after adjustment

**Possible causes:**
1. **Non-linear offset**: Some parts need different adjustments than others
   - *Solution:* May need to split file and adjust sections separately

2. **Speed/frame rate mismatch**: Video plays at different speed than subtitles expect
   - *Solution:* Requires time-stretching, not simple shifting

3. **Incorrect reference point**: Initial estimate was wrong
   - *Solution:* Try multiple reference points throughout the video

4. **Corrupted subtitle file**: Timestamps are malformed
   - *Solution:* Validate subtitle file format first

---

### Best Practices

1. **Always backup originals**: Keep `filename.srt.bak` before adjusting
2. **Test incrementally**: Start small, adjust gradually
3. **Use descriptive names**: Rename output to indicate adjustment (e.g., `movie_+2.5s.srt`)
4. **Verify at multiple points**: Check sync at beginning, middle, and end
5. **Document your process**: Note what worked for future reference

### Tips for Specific Use Cases

**For TV Series:**
- Episodes often have consistent offsets within a season
- Once you find the right offset for one episode, try it on others
- Save different offsets for different sources (Web-DL, Blu-ray, etc.)

**For Language Learners:**
- Keep dual subtitle files with different offsets if needed
- Smaller adjustments (±200ms) can significantly improve comprehension

**For Content Creators:**
- Export subtitles from your editor first to check baseline timing
- Apply ChronoSub adjustments as final post-processing step
- Test on multiple video players before publishing

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
