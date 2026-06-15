use anyhow::{bail, Result};

pub const DEFAULT_PROJECT_SLUG: &str = "atelier";
pub const DEFAULT_SUFFIX_LEN: usize = 4;

pub fn validate_record_id(id: &str) -> Result<()> {
    let (slug, suffix) = id
        .split_once('-')
        .ok_or_else(|| anyhow::anyhow!("expected <project-slug>-<random-base36>"))?;
    if slug.is_empty()
        || suffix.is_empty()
        || !slug
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
        || !suffix
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit())
    {
        bail!("expected <project-slug>-<random-base36>");
    }
    Ok(())
}

pub fn legacy_issue_id(number: i64) -> String {
    format!(
        "{}-{}",
        DEFAULT_PROJECT_SLUG,
        base36_padded(number as u64, DEFAULT_SUFFIX_LEN)
    )
}

pub fn base36_padded(mut value: u64, len: usize) -> String {
    const ALPHABET: &[u8; 36] = b"0123456789abcdefghijklmnopqrstuvwxyz";
    let mut chars = Vec::new();
    loop {
        chars.push(ALPHABET[(value % 36) as usize] as char);
        value /= 36;
        if value == 0 {
            break;
        }
    }
    while chars.len() < len {
        chars.push('0');
    }
    chars.reverse();
    let suffix: String = chars.into_iter().collect();
    suffix[suffix.len().saturating_sub(len)..].to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn legacy_ids_are_project_scoped_base36() {
        assert_eq!(legacy_issue_id(1), "atelier-0001");
        assert_eq!(legacy_issue_id(36), "atelier-0010");
    }

    #[test]
    fn validates_project_scoped_ids() {
        assert!(validate_record_id("atelier-z1p8").is_ok());
        assert!(validate_record_id("ISS-0001").is_err());
        assert!(validate_record_id("1").is_err());
        assert!(validate_record_id("atelier-").is_err());
        assert!(validate_record_id("-0001").is_err());
    }
}
