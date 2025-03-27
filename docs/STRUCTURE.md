# Project Structure

```
rustdiag/
├── Cargo.toml       # Project configuration and dependencies
├── README.md        # Project overview and documentation
├── CLAUDE.md        # Information for Claude to reference
├── docs/            # Project documentation
│   ├── CHANGELOG.md           # Record of all changes
│   ├── STRUCTURE.md           # This file - overview of project structure
│   └── DEVELOPMENT_PLAN.md    # Milestone-driven development checklist
└── src/             # Source code
    ├── main.rs                # Entry point, CLI setup
    ├── commands/             # CLI command handlers
    │   ├── mod.rs
    │   ├── scan.rs           # Port scanning
    │   ├── dns.rs            # DNS leak testing
    │   ├── inspect.rs        # Interface inspection
    │   ├── watch.rs          # Bandwidth monitoring
    │   ├── outbound.rs       # Traffic analysis
    │   └── summary.rs        # Information summary
    ├── network/              # Core network functionality
    │   ├── mod.rs
    │   ├── interfaces.rs     # Interface detection
    │   ├── ports.rs          # Port scanning logic
    │   ├── dns.rs            # DNS resolution logic
    │   └── traffic.rs        # Traffic monitoring
    └── util/                 # Utility functions
        ├── mod.rs
        ├── display.rs        # Output formatting
        └── config.rs         # Configuration handling
```

The structure follows a modular design approach with clear separation of concerns:

- **commands/**: Contains the implementation of CLI subcommands, with each file corresponding to a specific subcommand.
- **network/**: Contains the core network functionality, independent of the CLI layer.
- **util/**: Contains utility functions for output formatting, configuration, and other shared functionality.

This separation allows for:
1. Easy testing of core functionality without CLI dependencies
2. Clear organization of code based on feature areas
3. Foundation for future plugin architecture (planned for v0.3.x)
