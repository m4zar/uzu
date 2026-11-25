use anyhow::{Result, anyhow};
use regex::{Regex, Replacer};

// A synced file that no longer matches its pattern has lost the field the task keeps in step, so it is an error.
pub fn replace_required(
    input: &str,
    pattern: &str,
    replacement: impl Replacer,
) -> Result<String> {
    let regex = Regex::new(pattern)?;
    if !regex.is_match(input) {
        return Err(anyhow!("Nothing matches {pattern}"));
    }
    Ok(regex.replace_all(input, replacement).into_owned())
}

pub fn replace_json_string(
    input: &str,
    key: &str,
    value: &str,
) -> Result<String> {
    let literal = serde_json::to_string(value)?;
    replace_required(input, &format!(r#"("{key}"\s*:\s*)"[^"]*""#), format!("${{1}}{literal}"))
}
