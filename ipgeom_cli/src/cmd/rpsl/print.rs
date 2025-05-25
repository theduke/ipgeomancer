use anyhow::Result;
use clap::Args;
use ipgeom_rpsl::{RpslObject, parse_objects_read_iter};
use std::fs::File;
use std::io::BufReader;
use std::net::IpAddr;
use std::path::PathBuf;

#[derive(Args)]
pub struct Print {
    /// Path to the RPSL file
    pub path: PathBuf,
    /// Only print inetnum/inet6num objects that contain one of these IPs
    #[arg(long, value_name = "IP")]
    pub ip: Vec<IpAddr>,
}

pub fn handle(args: Print) -> Result<()> {
    let file = File::open(args.path)?;
    let reader = BufReader::new(file);

    for res in parse_objects_read_iter(reader) {
        let obj = res?;
        if args.ip.is_empty() {
            println!("{}", obj.to_rpsl());
            continue;
        }

        match RpslObject::try_from(obj.clone()).unwrap_or(RpslObject::Other(obj.clone())) {
            RpslObject::Inetnum(inet) => {
                if args.ip.iter().any(|ip| match ip {
                    IpAddr::V4(addr) => inet.inetnum.contains(addr),
                    _ => false,
                }) {
                    println!("{}", obj.to_rpsl());
                }
            }
            RpslObject::Inet6num(inet) => {
                if args.ip.iter().any(|ip| match ip {
                    IpAddr::V6(addr) => inet.inet6num.contains(addr),
                    _ => false,
                }) {
                    println!("{}", obj.to_rpsl());
                }
            }
            _ => {}
        }
    }

    Ok(())
}
