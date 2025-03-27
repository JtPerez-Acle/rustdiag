# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.1] - 2025-03-27

### Added
- Added comprehensive DNS resolution path display in the DNS test module

### Changed

### Deprecated

### Removed

### Fixed
- Fixed DNS module test function shadowing issue that caused test failures
- Fixed command line interface option conflict (-i) between interface and interval
- Fixed parameter mismatches in Watch and Outbound command handlers
- Fixed async function calls in main.rs to properly use .await
- Fixed section formatter test to correctly verify output format

### Security

## [0.1.0] - 2025-03-27

### Added
- Initial project setup and structure
- Core dependencies in Cargo.toml
- CLI framework using clap with subcommands:
  - `scan`: Port scanning functionality
  - `dns`: DNS testing and leak detection
  - `inspect`: Network interface inspection
  - `watch`: Bandwidth monitoring
  - `outbound`: Outbound traffic analysis
  - `summary`: Generate diagnostic report
- Output formatting with verbose mode support
- Network interfaces detection module
- Port scanning functionality with port range parsing
- DNS resolution and leak testing with async support
- Traffic monitoring capabilities with bandwidth measurement
- Configuration management system
- Unit tests for all modules following TDD approach
- Added tokio runtime for async functionality

### Changed

### Deprecated

### Removed

### Fixed

### Security

## [Unreleased]

### Added
- Initial project setup

### Changed

### Deprecated

### Removed

### Fixed

### Security
