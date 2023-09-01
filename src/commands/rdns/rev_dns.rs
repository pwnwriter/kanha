use crate::commands::kanha_helpers::read_lines;
use crate::interface::RdnsArgs;
use dns_lookup::lookup_addr;

#[allow(non_snake_case)]
pub fn Rdns(ip_addr: std::net::IpAddr) -> Result<String, Box<dyn std::error::Error>> {
    let host = lookup_addr(&ip_addr)?;

    Ok(host.to_string())
}

pub async fn reverse_dns_lookup(rdns_args: RdnsArgs) -> Result<(), Box<dyn std::error::Error>> {
    let ip_addresses = read_lines(&rdns_args.filename).await?;

    for line in ip_addresses {
        match line {
            Ok(ip_str) => match ip_str.parse() {
                Ok(ip_addr) => match Rdns(ip_addr) {
                    Ok(hostname) => println!("{} => {}", ip_str, hostname),
                    Err(e) => eprintln!("Error looking up {}: {}", ip_str, e),
                },
                Err(e) => eprintln!("Error parsing IP address {}: {}", ip_str, e),
            },
            Err(e) => eprintln!("Error reading line from file: {}", e),
        }
    }

    Ok(())
}
