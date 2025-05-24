use serde::Serialize;
use std::collections::HashMap;

/// Known RPSL object types.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum ObjectType {
    Inetnum,
    Inet6num,
    AutNum,
    Person,
    Role,
    Organisation,
    Mntner,
    Route,
    Route6,
    Other(String),
}

impl ObjectType {
    pub fn from_key(key: &str) -> Self {
        match key {
            "inetnum" => ObjectType::Inetnum,
            "inet6num" => ObjectType::Inet6num,
            "aut-num" => ObjectType::AutNum,
            "person" => ObjectType::Person,
            "role" => ObjectType::Role,
            "organisation" | "organization" => ObjectType::Organisation,
            "mntner" => ObjectType::Mntner,
            "route" => ObjectType::Route,
            "route6" => ObjectType::Route6,
            other => ObjectType::Other(other.to_string()),
        }
    }
}

/// RPSL object
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Object {
    obj_type: ObjectType,
    attributes: HashMap<String, Vec<String>>,
}

impl Object {
    /// Create a new RPSL object with the given type
    pub fn new(obj_type: ObjectType) -> Self {
        Object {
            obj_type,
            attributes: HashMap::new(),
        }
    }

    /// Add an attribute to the object
    pub fn add(&mut self, key: String, value: String) {
        self.attributes.entry(key).or_default().push(value);
    }

    /// Get the type of this RPSL object
    pub fn obj_type(&self) -> &ObjectType {
        &self.obj_type
    }

    pub fn get(&self, key: &str) -> Option<&[String]> {
        self.attributes.get(key).map(|v| v.as_slice())
    }

    /// Get a reference to the underlying attribute map
    pub fn attributes(&self) -> &HashMap<String, Vec<String>> {
        &self.attributes
    }

    /// Consume the object and return the underlying attribute map
    pub fn into_attributes(self) -> HashMap<String, Vec<String>> {
        self.attributes
    }

    /// Create an object from an attribute map and a type
    pub fn from_attributes(obj_type: ObjectType, map: HashMap<String, Vec<String>>) -> Self {
        Object {
            obj_type,
            attributes: map,
        }
    }

    /// Convert the object back into an RPSL formatted string
    pub fn to_rpsl(&self) -> String {
        let mut out = String::new();
        for (key, vals) in &self.attributes {
            for val in vals {
                if val.contains('\n') {
                    for line in val.lines() {
                        out.push_str(key);
                        out.push_str(": ");
                        out.push_str(line);
                        out.push('\n');
                    }
                } else {
                    out.push_str(key);
                    out.push_str(": ");
                    out.push_str(val);
                    out.push('\n');
                }
            }
        }
        out
    }
}
