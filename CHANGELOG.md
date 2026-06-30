# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.0.0] - 2026-06-30

### Added
- Interactive command-line interface for subtitle timestamp adjustment
- Support for `.srt` (SubRip) subtitle files
- Support for `.vtt` (WebVTT) subtitle files
- Bidirectional timestamp shifting (forward/backward)
- Millisecond-precision timestamp manipulation
- Automatic encoding detection and preservation (UTF-8, UTF-16 LE, Windows-1252)
- Flexible output naming options (suffix, prefix, replace, custom)
- Built-in file browser for easy file selection
- Time format validation with helpful error messages
- Comprehensive documentation with usage guide and scenarios

### Documentation
- Added comprehensive README with installation instructions
- Added complete usage guide with common scenarios
- Added troubleshooting section
- Added contributing guidelines (CONTRIBUTING.md)
- Added MIT license (LICENSE)
- Added GitHub issue and PR templates
- Added this CHANGELOG.md

---

## [Unreleased]

### Potential Features (Roadmap)
- [ ] Batch processing support for multiple files
- [ ] Support for additional subtitle formats (ASS, SSA, SUB, etc.)
- [ ] Preview mode showing affected subtitles before applying changes
- [ ] Command-line argument mode for non-interactive scripting
- [ ] Recursive directory processing
- [ ] Configuration file support
- [ ] Edit specific parts of subtitle file (time ranges)
- [ ] Show diff/preview of changes before saving
- [ ] Undo functionality for adjustments
- [ ] Automatic sync detection by analyzing audio waveform
- [ ] GUI interface for easier use

---

## Versioning Scheme

- **Major version (X.0.0)**: Breaking changes or major milestones
- **Minor version (1.X.0)**: New features (backward compatible)
- **Patch version (1.0.X)**: Bug fixes (backward compatible)

---

## Links

- [GitHub Repository](https://github.com/Irwnda/chrono-sub)
- [crates.io Package](https://crates.io/crates/chrono-sub)
- [Issue Tracker](https://github.com/Irwnda/chrono-sub/issues)
