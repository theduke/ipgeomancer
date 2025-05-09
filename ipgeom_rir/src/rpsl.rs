use ipnet::{Ipv4Net, Ipv6Net};
use iprange::{IpNet, IpRange};
use std::collections::HashMap;
use time::{Date, OffsetDateTime, PrimitiveDateTime, macros::format_description};

// --- Data Structures (Enums and Structs) ---

#[derive(Debug, Clone, PartialEq)]
pub struct InetnumData {
    pub inetnum: Option<IpRange<Ipv4Net>>,
    pub netname: Option<String>,
    pub descr: Vec<String>,
    pub country: Option<String>,
    pub admin_c: Vec<String>,
    pub tech_c: Vec<String>,
    pub status: Option<String>,
    pub mnt_by: Vec<String>,
    pub created: Option<OffsetDateTime>,
    pub last_modified: Option<OffsetDateTime>,
    pub source: Option<String>,
    pub org: Option<String>,
    pub extra: HashMap<String, String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Inet6numData {
    pub inet6num: Option<IpRange<Ipv6Net>>,
    pub netname: Option<String>,
    pub descr: Vec<String>,
    pub country: Option<String>,
    pub admin_c: Vec<String>,
    pub tech_c: Vec<String>,
    pub status: Option<String>,
    pub mnt_by: Vec<String>,
    pub created: Option<OffsetDateTime>,
    pub last_modified: Option<OffsetDateTime>,
    pub source: Option<String>,
    pub org: Option<String>,
    pub extra: HashMap<String, String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AutNumData {
    pub aut_num: Option<String>,
    pub as_name: Option<String>,
    pub descr: Vec<String>,
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
    pub extra: HashMap<String, String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PersonData {
    pub person: Option<String>,
    pub address: Vec<String>,
    pub phone: Option<String>,
    pub fax_no: Option<String>,
    pub email: Option<String>,
    pub nic_hdl: Option<String>,
    pub mnt_by: Vec<String>,
    pub created: Option<OffsetDateTime>,
    pub last_modified: Option<OffsetDateTime>,
    pub source: Option<String>,
    pub extra: HashMap<String, String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RoleData {
    pub role: Option<String>,
    pub address: Vec<String>,
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
    pub extra: HashMap<String, String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct OrganisationData {
    pub organisation: Option<String>,
    pub org_name: Option<String>,
    pub org_type: Option<String>,
    pub address: Vec<String>,
    pub email: Option<String>,
    pub abuse_mailbox: Option<String>,
    pub mnt_ref: Vec<String>,
    pub mnt_by: Vec<String>,
    pub created: Option<OffsetDateTime>,
    pub last_modified: Option<OffsetDateTime>,
    pub source: Option<String>,
    pub extra: HashMap<String, String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MntnerData {
    pub mntner: Option<String>,
    pub descr: Vec<String>,
    pub admin_c: Vec<String>,
    pub tech_c: Vec<String>,
    pub upd_to: Vec<String>,
    pub mnt_nfy: Vec<String>,
    pub auth: Vec<String>,
    pub mnt_by: Vec<String>,
    pub created: Option<OffsetDateTime>,
    pub last_modified: Option<OffsetDateTime>,
    pub source: Option<String>,
    pub extra: HashMap<String, String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RouteData {
    pub route: Option<IpRange<Ipv4Net>>,
    pub descr: Vec<String>,
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
    pub extra: HashMap<String, String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Route6Data {
    pub route6: Option<IpRange<Ipv6Net>>,
    pub descr: Vec<String>,
    pub origin: Option<String>,
    pub member_of: Vec<String>,
    pub mnt_by: Vec<String>,
    pub created: Option<OffsetDateTime>,
    pub last_modified: Option<OffsetDateTime>,
    pub source: Option<String>,
    pub extra: HashMap<String, String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum RpslObject {
    Inetnum(InetnumData),
    Inet6num(Inet6numData),
    AutNum(AutNumData),
    Person(PersonData),
    Role(RoleData),
    Organisation(OrganisationData),
    Mntner(MntnerData),
    Route(RouteData),
    Route6(Route6Data),
    Other {
        obj_type: String,
        attributes: HashMap<String, String>,
    },
}

// --- Helper functions for parsing values ---

fn parse_datetime_flexible(s: &str) -> Option<OffsetDateTime> {
    // Try RFC3339 (e.g., 2023-10-26T10:20:30Z or 2023-10-26T10:20:30.123+02:00)
    if let Ok(dt) = OffsetDateTime::parse(s, &time::format_description::well_known::Rfc3339) {
        return Some(dt);
    }
    // Try "YYYY-MM-DD HH:MM:SS Z" (ARIN style, Z might be missing)
    // Note: `time` crate's OffsetDateTime::parse needs a specific offset like 'Z' or '+HH:MM'
    // For simplicity, if it looks like this pattern without Z, we might assume UTC or try other patterns.

    // Try YYYYMMDD HHMMSS (often without timezone)
    let yyyymmdd_hhmmss_format =
        format_description!("[year][month][day] [hour repr:24][minute][second]");
    if let Ok(pdt) = PrimitiveDateTime::parse(s, &yyyymmdd_hhmmss_format) {
        return Some(pdt.assume_utc());
    }
    // Try YYYYMMDD (common for 'changed', 'created', 'last-modified' in older RIPE formats)
    let yyyymmdd_format = format_description!("[year][month][day]");
    if let Ok(date) = Date::parse(s, yyyymmdd_format) {
        return Some(date.midnight().assume_utc());
    }
    // Try "YYYY-MM-DD"
    let yyyy_mm_dd_format = format_description!("[year]-[month]-[day]");
    if let Ok(date) = Date::parse(s, &yyyy_mm_dd_format) {
        return Some(date.midnight().assume_utc());
    }
    None
}

// Pops the first value for a key, if present. Removes the key from the map.
fn pop_string(map: &mut HashMap<String, Vec<String>>, key: &str) -> Option<String> {
    map.remove(key).and_then(|mut v| {
        if v.is_empty() {
            None
        } else {
            Some(v.remove(0))
        }
    })
}

// Pops all values for a key, if present. Removes the key from the map.
fn pop_vec_string(map: &mut HashMap<String, Vec<String>>, key: &str) -> Vec<String> {
    map.remove(key).unwrap_or_default()
}

fn pop_datetime(map: &mut HashMap<String, Vec<String>>, key: &str) -> Option<OffsetDateTime> {
    pop_string(map, key).and_then(|s| parse_datetime_flexible(&s))
}

fn pop_ip4_range(map: &mut HashMap<String, Vec<String>>, key: &str) -> Option<IpRange<Ipv4Net>> {
    pop_string(map, key).and_then(|s| s.parse().ok())
}

fn pop_ip6_range(map: &mut HashMap<String, Vec<String>>, key: &str) -> Option<IpRange<Ipv6Net>> {
    pop_string(map, key).and_then(|s| s.parse().ok())
}

fn pop_ip4_net(map: &mut HashMap<String, Vec<String>>, key: &str) -> Option<IpRange<Ipv4Net>> {
    pop_string(map, key).and_then(|s| s.parse().ok())
}

fn pop_ip6_net(map: &mut HashMap<String, Vec<String>>, key: &str) -> Option<IpRange<Ipv6Net>> {
    pop_string(map, key).and_then(|s| s.parse().ok())
}

// --- Main Parser Function ---

pub fn parse_rpsl(rpsl_text: &str) -> Vec<RpslObject> {
    let mut objects: Vec<RpslObject> = Vec::new();
    let mut current_object_attributes: HashMap<String, Vec<String>> = HashMap::new();
    let mut current_attribute_key: Option<String> = None;
    let mut first_attribute_key_in_object: Option<String> = None;

    for line in rpsl_text.lines() {
        let original_line = line;
        let trimmed_line = line.trim();

        if trimmed_line.is_empty() {
            if !current_object_attributes.is_empty() {
                if let Some(obj_type) = &first_attribute_key_in_object {
                    let obj = build_rpsl_object(obj_type, &mut current_object_attributes);
                    objects.push(obj);
                }
                current_object_attributes.clear();
                current_attribute_key = None;
                first_attribute_key_in_object = None;
            }
            continue;
        }

        if trimmed_line.starts_with('#') || trimmed_line.starts_with('%') {
            continue;
        }

        if original_line.starts_with(|c: char| c.is_whitespace()) && current_attribute_key.is_some()
        {
            if let Some(key) = current_attribute_key {
                if let Some(values) = current_object_attributes.get_mut(&key) {
                    if let Some(last_value) = values.last_mut() {
                        if !last_value.is_empty()
                            && !last_value.ends_with(' ')
                            && !trimmed_line.is_empty()
                        {
                            last_value.push(' ');
                        }
                        last_value.push_str(trimmed_line);
                    } else {
                        values.push(trimmed_line.to_string());
                    }
                }
            }
        } else {
            if let Some(colon_pos) = trimmed_line.find(':') {
                let key_str = trimmed_line[..colon_pos].trim().to_lowercase();
                let value_str = trimmed_line[colon_pos + 1..].trim().to_string();

                if first_attribute_key_in_object.is_none() {
                    first_attribute_key_in_object = Some(key_str.clone());
                }
                current_attribute_key = Some(key_str.clone());

                current_object_attributes
                    .entry(key_str)
                    .or_insert_with(Vec::new)
                    .push(value_str);
            } else {
                // Line has no colon and doesn't start with whitespace.
                // Attempt to append to current_attribute_key if one exists (implicit continuation).
                if let Some(key) = &current_attribute_key {
                    if let Some(values) = current_object_attributes.get_mut(key) {
                        if let Some(last_value) = values.last_mut() {
                            if !last_value.is_empty()
                                && !last_value.ends_with(' ')
                                && !trimmed_line.is_empty()
                            {
                                last_value.push(' ');
                            }
                            last_value.push_str(trimmed_line);
                        } else {
                            values.push(trimmed_line.to_string());
                        }
                    }
                } else {
                    // eprintln!("Skipping malformed/unassociated line: {}", trimmed_line);
                }
            }
        }
    }

    if !current_object_attributes.is_empty() {
        if let Some(obj_type) = &first_attribute_key_in_object {
            let obj = build_rpsl_object(obj_type, &mut current_object_attributes);
            objects.push(obj);
        }
    }

    objects
}

fn build_rpsl_object(
    obj_type_key: &str, // This is the already lowercased key like "inetnum"
    attributes: &mut HashMap<String, Vec<String>>, // Contains lowercased keys
) -> RpslObject {
    match obj_type_key {
        "inetnum" => RpslObject::Inetnum(InetnumData {
            inetnum: pop_ip4_range(attributes, "inetnum"),
            netname: pop_string(attributes, "netname"),
            descr: pop_vec_string(attributes, "descr"),
            country: pop_string(attributes, "country"),
            admin_c: pop_vec_string(attributes, "admin-c"),
            tech_c: pop_vec_string(attributes, "tech-c"),
            status: pop_string(attributes, "status"),
            mnt_by: pop_vec_string(attributes, "mnt-by"),
            created: pop_datetime(attributes, "created"),
            last_modified: pop_datetime(attributes, "last-modified")
                .or_else(|| pop_datetime(attributes, "changed")),
            source: pop_string(attributes, "source"),
            org: pop_string(attributes, "org"),
            extra: attributes
                .iter()
                .map(|(k, v)| (k.clone(), v.join("\n")))
                .collect(),
        }),
        "inet6num" => RpslObject::Inet6num(Inet6numData {
            inet6num: pop_ip6_range(attributes, "inet6num"),
            netname: pop_string(attributes, "netname"),
            descr: pop_vec_string(attributes, "descr"),
            country: pop_string(attributes, "country"),
            admin_c: pop_vec_string(attributes, "admin-c"),
            tech_c: pop_vec_string(attributes, "tech-c"),
            status: pop_string(attributes, "status"),
            mnt_by: pop_vec_string(attributes, "mnt-by"),
            created: pop_datetime(attributes, "created"),
            last_modified: pop_datetime(attributes, "last-modified")
                .or_else(|| pop_datetime(attributes, "changed")),
            source: pop_string(attributes, "source"),
            org: pop_string(attributes, "org"),
            extra: attributes
                .iter()
                .map(|(k, v)| (k.clone(), v.join("\n")))
                .collect(),
        }),
        "aut-num" => RpslObject::AutNum(AutNumData {
            aut_num: pop_string(attributes, "aut-num"),
            as_name: pop_string(attributes, "as-name").or_else(|| pop_string(attributes, "asname")),
            descr: pop_vec_string(attributes, "descr"),
            member_of: pop_vec_string(attributes, "member-of"),
            import: pop_vec_string(attributes, "import"),
            export: pop_vec_string(attributes, "export"),
            admin_c: pop_vec_string(attributes, "admin-c"),
            tech_c: pop_vec_string(attributes, "tech-c"),
            mnt_by: pop_vec_string(attributes, "mnt-by"),
            created: pop_datetime(attributes, "created"),
            last_modified: pop_datetime(attributes, "last-modified")
                .or_else(|| pop_datetime(attributes, "changed")),
            source: pop_string(attributes, "source"),
            org: pop_string(attributes, "org"),
            extra: attributes
                .iter()
                .map(|(k, v)| (k.clone(), v.join("\n")))
                .collect(),
        }),
        "person" => RpslObject::Person(PersonData {
            person: pop_string(attributes, "person"),
            address: pop_vec_string(attributes, "address"),
            phone: pop_string(attributes, "phone"),
            fax_no: pop_string(attributes, "fax-no"),
            email: pop_string(attributes, "email").or_else(|| pop_string(attributes, "e-mail")),
            nic_hdl: pop_string(attributes, "nic-hdl"),
            mnt_by: pop_vec_string(attributes, "mnt-by"),
            created: pop_datetime(attributes, "created")
                .or_else(|| pop_datetime(attributes, "changed")),
            last_modified: pop_datetime(attributes, "last-modified")
                .or_else(|| pop_datetime(attributes, "changed")),
            source: pop_string(attributes, "source"),
            extra: attributes
                .iter()
                .map(|(k, v)| (k.clone(), v.join("\n")))
                .collect(),
        }),
        "role" => RpslObject::Role(RoleData {
            role: pop_string(attributes, "role"),
            address: pop_vec_string(attributes, "address"),
            phone: pop_string(attributes, "phone"),
            fax_no: pop_string(attributes, "fax-no"),
            email: pop_string(attributes, "email").or_else(|| pop_string(attributes, "e-mail")),
            admin_c: pop_vec_string(attributes, "admin-c"),
            tech_c: pop_vec_string(attributes, "tech-c"),
            nic_hdl: pop_string(attributes, "nic-hdl"),
            mnt_by: pop_vec_string(attributes, "mnt-by"),
            created: pop_datetime(attributes, "created")
                .or_else(|| pop_datetime(attributes, "changed")),
            last_modified: pop_datetime(attributes, "last-modified")
                .or_else(|| pop_datetime(attributes, "changed")),
            source: pop_string(attributes, "source"),
            abuse_mailbox: pop_string(attributes, "abuse-mailbox"),
            extra: attributes
                .iter()
                .map(|(k, v)| (k.clone(), v.join("\n")))
                .collect(),
        }),
        "organisation" | "organization" => RpslObject::Organisation(OrganisationData {
            organisation: pop_string(attributes, "organisation")
                .or_else(|| pop_string(attributes, "organization")),
            org_name: pop_string(attributes, "org-name")
                .or_else(|| pop_string(attributes, "orgname")),
            org_type: pop_string(attributes, "org-type"),
            address: pop_vec_string(attributes, "address"),
            email: pop_string(attributes, "email").or_else(|| pop_string(attributes, "e-mail")),
            abuse_mailbox: pop_string(attributes, "abuse-mailbox"),
            mnt_ref: pop_vec_string(attributes, "mnt-ref"),
            mnt_by: pop_vec_string(attributes, "mnt-by"),
            created: pop_datetime(attributes, "created")
                .or_else(|| pop_datetime(attributes, "changed")),
            last_modified: pop_datetime(attributes, "last-modified")
                .or_else(|| pop_datetime(attributes, "changed")),
            source: pop_string(attributes, "source"),
            extra: attributes
                .iter()
                .map(|(k, v)| (k.clone(), v.join("\n")))
                .collect(),
        }),
        "mntner" => RpslObject::Mntner(MntnerData {
            mntner: pop_string(attributes, "mntner"),
            descr: pop_vec_string(attributes, "descr"),
            admin_c: pop_vec_string(attributes, "admin-c"),
            tech_c: pop_vec_string(attributes, "tech-c"),
            upd_to: pop_vec_string(attributes, "upd-to"),
            mnt_nfy: pop_vec_string(attributes, "mnt-nfy"),
            auth: pop_vec_string(attributes, "auth"),
            mnt_by: pop_vec_string(attributes, "mnt-by"),
            created: pop_datetime(attributes, "created")
                .or_else(|| pop_datetime(attributes, "changed")),
            last_modified: pop_datetime(attributes, "last-modified")
                .or_else(|| pop_datetime(attributes, "changed")),
            source: pop_string(attributes, "source"),
            extra: attributes
                .iter()
                .map(|(k, v)| (k.clone(), v.join("\n")))
                .collect(),
        }),
        "route" => RpslObject::Route(RouteData {
            route: pop_ip4_net(attributes, "route"),
            descr: pop_vec_string(attributes, "descr"),
            origin: pop_string(attributes, "origin"),
            member_of: pop_vec_string(attributes, "member-of"),
            inject: pop_vec_string(attributes, "inject"),
            aggr_mtd: pop_string(attributes, "aggr-mtd"),
            aggr_bndry: pop_string(attributes, "aggr-bndry"),
            export_comps: pop_string(attributes, "export-comps"),
            components: pop_string(attributes, "components"),
            holes: pop_vec_string(attributes, "holes"),
            mnt_by: pop_vec_string(attributes, "mnt-by"),
            created: pop_datetime(attributes, "created"),
            last_modified: pop_datetime(attributes, "last-modified")
                .or_else(|| pop_datetime(attributes, "changed")),
            source: pop_string(attributes, "source"),
            extra: attributes
                .iter()
                .map(|(k, v)| (k.clone(), v.join("\n")))
                .collect(),
        }),
        "route6" => RpslObject::Route6(Route6Data {
            route6: pop_ip6_net(attributes, "route6"),
            descr: pop_vec_string(attributes, "descr"),
            origin: pop_string(attributes, "origin"),
            member_of: pop_vec_string(attributes, "member-of"),
            mnt_by: pop_vec_string(attributes, "mnt-by"),
            created: pop_datetime(attributes, "created"),
            last_modified: pop_datetime(attributes, "last-modified")
                .or_else(|| pop_datetime(attributes, "changed")),
            source: pop_string(attributes, "source"),
            extra: attributes
                .iter()
                .map(|(k, v)| (k.clone(), v.join("\n")))
                .collect(),
        }),
        _ => {
            let all_attrs_for_other: HashMap<String, String> = attributes
                .iter()
                .map(|(k, v_vec)| (k.clone(), v_vec.join("\n")))
                .collect();
            // Clear attributes map as all its content is now moved to all_attrs_for_other for the "Other" type
            attributes.clear();

            RpslObject::Other {
                obj_type: obj_type_key.to_string(),
                attributes: all_attrs_for_other,
            }
        }
    }
}

// --- Example Usage (you can put this in main.rs or tests) ---
#[cfg(test)]
mod tests {
    use super::*;
    use iprange::ipv4_range_from_str;
    use time::macros::datetime;

    #[test]
    fn test_parse_simple_inetnum() {
        let data = r#"
inetnum:      192.0.2.0 - 192.0.2.255
netname:      EXAMPLE-NET
descr:        Example Network Block
country:      EU
admin-c:      PERSON-C-1
tech-c:       PERSON-C-1
status:       ASSIGNED PI
mnt-by:       MAINT-EXAMPLE
created:      20010101
last-modified: 20020202
source:       RIPE
extra-attr:   Some Value
        continuation for extra
"#;
        let objects = parse_rpsl(data);
        assert_eq!(objects.len(), 1);
        if let RpslObject::Inetnum(inetnum_data) = &objects[0] {
            assert_eq!(
                inetnum_data.inetnum,
                ipv4_range_from_str("192.0.2.0 - 192.0.2.255").ok()
            );
            assert_eq!(inetnum_data.netname.as_deref(), Some("EXAMPLE-NET"));
            assert_eq!(inetnum_data.descr, vec!["Example Network Block"]);
            assert_eq!(inetnum_data.country.as_deref(), Some("EU"));
            assert_eq!(
                inetnum_data.created,
                Some(datetime!(2001-01-01 00:00:00 UTC))
            );
            assert_eq!(
                inetnum_data.last_modified,
                Some(datetime!(2002-02-02 00:00:00 UTC))
            );
            assert_eq!(
                inetnum_data.extra.get("extra-attr").unwrap(),
                "Some Value continuation for extra"
            );
        } else {
            panic!("Expected Inetnum object");
        }
    }

    #[test]
    fn test_parse_multiple_objects() {
        let data = r#"
person:       John Doe
nic-hdl:      JD1-TEST
address:      123 Example St
address:      Anytown, USA
email:        jd@example.com
changed:      20200101
source:       TEST

aut-num:      AS65000
as-name:      EXAMPLE-AS
admin-c:      JD1-TEST
source:       TEST
remarks:      This is a test AS.
"#;
        let objects = parse_rpsl(data);
        assert_eq!(objects.len(), 2);
        assert!(matches!(objects[0], RpslObject::Person(_)));
        assert!(matches!(objects[1], RpslObject::AutNum(_)));

        if let RpslObject::Person(person_data) = &objects[0] {
            assert_eq!(person_data.person.as_deref(), Some("John Doe"));
            assert_eq!(person_data.address, vec!["123 Example St", "Anytown, USA"]);
            assert_eq!(
                person_data.last_modified,
                Some(datetime!(2020-01-01 00:00:00 UTC))
            )
        }
        if let RpslObject::AutNum(autnum_data) = &objects[1] {
            assert_eq!(autnum_data.aut_num.as_deref(), Some("AS65000"));
            assert_eq!(
                autnum_data.extra.get("remarks").unwrap(),
                "This is a test AS."
            );
        }
    }

    #[test]
    fn test_parse_other_object() {
        let data = r#"
poem:         The Raven
author:       Edgar Allan Poe
category:     Narrative
text:         Once upon a midnight dreary,
              while I pondered, weak and weary...
"#;
        let objects = parse_rpsl(data);
        assert_eq!(objects.len(), 1);
        if let RpslObject::Other {
            obj_type,
            attributes,
        } = &objects[0]
        {
            assert_eq!(obj_type, "poem");
            assert_eq!(attributes.get("poem").unwrap(), "The Raven");
            assert_eq!(attributes.get("author").unwrap(), "Edgar Allan Poe");
            assert_eq!(
                attributes.get("text").unwrap(),
                "Once upon a midnight dreary, while I pondered, weak and weary..."
            );
        } else {
            panic!("Expected Other object");
        }
    }

    #[test]
    fn test_arin_email_and_changed_date() {
        let data = r#"
person:       ARIN Person
nic-hdl:      AP1-ARIN
e-mail:       arin.person@example.net
changed:      2022-07-15T08:30:00Z
source:       ARIN
"#;
        let objects = parse_rpsl(data);
        assert_eq!(objects.len(), 1);
        if let RpslObject::Person(person_data) = &objects[0] {
            assert_eq!(
                person_data.email.as_deref(),
                Some("arin.person@example.net")
            );
            assert_eq!(
                person_data.last_modified,
                Some(datetime!(2022-07-15 08:30:00 UTC))
            );
        } else {
            panic!("Expected Person object");
        }
    }
    #[test]
    fn test_implicit_continuation_no_colon() {
        let data = r#"
descr:        This is a very long
              description that continues
              on multiple lines.
              Implicit continuation here
mnt-by:       MAINT-TEST
"#;
        let objects = parse_rpsl(data);
        assert_eq!(objects.len(), 1);
        if let RpslObject::Other {
            obj_type,
            attributes,
        } = &objects[0]
        {
            // Will be Other because first key is descr
            assert_eq!(obj_type, "descr");
            assert_eq!(
                attributes.get("descr").unwrap(),
                "This is a very long description that continues on multiple lines. Implicit continuation here"
            );
            assert_eq!(attributes.get("mnt-by").unwrap(), "MAINT-TEST");
        } else {
            panic!("Expected Other object, got {:?}", objects[0]);
        }
    }
}
