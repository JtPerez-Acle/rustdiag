use clap::{Parser, Subcommand};

/// RustDiag - A blazing-fast, secure, and human-friendly network diagnostic tool
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    /// Enable verbose output
    #[clap(short, long)]
    pub verbose: bool,

    /// Subcommands for different diagnostic functions
    #[clap(subcommand)]
    pub command: Commands,
}

/// Subcommands for RustDiag
#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Scan ports on local or remote hosts
    Scan {
        /// Target host to scan (IP or hostname)
        #[clap(required = true)]
        target: String,
        
        /// Port range to scan (e.g., 1-1000)
        #[clap(short, long, default_value = "1-1000")]
        ports: String,
    },
    
    /// Test DNS resolution and check for leaks
    Dns {
        /// Domain to resolve
        #[clap(default_value = "example.com")]
        domain: String,
    },
    
    /// Inspect network interfaces
    Inspect {
        /// Filter interfaces by name
        #[clap(short, long)]
        interface: Option<String>,
    },
    
    /// Monitor bandwidth usage
    Watch {
        /// Interface to monitor
        #[clap(short, long)]
        interface: Option<String>,
        
        /// Interval in seconds between updates
        #[clap(short = 't', long, default_value = "1")]
        interval: u64,
    },
    
    /// Analyze outbound traffic
    Outbound {
        /// Enable geolocation lookup
        #[clap(short, long)]
        geo: bool,
    },
    
    /// Show a summary of all diagnostic information
    Summary,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn verify_cli() {
        use clap::CommandFactory;
        Cli::command().debug_assert();
    }
    
    #[test]
    fn test_parse_scan_command() {
        let cli = Cli::parse_from(["rustdiag", "scan", "localhost", "--ports", "80-443"]);
        
        match cli.command {
            Commands::Scan { target, ports } => {
                assert_eq!(target, "localhost");
                assert_eq!(ports, "80-443");
            },
            _ => panic!("Expected Scan command"),
        }
    }
    
    #[test]
    fn test_parse_dns_command() {
        let cli = Cli::parse_from(["rustdiag", "dns", "google.com"]);
        
        match cli.command {
            Commands::Dns { domain } => {
                assert_eq!(domain, "google.com");
            },
            _ => panic!("Expected Dns command"),
        }
    }
    
    #[test]
    fn test_parse_inspect_command() {
        let cli = Cli::parse_from(["rustdiag", "inspect", "--interface", "eth0"]);
        
        match cli.command {
            Commands::Inspect { interface } => {
                assert_eq!(interface, Some("eth0".to_string()));
            },
            _ => panic!("Expected Inspect command"),
        }
    }
}
