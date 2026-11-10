//! The repository a command runs in, and how it finds items: on the base
//! branch's tip, and on the local branch named for each.

use crate::git;
use anyhow::{Context, Result};
use atelier_app::acts::{approval, mission_of, Checkout, SinceApproval};
use atelier_app::{identity, Branch, Located};
use atelier_core::{Config, Status, Thread};
use atelier_records::{
    allocate_id, parse_config, validate_id, CommitReader, ItemDir, MissionDir, Store,
};
use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

/// What an item's start depends on, as [`Repo::links`] finds it.
pub struct Links {
    pub blockers: Vec<(String, Option<Status>)>,
    pub mission: Option<(String, Option<Status>)>,
}

pub struct Repo {
    pub root: PathBuf,
    pub base: String,
    pub base_tip: String,
    /// The prefix of every ID, from the checkout's config.
    pub slug: String,
    /// The branch checked out, if any.
    pub branch: Option<String>,
    pub caller: Thread,
    reader: CommitReader,
}

impl Repo {
    pub fn open() -> Result<Repo> {
        let root = git::root()?;
        let config = Store::new(root.join(".atelier"))
            .config()
            .context("cannot read this repository's Atelier config")?;
        let base_tip = git::branch_tip(&root, &config.base_branch)?.with_context(|| {
            format!(
                "the base branch {} named in .atelier/config.toml does not exist",
                config.base_branch
            )
        })?;
        Ok(Repo {
            branch: git::current_branch(&root)?,
            reader: CommitReader::new(&root),
            caller: identity(|name| std::env::var(name).ok()),
            base: config.base_branch,
            slug: config.slug,
            base_tip,
            root,
        })
    }

    pub fn store(&self) -> Store {
        Store::new(self.root.join(".atelier"))
    }

    /// The item `id`: its record at the base's tip, and its branch.
    pub fn locate(&mut self, id: &str) -> Result<Located> {
        let tip = git::branch_tip(&self.root, id)?;
        let base = self.reader.item(&self.base_tip, id)?;
        self.locate_at(id, tip, base)
    }

    /// The item `id` as a planning command on the base finds it: its record
    /// in this checkout, where planning is written before it is committed,
    /// and its branch.
    pub fn locate_planned(&mut self, id: &str) -> Result<Located> {
        let tip = git::branch_tip(&self.root, id)?;
        let base = self.checkout_item(id)?;
        self.locate_at(id, tip, base)
    }

    fn locate_at(
        &mut self,
        id: &str,
        tip: Option<String>,
        base: Option<ItemDir>,
    ) -> Result<Located> {
        let branch = match tip {
            None => None,
            Some(tip) => {
                let merged = git::is_ancestor(&self.root, &tip, &self.base_tip)?;
                let record = if self.branch.as_deref() == Some(id) {
                    self.checkout_item(id)?
                } else {
                    self.reader.item(&tip, id)?
                };
                Some(Branch { merged, record })
            }
        };
        Ok(Located {
            id: id.to_string(),
            base,
            branch,
        })
    }

    /// The item of the checked-out branch, when it is named for an item.
    pub fn current(&mut self) -> Result<Option<Located>> {
        let Some(branch) = self.branch.clone() else {
            return Ok(None);
        };
        if branch == self.base || validate_id(&branch).is_err() {
            return Ok(None);
        }
        let located = self.locate(&branch)?;
        Ok(located.record().is_some().then_some(located))
    }

    /// Every item on the base and on a local branch named for one.
    pub fn everything(&mut self) -> Result<Vec<Located>> {
        let tips: BTreeMap<String, String> = git::branches(&self.root)?
            .into_iter()
            .filter(|(name, _)| validate_id(name).is_ok())
            .collect();
        let mut ids: BTreeSet<String> = self.reader.item_ids(&self.base_tip)?.into_iter().collect();
        ids.extend(tips.keys().cloned());
        ids.into_iter()
            .map(|id| {
                let tip = tips.get(&id).cloned();
                let base = self.reader.item(&self.base_tip, &id)?;
                self.locate_at(&id, tip, base)
            })
            .collect()
    }

    /// Every mission at the base's tip.
    pub fn missions(&mut self) -> Result<Vec<MissionDir>> {
        let ids = self.reader.mission_ids(&self.base_tip)?;
        let mut missions = Vec::new();
        for id in ids {
            missions.extend(self.reader.mission(&self.base_tip, &id)?);
        }
        Ok(missions)
    }

