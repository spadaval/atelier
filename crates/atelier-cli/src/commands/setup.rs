//! Install the documented Claude Code SessionStart/Stop command hooks.
use anyhow::{bail, Context, Result};
use atelier_app::storage_layout::StorageLayout;
use serde_json::{json, Value};
use std::{fs, io::Write, path::Path};

pub fn run(binary: Option<&Path>, quiet: bool) -> Result<()> {
    let layout = StorageLayout::discover()?;
    let directory = layout.repo_root().join(".claude");
    let path = directory.join("settings.json");
    for candidate in [&directory, &path] {
        if fs::symlink_metadata(candidate).is_ok_and(|metadata| metadata.file_type().is_symlink()) {
            bail!(
                "setup refuses symbolic link {}; use a real project settings path",
                candidate.display()
            );
        }
    }
    let before = match fs::read(&path) {
        Ok(bytes) => bytes,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Vec::new(),
        Err(error) => return Err(error.into()),
    };
    let mut settings: Value = if before.is_empty() {
        json!({})
    } else {
        serde_json::from_slice(&before)
            .context("Claude settings are invalid JSON; no files changed")?
    };
    let object = settings
        .as_object_mut()
        .context("Claude settings must be an object")?;
    let hooks = object
        .entry("hooks")
        .or_insert_with(|| json!({}))
        .as_object_mut()
        .context("Claude hooks must be an object")?;
    let command = match binary {
        Some(path) => format!(
            "'{}'",
            fs::canonicalize(path)
                .context("hook binary path is unavailable")?
                .to_str()
                .context("hook binary path is not UTF-8")?
                .replace('\'', "'\\''")
        ),
        None => "atelier".into(),
    };
    for (event, action) in [("SessionStart", "prime"), ("Stop", "land")] {
        let marker = format!(" # atelier-managed:{event}");
        let groups = hooks
            .entry(event)
            .or_insert_with(|| json!([]))
            .as_array_mut()
            .context("Claude hook events must be arrays")?;
        for group in groups.iter_mut() {
            let handlers = group["hooks"]
                .as_array_mut()
                .context("Claude matcher groups require a hooks array")?;
            handlers.retain(|handler| {
                !(handler["type"] == "command"
                    && handler["command"]
                        .as_str()
                        .is_some_and(|command| command.ends_with(&marker)))
            });
        }
        groups.retain(|group| {
            group["hooks"]
                .as_array()
                .is_none_or(|handlers| !handlers.is_empty())
        });
        groups.push(json!({"hooks": [{"type": "command", "command": format!("{command} {action} --hook claude{marker}"), "timeout": 30}]}));
    }
    let mut after = serde_json::to_vec_pretty(&settings)?;
    after.push(b'\n');
    if after != before {
        fs::create_dir_all(&directory)?;
        let mut staged = tempfile::NamedTempFile::new_in(&directory)?;
        staged.write_all(&after)?;
        // Keep the settings file's mode; a temporary file starts at 0600
        // (atelier-v44v). A new one gets 0644.
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let permissions = fs::metadata(&path)
                .map(|existing| existing.permissions())
                .unwrap_or_else(|_| fs::Permissions::from_mode(0o644));
            staged.as_file().set_permissions(permissions)?;
        }
        staged.as_file().sync_all()?;
        if fs::read(&path).unwrap_or_default() != before {
            bail!("Claude settings changed during setup; no update installed; retry");
        }
        staged.persist(&path).map_err(|error| error.error)?;
    }
    if quiet {
        println!("{}", path.display());
    } else {
        println!("Installed Claude Code SessionStart and Stop hooks in {}. Commit shared settings with the change. Hooks run prime on session start and land at Stop; repeated failures warn instead of looping. Restart Claude Code to load the settings.", path.display());
        if settings["disableAllHooks"] == true {
            println!("Hooks are disabled by existing settings; enable them in Claude Code before dogfooding.");
        }
    }
    Ok(())
}
