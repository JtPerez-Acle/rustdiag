# Development Plan

This document outlines the milestone-driven development plan for RustDiag.

## Milestone 1: Project Setup
- [x] Initialize repository
- [x] Create basic README
- [x] Set up project structure
- [x] Add core dependencies to Cargo.toml
- [x] Add testing framework and initial tests

## Milestone 2: CLI Framework
- [x] Set up clap for command-line parsing
- [x] Implement basic CLI structure with subcommands
- [x] Add logging and error handling framework
- [x] Create help documentation for each command
- [x] Implement config file support

## Milestone 3: Interface Inspection (`inspect` command)
- [x] Implement interface detection
- [x] Display interface details (name, IP, MAC, MTU)
- [x] Add filtering options
- [x] Write tests for interface detection
- [ ] Add interface state detection

## Milestone 4: Port Scanning (`scan` command)
- [x] Implement basic TCP port scanner
- [x] Add async scanning for performance
- [ ] Implement service detection
- [x] Add options for scan types and ranges
- [x] Write tests for port scanning

## Milestone 5: DNS Leak Testing (`dns` command)
- [x] Implement DNS resolver detection
- [x] Add DNS query functionality
- [x] Implement leak detection logic
- [x] Display DNS resolution path
- [x] Write tests for DNS functionality

## Milestone 6: Bandwidth Monitoring (`watch` command)
- [x] Implement real-time bandwidth monitoring
- [x] Add per-interface statistics
- [ ] Implement data collection and analysis
- [ ] Add basic visualization in terminal
- [x] Write tests for monitoring functionality

## Milestone 7: Outbound Traffic Analysis (`outbound` command)
- [x] Implement connection tracking
- [ ] Add geolocation for outbound IPs
- [ ] Implement suspicious connection detection
- [ ] Create visualization of connection map
- [x] Write tests for traffic analysis

## Milestone 8: Smart Summary (`summary` command)
- [x] Implement data collection from all modules
- [x] Create summary report format
- [ ] Add differential analysis between runs
- [ ] Implement recommendation engine
- [x] Write tests for summary functionality

## Milestone 9: TUI Implementation (v0.2.x)
- [ ] Set up TUI framework
- [ ] Implement dashboard layout
- [ ] Add real-time update functionality
- [ ] Create visualization components
- [ ] Implement interactive features

## Milestone 10: Plugin System (v0.3.x)
- [ ] Design plugin API
- [ ] Implement plugin loading mechanism
- [ ] Create example plugins
- [ ] Add plugin documentation
- [ ] Implement safe execution environment

## Milestone 11: Documentation and Polishing (v1.0)
- [ ] Complete user documentation
- [ ] Create tutorial and examples
- [ ] Perform cross-platform testing
- [ ] Optimize performance
- [ ] Prepare release packaging

## Testing Strategy

Following Test-Driven Development (TDD) principles:

1. Write failing tests for each feature before implementation
2. Implement the minimum code necessary to pass the tests
3. Refactor while keeping tests passing

Types of tests:
- Unit tests for core functionality
- Integration tests for CLI behavior
- Mock tests for network-dependent features
- Benchmark tests for performance-critical code
