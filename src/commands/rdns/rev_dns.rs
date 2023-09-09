use crate::commands::kanha_helpers::{read_lines, read_urls_from_stdin};
use crate::interface::RdnsArgs;
use dns_lookup::lookup_addr;

#[allow(non_snake_case)]
pub fn Rdns(ip_addr: std::net::IpAddr) -> Result<String, Box<dyn std::error::Error>> {
    let host = lookup_addr(&ip_addr)?;

    Ok(host.to_string())
}

pub async fn reverse_dns_lookup(rdns_args: RdnsArgs) -> Result<(), Box<dyn std::error::Error>> {
    // Type Result<Vec<String>, Box<dyn std::error::Error>>
    let ip_addresses = if rdns_args.stdin {
        read_urls_from_stdin()
    } else if let Some(filename) = rdns_args.filename {
        let lines_result: Result<Vec<_>, _> = read_lines(&filename).await?.collect();
        Ok(lines_result?)
    } else {
        Err("Either stdin or filename must be provided")?
    };

    let ip_addresses = ip_addresses?;

    for line in ip_addresses {
        match line.parse() {
            Ok(ip_addr) => match Rdns(ip_addr) {
                Ok(hostname) => println!("{} => {}", line, hostname),
                Err(e) => eprintln!("Error looking up {}: {}", line, e),
            },
            Err(e) => eprintln!("Error parsing IP address {}: {}", line, e),
        }
    }

    Ok(())
}
