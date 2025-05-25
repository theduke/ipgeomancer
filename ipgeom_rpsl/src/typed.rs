use crate::Object;
use anyhow::{Context, Error, anyhow, bail};
use ipnet::{Ipv4Net, Ipv6Net};
use iprange::IpRange;
use serde::Serialize;
use std::collections::HashMap;
use time::{Date, OffsetDateTime, PrimitiveDateTime, macros::format_description};

/// Data for an `inetnum` object
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Inetnum {
    pub inetnum: IpRange<Ipv4Net>,
    pub netname: Option<String>,
    pub descr: Option<String>,
    pub country: Option<String>,
    pub admin_c: Vec<String>,
    pub tech_c: Vec<String>,
    pub status: Option<String>,
    pub mnt_by: Vec<String>,
    pub created: Option<OffsetDateTime>,
    pub last_modified: Option<OffsetDateTime>,
    pub changed: Option<OffsetDateTime>,
    pub source: Option<String>,
    pub org: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Inet6num {
    pub inet6num: IpRange<Ipv6Net>,
    pub netname: Option<String>,
    pub descr: Option<String>,
    pub country: Option<String>,
    pub admin_c: Vec<String>,
    pub tech_c: Vec<String>,
    pub status: Option<String>,
    pub mnt_by: Vec<String>,
    pub created: Option<OffsetDateTime>,
    pub last_modified: Option<OffsetDateTime>,
    pub changed: Option<OffsetDateTime>,
    pub source: Option<String>,
    pub org: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct AutNum {
    pub aut_num: String,
    pub as_name: Option<String>,
    pub descr: Option<String>,
    pub member_of: Vec<String>,
    pub import: Vec<String>,
    pub export: Vec<String>,
    pub admin_c: Vec<String>,
    pub tech_c: Vec<String>,
    pub mnt_by: Vec<String>,
    pub created: Option<OffsetDateTime>,
    pub last_modified: Option<OffsetDateTime>,
    pub changed: Option<OffsetDateTime>,
    pub source: Option<String>,
    pub org: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Person {
    pub person: String,
    pub address: Option<String>,
    pub phone: Option<String>,
    pub fax_no: Option<String>,
    pub email: Option<String>,
    pub nic_hdl: Option<String>,
    pub mnt_by: Vec<String>,
    pub created: Option<OffsetDateTime>,
    pub last_modified: Option<OffsetDateTime>,
    pub changed: Option<OffsetDateTime>,
    pub source: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Role {
    pub role: String,
    pub address: Option<String>,
    pub phone: Option<String>,
    pub fax_no: Option<String>,
    pub email: Option<String>,
    pub admin_c: Vec<String>,
    pub tech_c: Vec<String>,
    pub nic_hdl: Option<String>,
    pub mnt_by: Vec<String>,
    pub created: Option<OffsetDateTime>,
    pub changed: Option<OffsetDateTime>,
    pub last_modified: Option<OffsetDateTime>,
    pub source: Option<String>,
    pub abuse_mailbox: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Organisation {
    pub organisation: String,
    pub org_name: Option<String>,
    pub org_type: Option<String>,
    pub address: Option<String>,
    pub email: Option<String>,
    pub abuse_mailbox: Option<String>,
    pub mnt_ref: Vec<String>,
    pub mnt_by: Vec<String>,
    pub created: Option<OffsetDateTime>,
    pub last_modified: Option<OffsetDateTime>,
    pub source: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Mntner {
    pub mntner: String,
    pub descr: Option<String>,
    pub admin_c: Vec<String>,
    pub tech_c: Vec<String>,
    pub upd_to: Vec<String>,
    pub mnt_nfy: Vec<String>,
    pub auth: Vec<String>,
    pub mnt_by: Vec<String>,
    pub created: Option<OffsetDateTime>,
    pub last_modified: Option<OffsetDateTime>,
    pub source: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Route {
    pub route: IpRange<Ipv4Net>,
    pub descr: Option<String>,
    pub origin: Option<String>,
    pub member_of: Vec<String>,
    pub inject: Vec<String>,
    pub aggr_mtd: Option<String>,
    pub aggr_bndry: Option<String>,
    pub export_comps: Option<String>,
    pub components: Option<String>,
    pub holes: Vec<String>,
    pub mnt_by: Vec<String>,
    pub created: Option<OffsetDateTime>,
    pub last_modified: Option<OffsetDateTime>,
    pub source: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Route6 {
    pub route6: IpRange<Ipv6Net>,
    pub descr: Option<String>,
    pub origin: Option<String>,
    pub member_of: Vec<String>,
    pub mnt_by: Vec<String>,
    pub created: Option<OffsetDateTime>,
    pub last_modified: Option<OffsetDateTime>,
    pub source: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum RpslObject {
    Inetnum(Inetnum),
    Inet6num(Inet6num),
    AutNum(AutNum),
    Person(Person),
    Role(Role),
    Organisation(Organisation),
    Mntner(Mntner),
    Route(Route),
    Route6(Route6),
    Other(Object),
}

impl RpslObject {
    /// Returns `true` if the rpsl object is [`Inetnum`].
    ///
    /// [`Inetnum`]: RpslObject::Inetnum
    #[must_use]
    pub fn is_inetnum(&self) -> bool {
        matches!(self, Self::Inetnum(..))
    }

    /// Returns `true` if the rpsl object is [`Inet6num`].
    ///
    /// [`Inet6num`]: RpslObject::Inet6num
    #[must_use]
    pub fn is_inet6num(&self) -> bool {
        matches!(self, Self::Inet6num(..))
    }
}

/// Parse various datetime formats used in RPSL
fn parse_datetime_flexible(s: &str) -> Result<OffsetDateTime, anyhow::Error> {
    fn parse_exact(s: &str) -> Result<OffsetDateTime, anyhow::Error> {
        if let Ok(dt) = OffsetDateTime::parse(s, &time::format_description::well_known::Rfc3339) {
            return Ok(dt);
        }
        let yyyymmdd_hhmmss =
            format_description!("[year][month][day] [hour repr:24][minute][second]");
        if let Ok(pdt) = PrimitiveDateTime::parse(s, &yyyymmdd_hhmmss) {
            return Ok(pdt.assume_utc());
        }
        let yyyymmdd = format_description!("[year][month][day]");
        if let Ok(date) = Date::parse(s, yyyymmdd) {
            return Ok(date.midnight().assume_utc());
        }
        let yyyy_mm_dd = format_description!("[year]-[month]-[day]");
        if let Ok(date) = Date::parse(s, &yyyy_mm_dd) {
            return Ok(date.midnight().assume_utc());
        }

        bail!("invalid datetime format: {s}");
    }

    let trimmed = s.trim();
    if let Ok(dt) = parse_exact(trimmed) {
        return Ok(dt);
    }

    if let Some(last) = trimmed.split_whitespace().last() {
        if last != trimmed {
            if let Ok(dt) = parse_exact(last) {
                return Ok(dt);
            }
        }
    }

    bail!("invalid datetime format: {s}")
}

fn pop_single(map: &mut HashMap<String, Vec<String>>, key: &str) -> Option<String> {
    map.remove(key).and_then(|mut v| {
        if v.is_empty() {
            None
        } else {
            Some(v.remove(0))
        }
    })
}

fn pop_multi(map: &mut HashMap<String, Vec<String>>, key: &str) -> Vec<String> {
    map.remove(key).unwrap_or_default()
}

fn pop_text(map: &mut HashMap<String, Vec<String>>, key: &str) -> Option<String> {
    map.remove(key).map(|v| v.join("\n"))
}

fn pop_datetime(
    map: &mut HashMap<String, Vec<String>>,
    key: &str,
) -> Result<Option<OffsetDateTime>, Error> {
    match pop_single(map, key) {
        Some(s) => Ok(parse_datetime_flexible(&s).ok()),
        None => Ok(None),
    }
}

fn parse_ipv4_range(s: &str) -> Result<IpRange<Ipv4Net>, Error> {
    let trimmed = s.trim();
    if let Ok(net) = trimmed.parse::<Ipv4Net>() {
        let mut r = IpRange::new();
        r.add(net);
        return Ok(r);
    }
    if let Some((start, end)) = trimmed.split_once('-') {
        let start = start.trim().parse::<std::net::Ipv4Addr>()?;
        let end = end.trim().parse::<std::net::Ipv4Addr>()?;
        let mut r = IpRange::new();
        let mut cur = u32::from(start) as u64;
        let end = u32::from(end) as u64;
        while cur <= end {
            let mut prefix = 32 - (cur as u32).trailing_zeros();
            loop {
                if prefix == 0 {
                    // size would overflow u32, but this can only happen when the
                    // remaining range covers the entire IPv4 space.
                    if end == u32::MAX as u64 {
                        break;
                    } else {
                        prefix += 1;
                        continue;
                    }
                }
                let size = 1u64 << (32 - prefix);
                if (cur & (size - 1)) != 0 || cur + size - 1 > end {
                    prefix += 1;
                } else {
                    break;
                }
            }
            let net = Ipv4Net::new(std::net::Ipv4Addr::from(cur as u32), prefix as u8)?;
            r.add(net);
            if prefix == 0 {
                break;
            }
            cur += 1u64 << (32 - prefix);
        }
        return Ok(r);
    }
    Err(anyhow!("invalid IPv4 range: {s}"))
}

fn parse_ipv6_range(s: &str) -> Result<IpRange<Ipv6Net>, Error> {
    let trimmed = s.trim();
    if let Ok(net) = trimmed.parse::<Ipv6Net>() {
        let mut r = IpRange::new();
        r.add(net);
        return Ok(r);
    }
    if let Some((start, end)) = trimmed.split_once('-') {
        let start = start.trim().parse::<std::net::Ipv6Addr>()?;
        let end = end.trim().parse::<std::net::Ipv6Addr>()?;
        let mut r = IpRange::new();
        let mut cur = u128::from(start);
        let end = u128::from(end);
        while cur <= end {
            let mut prefix = 128 - cur.trailing_zeros();
            loop {
                if prefix == 0 {
                    if end == u128::MAX {
                        break;
                    } else {
                        prefix += 1;
                        continue;
                    }
                }
                let size = 1u128 << (128 - prefix);
                if cur & (size - 1) != 0 || cur + size - 1 > end {
                    prefix += 1;
                } else {
                    break;
                }
            }
            let net = Ipv6Net::new(std::net::Ipv6Addr::from(cur), prefix as u8)?;
            r.add(net);
            if prefix == 0 {
                break;
            }
            cur += 1u128 << (128 - prefix);
        }
        return Ok(r);
    }
    Err(anyhow!("invalid IPv6 range: {s}"))
}

fn pop_range4(
    map: &mut HashMap<String, Vec<String>>,
    key: &str,
) -> Result<Option<IpRange<Ipv4Net>>, Error> {
    match pop_single(map, key) {
        Some(s) => Ok(Some(parse_ipv4_range(&s)?)),
        None => Ok(None),
    }
}

fn pop_range6(
    map: &mut HashMap<String, Vec<String>>,
    key: &str,
) -> Result<Option<IpRange<Ipv6Net>>, Error> {
    match pop_single(map, key) {
        Some(s) => Ok(Some(parse_ipv6_range(&s)?)),
        None => Ok(None),
    }
}

fn pop_net4(
    map: &mut HashMap<String, Vec<String>>,
    key: &str,
) -> Result<Option<IpRange<Ipv4Net>>, Error> {
    match pop_single(map, key) {
        Some(s) => Ok(Some(parse_ipv4_range(&s)?)),
        None => Ok(None),
    }
}

fn pop_net6(
    map: &mut HashMap<String, Vec<String>>,
    key: &str,
) -> Result<Option<IpRange<Ipv6Net>>, Error> {
    match pop_single(map, key) {
        Some(s) => Ok(Some(parse_ipv6_range(&s)?)),
        None => Ok(None),
    }
}

impl TryFrom<Object> for RpslObject {
    type Error = anyhow::Error;

    fn try_from(obj: Object) -> Result<Self, Self::Error> {
        let obj_type = obj.obj_type().clone();
        let mut map = obj.into_attributes();
        match obj_type {
            crate::ObjectType::Inetnum => {
                let inetnum = pop_range4(&mut map, "inetnum")?.context("missing inetnum range")?;
                let res = RpslObject::Inetnum(Inetnum {
                    inetnum,
                    netname: pop_single(&mut map, "netname"),
                    descr: pop_text(&mut map, "descr"),
                    country: pop_single(&mut map, "country"),
                    admin_c: pop_multi(&mut map, "admin-c"),
                    tech_c: pop_multi(&mut map, "tech-c"),
                    status: pop_single(&mut map, "status"),
                    mnt_by: pop_multi(&mut map, "mnt-by"),
                    created: pop_datetime(&mut map, "created")?,
                    last_modified: pop_datetime(&mut map, "last-modified")?,
                    changed: pop_datetime(&mut map, "changed")?,
                    source: pop_single(&mut map, "source"),
                    org: pop_single(&mut map, "org"),
                });
                Ok(res)
            }
            crate::ObjectType::Inet6num => {
                let inet6num =
                    pop_range6(&mut map, "inet6num")?.context("missing inet6num range")?;
                let res = RpslObject::Inet6num(Inet6num {
                    inet6num,
                    netname: pop_single(&mut map, "netname"),
                    descr: pop_text(&mut map, "descr"),
                    country: pop_single(&mut map, "country"),
                    admin_c: pop_multi(&mut map, "admin-c"),
                    tech_c: pop_multi(&mut map, "tech-c"),
                    status: pop_single(&mut map, "status"),
                    mnt_by: pop_multi(&mut map, "mnt-by"),
                    created: pop_datetime(&mut map, "created")?,
                    last_modified: pop_datetime(&mut map, "last-modified")?,
                    changed: pop_datetime(&mut map, "changed")?,
                    source: pop_single(&mut map, "source"),
                    org: pop_single(&mut map, "org"),
                });
                Ok(res)
            }
            crate::ObjectType::AutNum => {
                let aut_num = pop_single(&mut map, "aut-num").context("missing aut-num")?;
                let res = RpslObject::AutNum(AutNum {
                    aut_num,
                    as_name: pop_single(&mut map, "as-name")
                        .or_else(|| pop_single(&mut map, "asname")),
                    descr: pop_text(&mut map, "descr"),
                    member_of: pop_multi(&mut map, "member-of"),
                    import: pop_multi(&mut map, "import"),
                    export: pop_multi(&mut map, "export"),
                    admin_c: pop_multi(&mut map, "admin-c"),
                    tech_c: pop_multi(&mut map, "tech-c"),
                    mnt_by: pop_multi(&mut map, "mnt-by"),
                    created: pop_datetime(&mut map, "created")?,
                    last_modified: pop_datetime(&mut map, "last-modified")?,
                    changed: pop_datetime(&mut map, "changed")?,
                    source: pop_single(&mut map, "source"),
                    org: pop_single(&mut map, "org"),
                });
                Ok(res)
            }
            crate::ObjectType::Person => {
                let person = pop_single(&mut map, "person").context("missing person")?;
                let res = RpslObject::Person(Person {
                    person,
                    address: pop_text(&mut map, "address"),
                    phone: pop_single(&mut map, "phone"),
                    fax_no: pop_single(&mut map, "fax-no"),
                    email: pop_single(&mut map, "email").or_else(|| pop_single(&mut map, "e-mail")),
                    nic_hdl: pop_single(&mut map, "nic-hdl"),
                    mnt_by: pop_multi(&mut map, "mnt-by"),
                    created: pop_datetime(&mut map, "created")?,
                    last_modified: pop_datetime(&mut map, "last-modified")?,
                    changed: pop_datetime(&mut map, "changed")?,
                    source: pop_single(&mut map, "source"),
                });
                Ok(res)
            }
            crate::ObjectType::Role => {
                let role = pop_single(&mut map, "role").context("missing role")?;
                let res = RpslObject::Role(Role {
                    role,
                    address: pop_text(&mut map, "address"),
                    phone: pop_single(&mut map, "phone"),
                    fax_no: pop_single(&mut map, "fax-no"),
                    email: pop_single(&mut map, "email").or_else(|| pop_single(&mut map, "e-mail")),
                    admin_c: pop_multi(&mut map, "admin-c"),
                    tech_c: pop_multi(&mut map, "tech-c"),
                    nic_hdl: pop_single(&mut map, "nic-hdl"),
                    mnt_by: pop_multi(&mut map, "mnt-by"),
                    created: pop_datetime(&mut map, "created")?,
                    last_modified: pop_datetime(&mut map, "last-modified")?,
                    changed: pop_datetime(&mut map, "changed")?,
                    source: pop_single(&mut map, "source"),
                    abuse_mailbox: pop_single(&mut map, "abuse-mailbox"),
                });
                Ok(res)
            }
            crate::ObjectType::Organisation => {
                let organisation = pop_single(&mut map, "organisation")
                    .or_else(|| pop_single(&mut map, "organization"))
                    .context("missing organisation/organization")?;
                let res = RpslObject::Organisation(Organisation {
                    organisation,
                    org_name: pop_single(&mut map, "org-name")
                        .or_else(|| pop_single(&mut map, "orgname")),
                    org_type: pop_single(&mut map, "org-type"),
                    address: pop_text(&mut map, "address"),
                    email: pop_single(&mut map, "email").or_else(|| pop_single(&mut map, "e-mail")),
                    abuse_mailbox: pop_single(&mut map, "abuse-mailbox"),
                    mnt_ref: pop_multi(&mut map, "mnt-ref"),
                    mnt_by: pop_multi(&mut map, "mnt-by"),
                    created: pop_datetime(&mut map, "created")?,
                    last_modified: pop_datetime(&mut map, "last-modified")?,
                    source: pop_single(&mut map, "source"),
                });
                Ok(res)
            }
            crate::ObjectType::Mntner => {
                let mntner = pop_single(&mut map, "mntner").context("missing mntner")?;
                let res = RpslObject::Mntner(Mntner {
                    mntner,
                    descr: pop_text(&mut map, "descr"),
                    admin_c: pop_multi(&mut map, "admin-c"),
                    tech_c: pop_multi(&mut map, "tech-c"),
                    upd_to: pop_multi(&mut map, "upd-to"),
                    mnt_nfy: pop_multi(&mut map, "mnt-nfy"),
                    auth: pop_multi(&mut map, "auth"),
                    mnt_by: pop_multi(&mut map, "mnt-by"),
                    created: pop_datetime(&mut map, "created")?,
                    last_modified: pop_datetime(&mut map, "last-modified")?,
                    source: pop_single(&mut map, "source"),
                });
                Ok(res)
            }
            crate::ObjectType::Route => {
                let route = pop_net4(&mut map, "route")?.context("missing route")?;
                let res = RpslObject::Route(Route {
                    route,
                    descr: pop_text(&mut map, "descr"),
                    origin: pop_single(&mut map, "origin"),
                    member_of: pop_multi(&mut map, "member-of"),
                    inject: pop_multi(&mut map, "inject"),
                    aggr_mtd: pop_single(&mut map, "aggr-mtd"),
                    aggr_bndry: pop_single(&mut map, "aggr-bndry"),
                    export_comps: pop_single(&mut map, "export-comps"),
                    components: pop_single(&mut map, "components"),
                    holes: pop_multi(&mut map, "holes"),
                    mnt_by: pop_multi(&mut map, "mnt-by"),
                    created: pop_datetime(&mut map, "created")?,
                    last_modified: pop_datetime(&mut map, "last-modified")?,
                    source: pop_single(&mut map, "source"),
                });
                Ok(res)
            }
            crate::ObjectType::Route6 => {
                let route6 = pop_net6(&mut map, "route6")?.context("missing route6")?;
                let res = RpslObject::Route6(Route6 {
                    route6,
                    descr: pop_text(&mut map, "descr"),
                    origin: pop_single(&mut map, "origin"),
                    member_of: pop_multi(&mut map, "member-of"),
                    mnt_by: pop_multi(&mut map, "mnt-by"),
                    created: pop_datetime(&mut map, "created")?,
                    last_modified: pop_datetime(&mut map, "last-modified")?,
                    source: pop_single(&mut map, "source"),
                });
                Ok(res)
            }
            crate::ObjectType::Other(name) => Ok(RpslObject::Other(Object::from_attributes(
                crate::ObjectType::Other(name),
                map,
            ))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parse_objects;
    use time::macros::datetime;

    fn first(obj_text: &str) -> Object {
        parse_objects(obj_text).unwrap().remove(0)
    }

    #[test]
    fn convert_inetnum() {
        let data = "inetnum: 192.0.2.0/24\nnetname: TEST-NET\ndescr: Example\nadmin-c: AC1\nmnt-by: MAINT\ncreated: 20010101\nlast-modified: 20020202\nsource: TEST\n";
        let obj = first(data);
        if let RpslObject::Inetnum(inet) = RpslObject::try_from(obj).unwrap() {
            assert_eq!(inet.netname.as_deref(), Some("TEST-NET"));
            assert_eq!(inet.descr.as_deref(), Some("Example"));
            assert_eq!(inet.admin_c, vec!["AC1"]);
            assert_eq!(inet.mnt_by, vec!["MAINT"]);
            assert_eq!(inet.inetnum, parse_ipv4_range("192.0.2.0/24").unwrap());
            assert_eq!(inet.created, Some(datetime!(2001-01-01 00:00:00 UTC)));
            assert_eq!(inet.last_modified, Some(datetime!(2002-02-02 00:00:00 UTC)));
        } else {
            panic!();
        }
    }

    #[test]
    fn convert_inetnum_range() {
        let data = "inetnum: 193.194.160.0 - 193.194.191.255\n";
        let obj = first(data);
        if let RpslObject::Inetnum(inet) = RpslObject::try_from(obj).unwrap() {
            assert_eq!(
                inet.inetnum,
                parse_ipv4_range("193.194.160.0 - 193.194.191.255").unwrap()
            );
        } else {
            panic!();
        }
    }

    #[test]
    fn parse_ipv4_range_full() {
        let range = parse_ipv4_range("0.0.0.0 - 255.255.255.255").unwrap();
        let mut expected = IpRange::new();
        expected.add(Ipv4Net::new("0.0.0.0".parse().unwrap(), 0).unwrap());
        assert_eq!(range, expected);
    }

    #[test]
    fn parse_ipv6_range_full() {
        let range = parse_ipv6_range(":: - ffff:ffff:ffff:ffff:ffff:ffff:ffff:ffff").unwrap();
        let mut expected = IpRange::new();
        expected.add(Ipv6Net::new("::".parse().unwrap(), 0).unwrap());
        assert_eq!(range, expected);
    }

    #[test]
    fn convert_inet6num() {
        let data = "inet6num: 2001:db8::/32\nnetname: V6-NET\ndescr: IPv6 net\nadmin-c: AC1\nmnt-by: MAINT\ncreated: 20040101\nsource: TEST\n";
        let obj = first(data);
        if let RpslObject::Inet6num(inet) = RpslObject::try_from(obj).unwrap() {
            assert_eq!(inet.netname.as_deref(), Some("V6-NET"));
            assert_eq!(inet.descr.as_deref(), Some("IPv6 net"));
            assert_eq!(inet.inet6num, parse_ipv6_range("2001:db8::/32").unwrap());
        } else {
            panic!();
        }
    }

    #[test]
    fn convert_autnum() {
        let data = "aut-num: AS65000\nas-name: TEST-AS\ndescr: Example AS\nimport: from AS1 accept ANY\nexport: to AS1 announce ANY\nadmin-c: AC1\ntech-c: TC1\nmnt-by: MAINT\nsource: TEST\n";
        let obj = first(data);
        if let RpslObject::AutNum(aut) = RpslObject::try_from(obj).unwrap() {
            assert_eq!(aut.aut_num, "AS65000");
            assert_eq!(aut.as_name.as_deref(), Some("TEST-AS"));
            assert_eq!(aut.import, vec!["from AS1 accept ANY"]);
        } else {
            panic!();
        }
    }

    #[test]
    fn convert_person() {
        let data = "person: John Doe\naddress: 1 Main St\naddress: Town\nphone: +1 555 123\nemail: john@example.com\nnic-hdl: JD1\nmnt-by: MAINT\nchanged: 20200101\nsource: TEST\n";
        let obj = first(data);
        if let RpslObject::Person(p) = RpslObject::try_from(obj).unwrap() {
            assert_eq!(p.person, "John Doe");
            assert_eq!(p.address.as_deref(), Some("1 Main St\nTown"));
            assert_eq!(p.mnt_by, vec!["MAINT"]);
        } else {
            panic!();
        }
    }

    #[test]
    fn convert_role() {
        let data = "role: Net Admin\naddress: 1 Admin St\nphone: +1 555 000\nadmin-c: AC1\ntech-c: TC1\nnic-hdl: NA1\nmnt-by: MAINT\nabuse-mailbox: abuse@example.com\nsource: TEST\n";
        let obj = first(data);
        if let RpslObject::Role(r) = RpslObject::try_from(obj).unwrap() {
            assert_eq!(r.role, "Net Admin");
            assert_eq!(r.abuse_mailbox.as_deref(), Some("abuse@example.com"));
        } else {
            panic!();
        }
    }

    #[test]
    fn convert_organisation() {
        let data = "organisation: ORG1\norg-name: Example Org\norg-type: OTHER\naddress: 1 Org St\nemail: org@example.com\nmnt-ref: MAINT\nmnt-by: MAINT\nsource: TEST\n";
        let obj = first(data);
        if let RpslObject::Organisation(o) = RpslObject::try_from(obj).unwrap() {
            assert_eq!(o.organisation, "ORG1");
            assert_eq!(o.mnt_by, vec!["MAINT"]);
        } else {
            panic!();
        }
    }

    #[test]
    fn convert_mntner() {
        let data = "mntner: MAINT-EXAMPLE\ndescr: Maintainer\nadmin-c: AC1\ntech-c: TC1\nupd-to: upd@example.com\nauth: PASSWORD\nmnt-by: MAINT-EXAMPLE\nchanged: 20200101\nsource: TEST\n";
        let obj = first(data);
        if let RpslObject::Mntner(m) = RpslObject::try_from(obj).unwrap() {
            assert_eq!(m.mntner, "MAINT-EXAMPLE");
            assert_eq!(m.descr.as_deref(), Some("Maintainer"));
            assert_eq!(m.auth, vec!["PASSWORD"]);
        } else {
            panic!();
        }
    }

    #[test]
    fn convert_route() {
        let data = "route: 192.0.2.0/24\norigin: AS65000\nmnt-by: MAINT\nsource: TEST\n";
        let obj = first(data);
        if let RpslObject::Route(r) = RpslObject::try_from(obj).unwrap() {
            assert_eq!(r.route, parse_ipv4_range("192.0.2.0/24").unwrap());
            assert_eq!(r.origin.as_deref(), Some("AS65000"));
        } else {
            panic!();
        }
    }

    #[test]
    fn convert_route6() {
        let data = "route6: 2001:db8::/32\norigin: AS65000\nmnt-by: MAINT\nsource: TEST\n";
        let obj = first(data);
        if let RpslObject::Route6(r) = RpslObject::try_from(obj).unwrap() {
            assert_eq!(r.route6, parse_ipv6_range("2001:db8::/32").unwrap());
        } else {
            panic!();
        }
    }

    #[test]
    fn convert_other() {
        let data = "poem: The Raven\nline: Once upon a midnight dreary\n";
        let obj = first(data);
        match RpslObject::try_from(obj).unwrap() {
            RpslObject::Other(o) => {
                assert!(o.get("poem").is_some());
            }
            _ => panic!(),
        }
    }

    #[test]
    fn parse_changed_with_prefix() {
        let data = "person: John\nchanged: ripe-dbm@ripe.net 20040521\nsource: TEST\n";
        let obj = first(data);
        if let RpslObject::Person(p) = RpslObject::try_from(obj).unwrap() {
            assert_eq!(p.changed, Some(datetime!(2004-05-21 00:00:00 UTC)));
            assert_eq!(p.last_modified, None);
        } else {
            panic!();
        }
    }

    #[test]
    fn parse_datetime_formats() {
        let dt = parse_datetime_flexible("2020-01-02T03:04:05Z").unwrap();
        assert_eq!(dt, datetime!(2020-01-02 03:04:05 UTC));

        let dt = parse_datetime_flexible("20200102 030405").unwrap();
        assert_eq!(dt, datetime!(2020-01-02 03:04:05 UTC));

        let dt = parse_datetime_flexible("20200102").unwrap();
        assert_eq!(dt, datetime!(2020-01-02 00:00:00 UTC));

        let dt = parse_datetime_flexible("2020-01-02").unwrap();
        assert_eq!(dt, datetime!(2020-01-02 00:00:00 UTC));

        assert!(parse_datetime_flexible("not a date").is_err());
    }

    #[test]
    fn invalid_ip_ranges() {
        let r = parse_ipv4_range("10.0.0.1 - 10.0.0.0").unwrap();
        assert!(r.iter().next().is_none());
        assert!(parse_ipv4_range("bogus").is_err());

        let r = parse_ipv6_range("ffff:: - ::").unwrap();
        assert!(r.iter().next().is_none());
        assert!(parse_ipv6_range("invalid").is_err());
    }
}
