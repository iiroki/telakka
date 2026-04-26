use serde::de::DeserializeOwned;

pub fn parse_json<T: DeserializeOwned>(s: &str) -> Result<T, String> {
    serde_json::from_str(s).map_err(|e| e.to_string())
}

pub fn parse_ndjson<T: DeserializeOwned>(s: &str) -> Result<Vec<T>, String> {
    s.lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(parse_json)
        .collect()
}
