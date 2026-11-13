//! Copied from the old tool's `crates/atelier-records/src/record_id.rs`,
//! which 819024c75 deleted, trimmed to what
//! the new records use; the random suffix also names review files.

use std::fs::File;
use std::io::{self, Read};

const SUFFIX_LEN: usize = 4;
pub(crate) const MAX_ALLOC_ATTEMPTS: usize = 128;

/// A new `<slug>-<4 characters>` ID that `exists` says is free.
pub fn allocate_id(slug: &str, exists: impl Fn(&str) -> bool) -> io::Result<String> {
    validate_slug(slug).map_err(|message| io::Error::new(io::ErrorKind::InvalidInput, message))?;
    for _ in 0..MAX_ALLOC_ATTEMPTS {
        let id = format!("{slug}-{}", random_suffix()?);
        if !exists(&id) {
            return Ok(id);
        }
    }
    Err(io::Error::other(format!(
        "no free ID after {MAX_ALLOC_ATTEMPTS} attempts"
    )))
}

/// IDs are `<slug>-<suffix>`; the suffix follows the last hyphen, so slugs
/// may contain hyphens.
pub fn validate_id(id: &str) -> Result<(), String> {
    let valid = id.rsplit_once('-').is_some_and(|(slug, suffix)| {
        validate_slug(slug).is_ok()
            && !suffix.is_empty()
            && suffix
                .chars()
                .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit())
    });
    if valid {
        Ok(())
    } else {
        Err(format!("`{id}` is not an ID, <slug>-<random-base36>"))
    }
}

/// Lowercase ASCII letters, digits and inner single hyphens.
pub(crate) fn validate_slug(slug: &str) -> Result<(), String> {
    if slug.is_empty()
        || slug.starts_with('-')
        || slug.ends_with('-')
        || slug.contains("--")
        || !slug
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
    {
        return Err(format!(
            "`{slug}` is not a slug: lowercase letters, digits and single inner hyphens"
        ));
    }
    Ok(())
}

/// Four random base-36 characters.
pub(crate) fn random_suffix() -> io::Result<String> {
    let mut bytes = [0u8; 8];
    File::open("/dev/urandom")?.read_exact(&mut bytes)?;
    Ok(base36_padded(u64::from_le_bytes(bytes), SUFFIX_LEN))
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
    fn validates_ids_of_any_slug() {
        assert!(validate_id("atelier-z1p8").is_ok());
        assert!(validate_id("cosplay-studio-z1p8").is_ok());
        for bad in ["ISS-0001", "-z1p8", "a--b-z1p8", "1", "atelier-"] {
            assert!(validate_id(bad).is_err(), "{bad}");
        }
    }

    #[test]
    fn allocates_ids_with_the_slug_and_refuses_bad_slugs() {
        let id = allocate_id("astromech", |_| false).unwrap();
        assert!(
            id.starts_with("astromech-") && id.len() == "astromech-".len() + 4,
            "{id}"
        );
        assert!(validate_id(&id).is_ok());
        assert!(allocate_id("Astro Mech", |_| false).is_err());
        assert!(allocate_id("astromech", |_| true).is_err());
    }

    #[test]
    fn pads_and_truncates_to_the_length() {
        assert_eq!(base36_padded(1, 4), "0001");
        assert_eq!(base36_padded(36, 4), "0010");
        assert_eq!(base36_padded(36u64.pow(4), 4), "0000");
    }
}
