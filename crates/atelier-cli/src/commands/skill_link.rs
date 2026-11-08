//! `setup --link` and `--unlink`: point the globally installed Agent Factory
//! skill at a checkout of this repository, so it can be edited live.
//!
//! Vercel's `skills` CLI installs the skill as real files in
//! `~/.agents/skills/agent-factory` with a relative symlink to them from
//! `~/.claude/skills/agent-factory`, and records it in its lock file. Its
//! `update` and `add` replace a symlink to a skill the lock file lists, and
//! leave an unlisted one alone, so linking removes the lock entry too.
use anyhow::{bail, Context, Result};
use atelier_app::storage_layout::StorageLayout;
use serde_json::Value;
use std::{
    env, fs,
    io::Write,
    path::{Path, PathBuf},
};

const SKILL: &str = "agent-factory";
const REINSTALL: &str = "npx skills add spadaval/atelier --skill agent-factory -g";

/// The paths `skills add -g` uses, resolved as `skills` 1.7.6 resolves them.
struct Install {
    links: [PathBuf; 2],
    lock: PathBuf,
}

impl Install {
    fn from_env() -> Result<Self> {
        let home = env::var_os("HOME")
            .filter(|home| !home.is_empty())
            .map(PathBuf::from)
            .context("HOME is not set, so the installed skill cannot be found")?;
        let claude = env::var("CLAUDE_CONFIG_DIR")
            .ok()
            .map(|dir| dir.trim().to_owned())
            .filter(|dir| !dir.is_empty())
            .map_or_else(|| home.join(".claude"), PathBuf::from);
        let lock = env::var_os("XDG_STATE_HOME")
            .filter(|dir| !dir.is_empty())
            .map_or_else(
                || home.join(".agents"),
                |dir| PathBuf::from(dir).join("skills"),
            )
            .join(".skill-lock.json");
        Ok(Self {
            links: [
                home.join(".agents/skills").join(SKILL),
                claude.join("skills").join(SKILL),
            ],
            lock,
        })
    }
}

/// The `skills` lock file, parsed. A missing or unreadable one lists
/// nothing, as `skills` itself reads it.
struct Lock {
    value: Option<Value>,
}

impl Lock {
    fn read(path: &Path) -> Result<Self> {
        let bytes = match fs::read(path) {
            Ok(bytes) => bytes,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                return Ok(Self { value: None })
            }
            Err(error) => {
                return Err(error).with_context(|| format!("cannot read {}", path.display()))
            }
        };
        Ok(Self {
            value: serde_json::from_slice(&bytes).ok(),
        })
    }

    fn lists_skill(&self) -> bool {
        self.value
            .as_ref()
            .and_then(|lock| lock.get("skills"))
            .and_then(Value::as_object)
            .is_some_and(|skills| skills.contains_key(SKILL))
    }

    /// Removes the skill's entry, leaving every other one, and writes the
    /// file in place of the old one.
    fn remove_skill(mut self, path: &Path) -> Result<()> {
        let Some(skills) = self
            .value
            .as_mut()
            .and_then(|lock| lock.get_mut("skills"))
            .and_then(Value::as_object_mut)
        else {
            return Ok(());
        };
        skills.remove(SKILL);
        let directory = path.parent().context("lock file has no directory")?;
        let mut staged = tempfile::NamedTempFile::new_in(directory)
            .with_context(|| format!("cannot write beside {}", path.display()))?;
        staged.write_all(&serde_json::to_vec_pretty(&self.value)?)?;
        if let Ok(existing) = fs::metadata(path) {
            staged.as_file().set_permissions(existing.permissions())?;
        }
        staged.as_file().sync_all()?;
        staged
            .persist(path)
            .map_err(|error| error.error)
            .with_context(|| format!("cannot replace {}", path.display()))?;
        Ok(())
    }
}

/// What an install path held before linking.
enum Before {
    Absent,
    Link(PathBuf),
    /// A real directory the lock file lists, so `skills add` put it there.
    InstalledCopy,
}

fn inspect(path: &Path, tracked: bool, lock: &Path) -> Result<Before> {
    let metadata = match fs::symlink_metadata(path) {
        Ok(metadata) => metadata,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(Before::Absent),
        Err(error) => {
            return Err(error).with_context(|| format!("cannot inspect {}", path.display()))
        }
    };
    if metadata.file_type().is_symlink() {
        return Ok(Before::Link(fs::read_link(path)?));
    }
    if metadata.is_dir() && tracked {
        return Ok(Before::InstalledCopy);
    }
    let what = if metadata.is_dir() {
        format!(
            "a directory that the skills lock file {} does not list, so it may hold your own edits",
            lock.display()
        )
    } else {
        "a file, not a skill directory".into()
    };
    bail!(
        "setup --link refused: {} is {what}. Move it aside (`mv {0} {0}.bak`) and rerun; nothing changed.",
        path.display()
    )
}

