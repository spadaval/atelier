use anyhow::{bail, Context, Result};
pub use atelier_core::record_id::{
    base36_padded, legacy_issue_id, validate_record_id, DEFAULT_PROJECT_SLUG, DEFAULT_SUFFIX_LEN,
};
use std::fs::File;
use std::io::Read;
use std::time::{SystemTime, UNIX_EPOCH};

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
