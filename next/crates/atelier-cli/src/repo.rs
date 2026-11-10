//! The repository a command runs in, and how it finds items: on the base
//! branch's tip, and on the local branch named for each.

use crate::git;
use anyhow::{Context, Result};
use atelier_app::acts::{approval, Checkout, SinceApproval};
use atelier_app::{identity, Branch, Located};
use atelier_core::{Config, Status, Thread};
use atelier_records::{parse_config, validate_id, CommitReader, ItemDir, Store};
use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

pub struct Repo {
    pub root: PathBuf,
    pub base: String,
    pub base_tip: String,
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
        self.locate_at(id, tip)
    }

    fn locate_at(&mut self, id: &str, tip: Option<String>) -> Result<Located> {
        let base = self.reader.item(&self.base_tip, id)?;
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
                self.locate_at(&id, tip)
            })
            .collect()
    }

    /// Each blocker of `item`, with its status.
    pub fn blockers(&mut self, item: &ItemDir) -> Result<Vec<(String, Option<Status>)>> {
        item.item
            .blocked_by
            .iter()
            .map(|id| Ok((id.clone(), self.locate(id)?.status())))
            .collect()
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

pub fn item_dir(id: &str) -> String {
    format!(".atelier/items/{id}")
}

pub fn item_path(id: &str) -> String {
    format!(".atelier/items/{id}/item.md")
}
