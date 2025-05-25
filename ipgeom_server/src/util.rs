use ipgeom_rir::Database;
use serde_json::Value;
use std::net::IpAddr;

/// Lookup all possible country codes for the given IP address.
pub fn lookup_countries(db: &dyn Database, ip: IpAddr) -> Result<Vec<String>, anyhow::Error> {
    match ip {
        IpAddr::V4(v4) => db.lookup_ipv4_all(v4),
        IpAddr::V6(v6) => db.lookup_ipv6_all(v6),
    }
}

/// Lookup the inetnum/inet6num object for the given IP address and return it as JSON.
pub fn lookup_inet_object(
    db: &dyn Database,
    ip: IpAddr,
) -> Result<Option<(String, Value)>, anyhow::Error> {
    let res = match ip {
        IpAddr::V4(v4) => db.lookup_ipv4_with_obj(v4)?,
        IpAddr::V6(v6) => db.lookup_ipv6_with_obj(v6)?,
    };

    if let Some((_country, obj_type, obj_key)) = res {
        if let Some(json) = db.get_object(&obj_type, &obj_key)? {
            let val: Value = serde_json::from_str(&json)?;
            return Ok(Some((obj_type, val)));
        }
    }

    Ok(None)
}