    /// A new ID that no item or mission has, in this checkout, at the
    /// base's tip or as a local branch.
    pub fn new_id(&mut self) -> Result<String> {
        let store = self.store();
        let mut taken: BTreeSet<String> = store.ids("items")?.into_iter().collect();
        taken.extend(store.ids("missions")?);
        taken.extend(self.reader.item_ids(&self.base_tip)?);
        taken.extend(self.reader.mission_ids(&self.base_tip)?);
        taken.extend(git::branches(&self.root)?.into_iter().map(|(name, _)| name));
        Ok(allocate_id(&self.slug, |id| taken.contains(id))?)
    }

    /// Each blocker of `item`, with its status, and its mission, with the
    /// mission's status at the base's tip.
    pub fn links(&mut self, item: &ItemDir) -> Result<Links> {
        let blockers = item
            .item
            .blocked_by
            .iter()
            .map(|id| Ok((id.clone(), self.locate(id)?.status())))
            .collect::<Result<_>>()?;
        let mission = match &item.item.mission {
            None => None,
            Some(id) => {
                let found = self.reader.mission(&self.base_tip, id)?;
                mission_of(item, found.as_slice())
            }
        };
        Ok(Links { blockers, mission })
    }

    /// The item `id` as `commit` holds it.
    pub fn item_at(&mut self, commit: &str, id: &str) -> Result<Option<ItemDir>> {
        Ok(self.reader.item(commit, id)?)
    }

    /// What `merge` reads from this checkout, on the branch of `record`,
    /// whose HEAD is the commit `head`.
    pub fn checkout(&mut self, record: &ItemDir, head: &str) -> Result<Checkout> {
        let approved = approval(record).filter(|_| record.item.status == Status::Approved);
        let since_approval = match approved {
            None => None,
            Some(review) => {
                let commit = &review.review.commit;
                Some(SinceApproval {
                    changed: git::changed(&self.root, commit, head)?,
                    reviewed: self.reader.item(commit, &record.item.id)?,
                    head: self.reader.item(head, &record.item.id)?,
                })
            }
        };
        Ok(Checkout {
            uncommitted: self.uncommitted(&record.item.id)?,
            since_approval,
        })
    }

    /// What the checkout holds that no commit does and `merge` would leave
    /// behind: changes to tracked files, and new files in the item's
    /// directory, such as a review file.
    pub fn uncommitted(&self, id: &str) -> Result<Vec<String>> {
        let mut paths = git::uncommitted(&self.root)?;
        paths.extend(git::untracked(&self.root, &item_dir(id))?);
        Ok(paths)
    }

    /// `config.toml` as the base's tip holds it, which `merge` reads, so a
    /// work branch cannot weaken its own checks.
    pub fn base_config(&mut self) -> Result<Config> {
        let path = ".atelier/config.toml";
        let text = self.reader.text(&self.base_tip, path)?;
        let shown = format!("{}:{path}", self.base);
        Ok(parse_config(Path::new(&shown), &text)?)
    }

    fn checkout_item(&self, id: &str) -> Result<Option<ItemDir>> {
        let store = self.store();
        if !self.root.join(item_path(id)).exists() {
            return Ok(None);
        }
        Ok(Some(store.item(id)?))
    }
}

/// `root` when the caller's directory is not that checkout's root, so that
/// a printed Git command naming paths from the root needs `git -C <root>`.
pub fn git_at(root: &Path) -> Option<&Path> {
    let here = std::env::current_dir().and_then(std::fs::canonicalize).ok();
    (here.is_none() || here != std::fs::canonicalize(root).ok()).then_some(root)
}

/// The directories planning is written in.
const PLANNING: [&str; 2] = [".atelier/missions", ".atelier/items"];

/// The record changes the checkout at `root` holds that no commit does:
/// changed and new files under `.atelier/missions/` and `.atelier/items/`.
pub fn planning(root: &Path) -> Result<Vec<String>> {
    let mut paths: Vec<String> = git::uncommitted(root)?
        .into_iter()
        .filter(|path| {
            PLANNING
                .iter()
                .any(|dir| path.starts_with(&format!("{dir}/")))
        })
        .collect();
    for dir in PLANNING {
        paths.extend(git::untracked(root, dir)?);
    }
    paths.sort();
    Ok(paths)
}

pub fn item_dir(id: &str) -> String {
    format!(".atelier/items/{id}")
}

pub fn item_path(id: &str) -> String {
    format!(".atelier/items/{id}/item.md")
}

pub fn mission_path(id: &str) -> String {
    format!(".atelier/missions/{id}/mission.md")
}