pub fn link(checkout: Option<&Path>, quiet: bool) -> Result<()> {
    let checkout = match checkout {
        Some(path) => path.to_path_buf(),
        None => StorageLayout::discover()?.repo_root().to_path_buf(),
    };
    let checkout = fs::canonicalize(&checkout).with_context(|| {
        format!(
            "setup --link refused: checkout {} does not exist; nothing changed",
            checkout.display()
        )
    })?;
    let skill = checkout.join("skills").join(SKILL);
    if !skill.join("SKILL.md").is_file() {
        bail!(
            "setup --link refused: {} does not exist, so {} has no Agent Factory skill to link. Link a checkout of spadaval/atelier that has skills/{SKILL}; nothing changed.",
            skill.join("SKILL.md").display(),
            checkout.display()
        );
    }
    let install = Install::from_env()?;
    let lock = Lock::read(&install.lock)?;
    let tracked = lock.lists_skill();
    // Inspect both paths before changing either, so a refusal changes nothing.
    let before = install
        .links
        .iter()
        .map(|path| inspect(path, tracked, &install.lock))
        .collect::<Result<Vec<_>>>()?;

    // Drop the lock entry first: a copy left untracked by a failure below is
    // harmless, while a link the lock still lists is replaced by the next
    // `skills update`.
    if tracked {
        lock.remove_skill(&install.lock)?;
    }
    let mut rows = Vec::new();
    for (path, before) in install.links.iter().zip(before) {
        let was = match &before {
            Before::Link(target) if *target == skill => {
                rows.push(format!("  {}: already linked", path.display()));
                continue;
            }
            Before::Absent => "was absent".to_owned(),
            Before::Link(target) => format!("was a link to {}", target.display()),
            Before::InstalledCopy => "was a copy skills add installed, now removed".to_owned(),
        };
        replace_with_link(path, &skill, &before)?;
        rows.push(format!("  {}: {was}", path.display()));
    }
    if quiet {
        println!("{}", skill.display());
        return Ok(());
    }
    println!("Linked the Agent Factory skill to {}.", skill.display());
    for row in rows {
        println!("{row}");
    }
    if tracked {
        println!(
            "Removed {SKILL} from {}, so `npx skills update -g` leaves the links alone.",
            install.lock.display()
        );
    }
    println!(
        "The live skill follows whatever {} has checked out; keep it on master. Undo with `atelier setup --unlink`.",
        checkout.display()
    );
    Ok(())
}

fn replace_with_link(path: &Path, skill: &Path, before: &Before) -> Result<()> {
    let parent = path.parent().context("install path has no directory")?;
    fs::create_dir_all(parent).with_context(|| format!("cannot create {}", parent.display()))?;
    match before {
        Before::InstalledCopy => {
            fs::remove_dir_all(path).with_context(|| format!("cannot remove {}", path.display()))?
        }
        Before::Link(_) => {
            fs::remove_file(path).with_context(|| format!("cannot remove {}", path.display()))?
        }
        Before::Absent => {}
    }
    symlink_dir(skill, path).with_context(|| format!("cannot link {}", path.display()))
}

#[cfg(unix)]
fn symlink_dir(target: &Path, link: &Path) -> std::io::Result<()> {
    std::os::unix::fs::symlink(target, link)
}

#[cfg(windows)]
fn symlink_dir(target: &Path, link: &Path) -> std::io::Result<()> {
    std::os::windows::fs::symlink_dir(target, link)
}

pub fn unlink(quiet: bool) -> Result<()> {
    let install = Install::from_env()?;
    let tracked = Lock::read(&install.lock)?.lists_skill();
    let mut rows = Vec::new();
    for path in &install.links {
        let row = match fs::symlink_metadata(path) {
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => "no link".to_owned(),
            Err(error) => {
                return Err(error).with_context(|| format!("cannot inspect {}", path.display()))
            }
            Ok(_) if tracked => "left alone: skills add installed it".to_owned(),
            Ok(metadata) if metadata.file_type().is_symlink() => {
                let target = fs::read_link(path)?;
                fs::remove_file(path)
                    .with_context(|| format!("cannot remove {}", path.display()))?;
                if quiet {
                    println!("{}", path.display());
                }
                format!("removed its link to {}", target.display())
            }
            Ok(_) => "left alone: not a link".to_owned(),
        };
        rows.push(format!("  {}: {row}", path.display()));
    }
    if !quiet {
        println!("Unlinked the Agent Factory skill.");
        for row in rows {
            println!("{row}");
        }
        println!("Reinstall it with `{REINSTALL}`.");
    }
    Ok(())
}
