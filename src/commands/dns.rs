use anyhow::Result;
use crate::util::display::OutputFormatter;
use crate::network::dns;

/// Test DNS resolution and check for leaks
pub async fn test_dns(domain: &str, formatter: &OutputFormatter) -> Result<()> {
    formatter.verbose(format!("Resolving domain {}", domain));
    
    // Resolve the domain
    let resolution = dns::resolve_domain(domain).await?;
    
    formatter.section("DNS Resolution Results");
    formatter.item("Domain", &resolution.domain);
    
    if resolution.ip_addresses.is_empty() {
        formatter.item("IP Addresses", "None found");
    } else {
        formatter.item("IP Addresses", format!("{} found", resolution.ip_addresses.len()));
        for ip in &resolution.ip_addresses {
            formatter.item("  -", ip);
        }
    }
    
    // Check for DNS leaks
    formatter.verbose("Testing for DNS leaks");
    let leak_result = dns::test_dns_leaks().await?;
    
    if leak_result.leak_detected {
        formatter.error("DNS leak detected! Your DNS queries may not be secure.");
    } else {
        formatter.item("DNS Leak Test", "No leaks detected");
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_dns_returns_ok() {
        let formatter = OutputFormatter::new(false);
        let result = test_dns("example.com", &formatter).await;
        assert!(result.is_ok());
    }
}
