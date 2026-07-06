use anyhow::{bail, Result};

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validates_project_scoped_ids() {
        assert!(validate_record_id("atelier-z1p8").is_ok());
        assert!(validate_record_id("ISS-0001").is_err());
        assert!(validate_record_id("1").is_err());
    }
}
