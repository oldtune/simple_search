use std::str::FromStr;

#[derive(Debug)]
pub enum SearchType {
    Fuzzy,
    Normal,
    Regex,
}

impl FromStr for SearchType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "regex" => Ok(Self::Regex),
            "normal" => Ok(Self::Normal),
            "fuzzy" => Ok(Self::Fuzzy),
            _ => Err("Not supported".to_string()),
        }
    }
}
