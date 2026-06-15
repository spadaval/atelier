use anyhow::{bail, Context, Result};
use std::fs::File;
use std::io::Read;
use std::time::{SystemTime, UNIX_EPOCH};

pub const DEFAULT_PROJECT_SLUG: &str = "atelier";
const DEFAULT_SUFFIX_LEN: usize = 4;
const MAX_ALLOC_ATTEMPTS: usize = 128;

pub fn allocate_issue_id<F>(exists: F) -> Result<String>
where
    F: Fn(&str) -> Result<bool>,
{
    for _ in 0..MAX_ALLOC_ATTEMPTS {
        let id = format!(
            "{}-{}",
            DEFAULT_PROJECT_SLUG,
            random_base36_suffix(DEFAULT_SUFFIX_LEN)?
        );
        if !exists(&id)? {
            return Ok(id);
        }
    }
    bail!(
        "Unable to allocate unique record ID after {} attempts",
        MAX_ALLOC_ATTEMPTS
    )
}

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

fn random_base36_suffix(len: usize) -> Result<String> {
    let mut bytes = [0u8; 8];
    match File::open("/dev/urandom").and_then(|mut file| file.read_exact(&mut bytes)) {
        Ok(()) => {}
        Err(_) => {
            let nanos = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .context("System clock is before UNIX epoch")?
                .as_nanos();
            bytes.copy_from_slice(&(nanos as u64).to_le_bytes());
        }
    }
    let value = u64::from_le_bytes(bytes);
    Ok(base36_padded(value, len))
}

fn base36_padded(mut value: u64, len: usize) -> String {
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
    }
}
