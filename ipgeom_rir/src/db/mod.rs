use std::net::{Ipv4Addr, Ipv6Addr};

use ipgeom_rpsl::RpslObject;

/// Trait describing database backends that can store RPSL information and
/// provide IP geolocation lookups.
pub trait Database: Send + Sync {
    /// Run pending migrations if necessary.
    fn migrate(&self) -> Result<(), anyhow::Error>;

    /// Insert or update a single RPSL object in the database.
    ///
    /// The object is stored as JSON together with its type, unique key and
    /// source.  Any associated geoip mappings are updated as well.
    fn upsert_rpsl_object(&self, obj: &RpslObject) -> Result<(), anyhow::Error>;

    /// Insert or update multiple RPSL objects in a single batch.
    //
    // NOTE: batch inserts with multiple objects are important for performance
    fn upsert_rpsl_objects(&self, objs: &[RpslObject]) -> Result<(), anyhow::Error>;

    /// Fetch the raw JSON for an RPSL object by its type and key.
    fn get_object(&self, obj_type: &str, obj_key: &str) -> Result<Option<String>, anyhow::Error>;

    /// Perform a lookup for an IPv4 address. Returns the country code if found.
    fn lookup_ipv4(&self, addr: Ipv4Addr) -> Result<Option<String>, anyhow::Error>;

    /// Perform a lookup for an IPv6 address. Returns the country code if found.
    fn lookup_ipv6(&self, addr: Ipv6Addr) -> Result<Option<String>, anyhow::Error>;

    /// Perform a lookup for an IPv4 address and return all matching country codes.
    fn lookup_ipv4_all(&self, addr: Ipv4Addr) -> Result<Vec<String>, anyhow::Error>;

    /// Perform a lookup for an IPv6 address and return all matching country codes.
    fn lookup_ipv6_all(&self, addr: Ipv6Addr) -> Result<Vec<String>, anyhow::Error>;

    /// Lookup an IPv4 address and return the country code together with the
    /// referenced object type and key.
    fn lookup_ipv4_with_obj(
        &self,
        addr: Ipv4Addr,
    ) -> Result<Option<(String, String, String)>, anyhow::Error>;

    /// Lookup an IPv6 address and return the country code together with the
    /// referenced object type and key.
    fn lookup_ipv6_with_obj(
        &self,
        addr: Ipv6Addr,
    ) -> Result<Option<(String, String, String)>, anyhow::Error>;
}

pub mod sqlite;

/// Compute a deterministic key identifying an RPSL object.
pub fn object_key(obj: &RpslObject) -> String {
    use iprange::{IpNet, IpRange};

    fn range_to_string<N: IpNet + std::fmt::Display>(range: &IpRange<N>) -> String {
        range
            .iter()
            .map(|n| n.to_string())
            .collect::<Vec<_>>()
            .join(",")
    }

    match obj {
        RpslObject::Inetnum(i) => range_to_string(&i.inetnum),
        RpslObject::Inet6num(i) => range_to_string(&i.inet6num),
        RpslObject::AutNum(a) => a.aut_num.clone(),
        RpslObject::Person(p) => p.nic_hdl.clone().unwrap_or_else(|| p.person.clone()),
        RpslObject::Role(r) => r.nic_hdl.clone().unwrap_or_else(|| r.role.clone()),
        RpslObject::Organisation(o) => o.organisation.clone(),
        RpslObject::Mntner(m) => m.mntner.clone(),
        RpslObject::Route(r) => range_to_string(&r.route),
        RpslObject::Route6(r) => range_to_string(&r.route6),
        RpslObject::Other(o) => o
            .attributes()
            .iter()
            .next()
            .map(|(k, v)| format!("{k}:{}", v.first().cloned().unwrap_or_default()))
            .unwrap_or_else(|| "other".to_string()),
    }
}
