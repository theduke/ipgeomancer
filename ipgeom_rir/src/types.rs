/// Regional Internet Registry enumeration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Rir {
    Arin,
    Apnic,
    Ripe,
    Lacnic,
    Afrinic,
}

impl std::fmt::Display for Rir {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Rir::Arin => write!(f, "ARIN"),
            Rir::Apnic => write!(f, "APNIC"),
            Rir::Ripe => write!(f, "RIPE"),
            Rir::Lacnic => write!(f, "LACNIC"),
            Rir::Afrinic => write!(f, "AFRINIC"),
        }
    }
}

impl Rir {
    /// Name of the registry in lowercase form.
    pub fn name(&self) -> &'static str {
        match self {
            Rir::Arin => "arin",
            Rir::Apnic => "apnic",
            Rir::Ripe => "ripe",
            Rir::Lacnic => "lacnic",
            Rir::Afrinic => "afrinic",
        }
    }

    /// Array of all supported registries.
    pub const ALL: [Rir; 5] = [Rir::Arin, Rir::Apnic, Rir::Ripe, Rir::Lacnic, Rir::Afrinic];
}

impl std::str::FromStr for Rir {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "ARIN" => Ok(Rir::Arin),
            "APNIC" => Ok(Rir::Apnic),
            "RIPE" => Ok(Rir::Ripe),
            "LACNIC" => Ok(Rir::Lacnic),
            "AFRINIC" => Ok(Rir::Afrinic),
            _ => Err("Invalid RIR"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_and_name() {
        for rir in Rir::ALL.iter() {
            let upper = rir.to_string();
            let lower = rir.name();
            assert_eq!(upper.to_lowercase(), lower);
            assert_eq!(upper.parse::<Rir>().unwrap(), *rir);
        }
        assert!("unknown".parse::<Rir>().is_err());
    }
}
