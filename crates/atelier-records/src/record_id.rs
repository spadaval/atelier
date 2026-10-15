use anyhow::{bail, Context, Result};
use std::fs::File;
use std::io::Read;
use std::time::{SystemTime, UNIX_EPOCH};

const DEFAULT_SUFFIX_LEN: usize = 4;
const MAX_ALLOC_ATTEMPTS: usize = 128;

pub fn allocate_issue_id<F>(project_slug: &str, exists: F) -> Result<String>
where
    F: Fn(&str) -> Result<bool>,
{
    validate_project_slug(project_slug)?;
    for _ in 0..MAX_ALLOC_ATTEMPTS {
        let id = format!(
            "{}-{}",
            project_slug,
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

/// IDs are `<project-slug>-<suffix>`; the suffix follows the last hyphen, so
/// slugs may contain hyphens and IDs from any project slug stay valid.
pub fn validate_record_id(id: &str) -> Result<()> {
    let (slug, suffix) = id
        .rsplit_once('-')
        .ok_or_else(|| anyhow::anyhow!("expected <project-slug>-<random-base36>"))?;
    if validate_project_slug(slug).is_err()
        || suffix.is_empty()
        || !suffix
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit())
    {
        bail!("expected <project-slug>-<random-base36>");
    }
    Ok(())
}

/// Lowercase ASCII letters, digits and inner single hyphens.
pub fn validate_project_slug(slug: &str) -> Result<()> {
    if slug.is_empty()
        || slug.starts_with('-')
        || slug.ends_with('-')
        || slug.contains("--")
        || !slug
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
    {
        bail!("project slug '{slug}' must be lowercase letters, digits and single inner hyphens");
    }
    Ok(())
}

/// Derive a project slug from a repository directory name, if it has any
/// usable characters.
pub fn derive_project_slug(name: &str) -> Option<String> {
    let mut slug = String::new();
    for c in name.chars() {
        if c.is_ascii_alphanumeric() {
            slug.push(c.to_ascii_lowercase());
        } else if !slug.is_empty() && !slug.ends_with('-') {
            slug.push('-');
        }
    }
    let slug = slug.trim_end_matches('-').to_string();
    (!slug.is_empty()).then_some(slug)
}

#[cfg(test)]
pub fn legacy_issue_id(number: i64) -> String {
    format!(
        "atelier-{}",
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
        assert!(validate_record_id("astromech-z1p8").is_ok());
        assert!(validate_record_id("cosplay-studio-z1p8").is_ok());
        assert!(validate_record_id("ISS-0001").is_err());
        assert!(validate_record_id("-z1p8").is_err());
        assert!(validate_record_id("a--b-z1p8").is_err());
        assert!(validate_record_id("1").is_err());
    }

    #[test]
    fn allocates_ids_with_the_given_slug_and_refuses_invalid_slugs() {
        let id = allocate_issue_id("astromech", |_| Ok(false)).unwrap();
        assert!(id.starts_with("astromech-"), "{id}");
        assert!(validate_record_id(&id).is_ok());
        assert!(allocate_issue_id("Astro Mech", |_| Ok(false)).is_err());
    }

    #[test]
    fn derives_slugs_from_directory_names() {
        assert_eq!(
            derive_project_slug("astromech").as_deref(),
            Some("astromech")
        );
        assert_eq!(
            derive_project_slug("Cosplay_Studio.v2").as_deref(),
            Some("cosplay-studio-v2")
        );
        assert_eq!(derive_project_slug("--x--").as_deref(), Some("x"));
        assert_eq!(derive_project_slug("___"), None);
    }
}
