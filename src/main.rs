mod commands;
mod network;
mod util;

use anyhow::Result;
use clap::Parser;
use util::cli::{Cli, Commands};
use util::display::OutputFormatter;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();
    
    // Parse command line arguments
    let cli = Cli::parse();
    
    // Create output formatter
    let formatter = OutputFormatter::new(cli.verbose);
    
    // Process commands
    match cli.command {
        Commands::Scan { target, ports } => {
            formatter.section("Port Scan");
            formatter.item("Target", &target);
            formatter.item("Port Range", &ports);
            formatter.verbose(format!("Starting scan of {} on ports {}", target, ports));
            commands::scan::scan_ports(&target, &ports, &formatter)?;
        },
        Commands::Dns { domain } => {
            formatter.section("DNS Test");
            formatter.item("Domain", &domain);
            formatter.verbose(format!("Testing DNS for {}", domain));
            commands::dns::test_dns(&domain, &formatter).await?;
        },
        Commands::Inspect { interface } => {
            formatter.section("Interface Inspection");
            if let Some(ref iface) = interface {
                formatter.item("Interface", iface);
                formatter.verbose(format!("Inspecting interface {}", iface));
            } else {
                formatter.verbose("Inspecting all interfaces");
            }
            commands::inspect::inspect_interfaces(interface.as_deref(), &formatter)?;
        },
        Commands::Watch { interface, interval } => {
            formatter.section("Bandwidth Monitoring");
            if let Some(iface) = &interface {
                formatter.item("Interface", iface);
            } else {
                formatter.verbose("Monitoring all interfaces");
            }
            formatter.item("Interval", format!("{} seconds", interval));
            commands::watch::monitor_bandwidth(interface.as_deref(), interval, &formatter)?;
        },
        Commands::Outbound { geo } => {
            formatter.section("Outbound Traffic Analysis");
            if geo {
                formatter.item("Geolocation", "Enabled");
                formatter.verbose("Analyzing outbound traffic with geolocation");
            } else {
                formatter.verbose("Analyzing all outbound traffic");
            }
            commands::outbound::analyze_outbound(geo, &formatter)?;
        },
        Commands::Summary => {
            formatter.section("Network Diagnostic Summary");
            formatter.verbose("Generating comprehensive network diagnostic summary");
            commands::summary::generate_summary(&formatter)?;
        },
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_main_returns_ok() {
        // This is a simple test to ensure the main function returns Ok
        // In a real test, we would mock the CLI arguments and verify behavior
        // For now, this test will fail since we're not providing CLI arguments
        // We'll update this in a more comprehensive test suite
    }
    
    #[test]
    fn test_cli_parsing() {
        // Test that the CLI parser works correctly
        let args = vec!["rustdiag", "scan", "localhost", "--ports", "80-443"];
        let cli = Cli::parse_from(args);
        
        match cli.command {
            Commands::Scan { target, ports } => {
                assert_eq!(target, "localhost");
                assert_eq!(ports, "80-443");
            },
            _ => panic!("Expected Scan command"),
        }
    }
}
