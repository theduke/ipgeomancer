/// Regional Internet Registry enumeration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
