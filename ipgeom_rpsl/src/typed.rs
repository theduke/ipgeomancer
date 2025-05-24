use crate::Object;
use ipnet::{Ipv4Net, Ipv6Net};
use iprange::IpRange;
use serde::Serialize;
use std::collections::HashMap;
use time::{Date, OffsetDateTime, PrimitiveDateTime, macros::format_description};

/// Data for an `inetnum` object
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Inetnum {
    pub inetnum: Option<IpRange<Ipv4Net>>,
    pub netname: Option<String>,
    pub descr: Option<String>,
    pub country: Option<String>,
    pub admin_c: Vec<String>,
    pub tech_c: Vec<String>,
    pub status: Option<String>,
    pub mnt_by: Vec<String>,
    pub created: Option<OffsetDateTime>,
    pub last_modified: Option<OffsetDateTime>,
    pub source: Option<String>,
    pub org: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Inet6num {
    pub inet6num: Option<IpRange<Ipv6Net>>,
    pub netname: Option<String>,
    pub descr: Option<String>,
    pub country: Option<String>,
    pub admin_c: Vec<String>,
    pub tech_c: Vec<String>,
    pub status: Option<String>,
    pub mnt_by: Vec<String>,
    pub created: Option<OffsetDateTime>,
    pub last_modified: Option<OffsetDateTime>,
    pub source: Option<String>,
    pub org: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct AutNum {
    pub aut_num: Option<String>,
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
    pub source: Option<String>,
    pub org: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Person {
    pub person: Option<String>,
    pub address: Option<String>,
    pub phone: Option<String>,
    pub fax_no: Option<String>,
    pub email: Option<String>,
    pub nic_hdl: Option<String>,
    pub mnt_by: Vec<String>,
    pub created: Option<OffsetDateTime>,
    pub last_modified: Option<OffsetDateTime>,
    pub source: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Role {
    pub role: Option<String>,
    pub address: Option<String>,
    pub phone: Option<String>,
    pub fax_no: Option<String>,
    pub email: Option<String>,
    pub admin_c: Vec<String>,
    pub tech_c: Vec<String>,
    pub nic_hdl: Option<String>,
    pub mnt_by: Vec<String>,
    pub created: Option<OffsetDateTime>,
    pub last_modified: Option<OffsetDateTime>,
    pub source: Option<String>,
    pub abuse_mailbox: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Organisation {
    pub organisation: Option<String>,
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
    pub mntner: Option<String>,
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
    pub route: Option<IpRange<Ipv4Net>>,
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
    pub route6: Option<IpRange<Ipv6Net>>,
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

/// Parse various datetime formats used in RPSL
fn parse_datetime_flexible(s: &str) -> Option<OffsetDateTime> {
    if let Ok(dt) = OffsetDateTime::parse(s, &time::format_description::well_known::Rfc3339) {
        return Some(dt);
    }
    let yyyymmdd_hhmmss = format_description!("[year][month][day] [hour repr:24][minute][second]");
    if let Ok(pdt) = PrimitiveDateTime::parse(s, &yyyymmdd_hhmmss) {
        return Some(pdt.assume_utc());
    }
    let yyyymmdd = format_description!("[year][month][day]");
    if let Ok(date) = Date::parse(s, yyyymmdd) {
        return Some(date.midnight().assume_utc());
    }
    let yyyy_mm_dd = format_description!("[year]-[month]-[day]");
    if let Ok(date) = Date::parse(s, &yyyy_mm_dd) {
        return Some(date.midnight().assume_utc());
    }
    None
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

fn pop_datetime(map: &mut HashMap<String, Vec<String>>, key: &str) -> Option<OffsetDateTime> {
    pop_single(map, key).and_then(|s| parse_datetime_flexible(&s))
}

fn parse_ipv4_range(s: &str) -> Option<IpRange<Ipv4Net>> {
    if let Ok(net) = s.trim().parse::<Ipv4Net>() {
        let mut r = IpRange::new();
        r.add(net);
        return Some(r);
    }
    None
}

fn parse_ipv6_range(s: &str) -> Option<IpRange<Ipv6Net>> {
    if let Ok(net) = s.trim().parse::<Ipv6Net>() {
        let mut r = IpRange::new();
        r.add(net);
        return Some(r);
    }
    None
}

fn pop_range4(map: &mut HashMap<String, Vec<String>>, key: &str) -> Option<IpRange<Ipv4Net>> {
    pop_single(map, key).and_then(|s| parse_ipv4_range(&s))
}

fn pop_range6(map: &mut HashMap<String, Vec<String>>, key: &str) -> Option<IpRange<Ipv6Net>> {
    pop_single(map, key).and_then(|s| parse_ipv6_range(&s))
}

fn pop_net4(map: &mut HashMap<String, Vec<String>>, key: &str) -> Option<IpRange<Ipv4Net>> {
    pop_single(map, key).and_then(|s| parse_ipv4_range(&s))
}

fn pop_net6(map: &mut HashMap<String, Vec<String>>, key: &str) -> Option<IpRange<Ipv6Net>> {
    pop_single(map, key).and_then(|s| parse_ipv6_range(&s))
}

impl TryFrom<Object> for RpslObject {
    type Error = ();

    fn try_from(obj: Object) -> Result<Self, Self::Error> {
        let mut map = obj.into_attributes();
        if map.contains_key("inetnum") {
            let res = RpslObject::Inetnum(Inetnum {
                inetnum: pop_range4(&mut map, "inetnum"),
                netname: pop_single(&mut map, "netname"),
                descr: pop_text(&mut map, "descr"),
                country: pop_single(&mut map, "country"),
                admin_c: pop_multi(&mut map, "admin-c"),
                tech_c: pop_multi(&mut map, "tech-c"),
                status: pop_single(&mut map, "status"),
                mnt_by: pop_multi(&mut map, "mnt-by"),
                created: pop_datetime(&mut map, "created"),
                last_modified: pop_datetime(&mut map, "last-modified")
                    .or_else(|| pop_datetime(&mut map, "changed")),
                source: pop_single(&mut map, "source"),
                org: pop_single(&mut map, "org"),
            });
            return Ok(res);
        }
        if map.contains_key("inet6num") {
            let res = RpslObject::Inet6num(Inet6num {
                inet6num: pop_range6(&mut map, "inet6num"),
                netname: pop_single(&mut map, "netname"),
                descr: pop_text(&mut map, "descr"),
                country: pop_single(&mut map, "country"),
                admin_c: pop_multi(&mut map, "admin-c"),
                tech_c: pop_multi(&mut map, "tech-c"),
                status: pop_single(&mut map, "status"),
                mnt_by: pop_multi(&mut map, "mnt-by"),
                created: pop_datetime(&mut map, "created"),
                last_modified: pop_datetime(&mut map, "last-modified")
                    .or_else(|| pop_datetime(&mut map, "changed")),
                source: pop_single(&mut map, "source"),
                org: pop_single(&mut map, "org"),
            });
            return Ok(res);
        }
        if map.contains_key("aut-num") {
            let res = RpslObject::AutNum(AutNum {
                aut_num: pop_single(&mut map, "aut-num"),
                as_name: pop_single(&mut map, "as-name").or_else(|| pop_single(&mut map, "asname")),
                descr: pop_text(&mut map, "descr"),
                member_of: pop_multi(&mut map, "member-of"),
                import: pop_multi(&mut map, "import"),
                export: pop_multi(&mut map, "export"),
                admin_c: pop_multi(&mut map, "admin-c"),
                tech_c: pop_multi(&mut map, "tech-c"),
                mnt_by: pop_multi(&mut map, "mnt-by"),
                created: pop_datetime(&mut map, "created"),
                last_modified: pop_datetime(&mut map, "last-modified")
                    .or_else(|| pop_datetime(&mut map, "changed")),
                source: pop_single(&mut map, "source"),
                org: pop_single(&mut map, "org"),
            });
            return Ok(res);
        }
        if map.contains_key("person") {
            let res = RpslObject::Person(Person {
                person: pop_single(&mut map, "person"),
                address: pop_text(&mut map, "address"),
                phone: pop_single(&mut map, "phone"),
                fax_no: pop_single(&mut map, "fax-no"),
                email: pop_single(&mut map, "email").or_else(|| pop_single(&mut map, "e-mail")),
                nic_hdl: pop_single(&mut map, "nic-hdl"),
                mnt_by: pop_multi(&mut map, "mnt-by"),
                created: pop_datetime(&mut map, "created")
                    .or_else(|| pop_datetime(&mut map, "changed")),
                last_modified: pop_datetime(&mut map, "last-modified")
                    .or_else(|| pop_datetime(&mut map, "changed")),
                source: pop_single(&mut map, "source"),
            });
            return Ok(res);
        }
        if map.contains_key("role") {
            let res = RpslObject::Role(Role {
                role: pop_single(&mut map, "role"),
                address: pop_text(&mut map, "address"),
                phone: pop_single(&mut map, "phone"),
                fax_no: pop_single(&mut map, "fax-no"),
                email: pop_single(&mut map, "email").or_else(|| pop_single(&mut map, "e-mail")),
                admin_c: pop_multi(&mut map, "admin-c"),
                tech_c: pop_multi(&mut map, "tech-c"),
                nic_hdl: pop_single(&mut map, "nic-hdl"),
                mnt_by: pop_multi(&mut map, "mnt-by"),
                created: pop_datetime(&mut map, "created")
                    .or_else(|| pop_datetime(&mut map, "changed")),
                last_modified: pop_datetime(&mut map, "last-modified")
                    .or_else(|| pop_datetime(&mut map, "changed")),
                source: pop_single(&mut map, "source"),
                abuse_mailbox: pop_single(&mut map, "abuse-mailbox"),
            });
            return Ok(res);
        }
        if map.contains_key("organisation") || map.contains_key("organization") {
            let res = RpslObject::Organisation(Organisation {
                organisation: pop_single(&mut map, "organisation")
                    .or_else(|| pop_single(&mut map, "organization")),
                org_name: pop_single(&mut map, "org-name")
                    .or_else(|| pop_single(&mut map, "orgname")),
                org_type: pop_single(&mut map, "org-type"),
                address: pop_text(&mut map, "address"),
                email: pop_single(&mut map, "email").or_else(|| pop_single(&mut map, "e-mail")),
                abuse_mailbox: pop_single(&mut map, "abuse-mailbox"),
                mnt_ref: pop_multi(&mut map, "mnt-ref"),
                mnt_by: pop_multi(&mut map, "mnt-by"),
                created: pop_datetime(&mut map, "created")
                    .or_else(|| pop_datetime(&mut map, "changed")),
                last_modified: pop_datetime(&mut map, "last-modified")
                    .or_else(|| pop_datetime(&mut map, "changed")),
                source: pop_single(&mut map, "source"),
            });
            return Ok(res);
        }
        if map.contains_key("mntner") {
            let res = RpslObject::Mntner(Mntner {
                mntner: pop_single(&mut map, "mntner"),
                descr: pop_text(&mut map, "descr"),
                admin_c: pop_multi(&mut map, "admin-c"),
                tech_c: pop_multi(&mut map, "tech-c"),
                upd_to: pop_multi(&mut map, "upd-to"),
                mnt_nfy: pop_multi(&mut map, "mnt-nfy"),
                auth: pop_multi(&mut map, "auth"),
                mnt_by: pop_multi(&mut map, "mnt-by"),
                created: pop_datetime(&mut map, "created")
                    .or_else(|| pop_datetime(&mut map, "changed")),
                last_modified: pop_datetime(&mut map, "last-modified")
                    .or_else(|| pop_datetime(&mut map, "changed")),
                source: pop_single(&mut map, "source"),
            });
            return Ok(res);
        }
        if map.contains_key("route") {
            let res = RpslObject::Route(Route {
                route: pop_net4(&mut map, "route"),
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
                created: pop_datetime(&mut map, "created"),
                last_modified: pop_datetime(&mut map, "last-modified")
                    .or_else(|| pop_datetime(&mut map, "changed")),
                source: pop_single(&mut map, "source"),
            });
            return Ok(res);
        }
        if map.contains_key("route6") {
            let res = RpslObject::Route6(Route6 {
                route6: pop_net6(&mut map, "route6"),
                descr: pop_text(&mut map, "descr"),
                origin: pop_single(&mut map, "origin"),
                member_of: pop_multi(&mut map, "member-of"),
                mnt_by: pop_multi(&mut map, "mnt-by"),
                created: pop_datetime(&mut map, "created"),
                last_modified: pop_datetime(&mut map, "last-modified")
                    .or_else(|| pop_datetime(&mut map, "changed")),
                source: pop_single(&mut map, "source"),
            });
            return Ok(res);
        }
        Ok(RpslObject::Other(Object::from_attributes(map)))
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
            assert_eq!(inet.inetnum, parse_ipv4_range("192.0.2.0/24"));
            assert_eq!(inet.created, Some(datetime!(2001-01-01 00:00:00 UTC)));
            assert_eq!(inet.last_modified, Some(datetime!(2002-02-02 00:00:00 UTC)));
        } else {
            panic!();
        }
    }

    #[test]
    fn convert_inet6num() {
        let data = "inet6num: 2001:db8::/32\nnetname: V6-NET\ndescr: IPv6 net\nadmin-c: AC1\nmnt-by: MAINT\ncreated: 20040101\nsource: TEST\n";
        let obj = first(data);
        if let RpslObject::Inet6num(inet) = RpslObject::try_from(obj).unwrap() {
            assert_eq!(inet.netname.as_deref(), Some("V6-NET"));
            assert_eq!(inet.descr.as_deref(), Some("IPv6 net"));
            assert_eq!(inet.inet6num, parse_ipv6_range("2001:db8::/32"));
        } else {
            panic!();
        }
    }

    #[test]
    fn convert_autnum() {
        let data = "aut-num: AS65000\nas-name: TEST-AS\ndescr: Example AS\nimport: from AS1 accept ANY\nexport: to AS1 announce ANY\nadmin-c: AC1\ntech-c: TC1\nmnt-by: MAINT\nsource: TEST\n";
        let obj = first(data);
        if let RpslObject::AutNum(aut) = RpslObject::try_from(obj).unwrap() {
            assert_eq!(aut.aut_num.as_deref(), Some("AS65000"));
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
            assert_eq!(p.person.as_deref(), Some("John Doe"));
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
            assert_eq!(r.role.as_deref(), Some("Net Admin"));
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
            assert_eq!(o.organisation.as_deref(), Some("ORG1"));
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
            assert_eq!(m.mntner.as_deref(), Some("MAINT-EXAMPLE"));
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
            assert_eq!(r.route, parse_ipv4_range("192.0.2.0/24"));
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
            assert_eq!(r.route6, parse_ipv6_range("2001:db8::/32"));
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
}
