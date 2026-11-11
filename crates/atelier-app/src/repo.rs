//! The repository an act runs in, and how it finds items: on the base
//! branch's tip, and on the local branch named for each.

use crate::acts::{approval, mission_of, Checkout, Facts, SinceApproval};
use crate::{git, say, Branch, Located, Skip};
use anyhow::{bail, Context, Result};
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
    /// The mission's lead, who decides the item's questions when it may.
    pub lead: Option<Thread>,
}

/// A repository opened for one invocation, by its caller.
pub struct Repo<'s> {
    pub root: PathBuf,
    /// The directory it was opened from, which may be under `root`.
    pub dir: PathBuf,
    pub base: String,
    pub base_tip: String,
    /// The prefix of every ID, from the checkout's config.
    pub slug: String,
    /// The branch checked out, if any.
    pub branch: Option<String>,
    /// Who acts, as the records name them.
    pub caller: Thread,
    /// The invocation's skip, which its refusals and its commit go through.
    pub skip: &'s Skip,
    reader: CommitReader,
    /// Opened with [`Repo::open_committed`]: no record is read from the
    /// checkout's files.
    pub(crate) committed: bool,
}

impl<'s> Repo<'s> {
    /// The repository of the checkout `dir` is in, for `caller`, whose
    /// identity the records are given; `skip` is the invocation's. A caller
    /// the records could not name and read back is refused.
    pub fn open(dir: &Path, caller: Thread, skip: &'s Skip) -> Result<Repo<'s>> {
        Repo::opened(dir, caller, skip, false)
    }

    /// The repository of the checkout `dir` is in, opened for reading from
    /// commits only, as the Desk reads it: its `config.toml` as the commit
    /// `HEAD` names holds it, and each item's record on its branch from the
    /// branch's tip even when that branch is checked out here, so nothing
    /// the checkout has not committed is read. [`Repo::everything`],
    /// [`Repo::locate`], [`Repo::current`], [`Repo::missions`] and
    /// [`Repo::links`] read commits only. Planning's and the acts' methods,
    /// which read and write the checkout (`store`, `checkout_item`,
    /// `locate_planned`, `planned`, `new_id`, `uncommitted`), are not for a
    /// repository opened this way.
    pub fn open_committed(dir: &Path, caller: Thread, skip: &'s Skip) -> Result<Repo<'s>> {
        Repo::opened(dir, caller, skip, true)
    }

    fn opened(dir: &Path, caller: Thread, skip: &'s Skip, committed: bool) -> Result<Repo<'s>> {
        if !readable(&caller) {
            bail!("the caller's identity `{caller}` cannot be written into a record and read back as it is: it needs to be one line with no `: ` in it");
        }
        let root = git::root(dir)?;
        // Said plainly, so a thread outside Atelier sets it aside rather
        // than taking a missing file for a broken one.
        if !committed && !root.join(".atelier").exists() {
            bail!(
                "this is not an Atelier repository: {} has no .atelier/ directory",
                root.display()
            );
        }
        let mut reader = CommitReader::new(&root);
        let config = match committed {
            false => Store::new(root.join(".atelier"))
                .config()
                .map_err(Into::into),
            true => head_config(&root, &mut reader),
        }
        .context("cannot read this repository's Atelier config")?;
        let base_tip = git::branch_tip(&root, &config.base_branch)?.with_context(|| {
            format!(
                "the base branch {} named in .atelier/config.toml does not exist",
                config.base_branch
            )
        })?;
        Ok(Repo {
            branch: git::current_branch(&root)?,
            reader,
            caller,
            skip,
            base: config.base_branch,
            slug: config.slug,
            base_tip,
            root,
            dir: dir.to_path_buf(),
            committed,
        })
    }

    /// The root when the directory it was opened from is not the root, so
    /// that a printed Git command naming paths from the root needs `git -C
    /// <root>`.
    pub fn git_at(&self) -> Option<&Path> {
        let dir = std::fs::canonicalize(&self.dir).ok();
        (dir.is_none() || dir != std::fs::canonicalize(&self.root).ok()).then_some(&self.root)
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
                // A terminal status on the base wins, so the branch's record
                // is not read, and one that does not parse cannot hide it.
                let record = if base.as_ref().is_some_and(Located::terminal) {
                    None
                } else if !self.committed && self.branch.as_deref() == Some(id) {
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

    /// Every item in this checkout and on a local branch named for one, as
    /// a planning command finds them (see [`Repo::locate_planned`]).
    pub fn planned(&mut self) -> Result<Vec<Located>> {
        let mut ids: BTreeSet<String> = self.store().ids("items")?.into_iter().collect();
        let branches = git::branches(&self.root)?.into_iter().map(|(name, _)| name);
        ids.extend(branches.filter(|name| validate_id(name).is_ok()));
        ids.iter().map(|id| self.locate_planned(id)).collect()
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
        let (mission, lead) = match &item.item.mission {
            None => (None, None),
            Some(id) => {
                let found = self.reader.mission(&self.base_tip, id)?;
                let lead = found.as_ref().and_then(|dir| dir.mission.lead.clone());
                (mission_of(item, found.as_slice()), lead)
            }
        };
        Ok(Links {
            blockers,
            mission,
            lead,
        })
    }

    /// The mission `id` as `commit` holds it.
    pub fn mission_at(&mut self, commit: &str, id: &str) -> Result<Option<MissionDir>> {
        Ok(self.reader.mission(commit, id)?)
    }

    /// Every item `commit` holds.
    pub fn items_at(&mut self, commit: &str) -> Result<Vec<ItemDir>> {
        let mut items = Vec::new();
        for id in self.reader.item_ids(commit)? {
            items.extend(self.reader.item(commit, &id)?);
        }
        Ok(items)
    }

    /// The item `id` as `commit` holds it.
    pub fn item_at(&mut self, commit: &str, id: &str) -> Result<Option<ItemDir>> {
        Ok(self.reader.item(commit, id)?)
    }

    /// What `merge` reads from this checkout, on the branch of `record`,
    /// whose HEAD is the commit `head`.
    pub fn checkout(&mut self, record: &ItemDir, head: &str) -> Result<Checkout> {
        Ok(Checkout {
            uncommitted: self.uncommitted(&record.item.id)?,
            since_approval: self.since_approval(record, head)?,
        })
    }

    /// On an approved `record`: what differs between the commit its
    /// approval reviewed and the commit `head`.
    pub fn since_approval(
        &mut self,
        record: &ItemDir,
        head: &str,
    ) -> Result<Option<SinceApproval>> {
        let approved = approval(record).filter(|_| record.item.status == Status::Approved);
        let Some(review) = approved else {
            return Ok(None);
        };
        let commit = &review.review.commit;
        Ok(Some(SinceApproval {
            changed: git::changed(&self.root, commit, head)?,
            reviewed: self.reader.item(commit, &record.item.id)?,
            head: self.reader.item(head, &record.item.id)?,
        }))
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

    /// The item `id` as this checkout holds it, committed or not.
    pub fn checkout_item(&self, id: &str) -> Result<Option<ItemDir>> {
        let store = self.store();
        if !self.root.join(item_path(id)).exists() {
            return Ok(None);
        }
        Ok(Some(store.item(id)?))
    }
}

/// `config.toml` as the commit `HEAD` names holds it.
fn head_config(root: &Path, reader: &mut CommitReader) -> Result<Config> {
    let path = ".atelier/config.toml";
    let text = reader.text(&git::head(root)?, path)?;
    Ok(parse_config(Path::new(&format!("HEAD:{path}")), &text)?)
}

/// Whether the records can name `thread` and read it back as it is. Front
/// matter quotes any text, but a record's line ends at a line break, and
/// `Answer by <thread>: <answer>` and `Dropped by <thread>: <reason>` end
/// the name at its first `: `.
fn readable(thread: &Thread) -> bool {
    !thread.0.contains(['\n', '\r']) && !thread.0.contains(": ")
}

/// What `act`s on the item `located`, whose links are `links`, are checked
/// against, for `repo`'s caller.
pub fn facts<'a>(
    repo: &'a Repo,
    located: &'a Located,
    links: &'a Links,
    checkout: Option<&'a Checkout>,
) -> Facts<'a> {
    Facts {
        item: located.record().expect("a located item with a record"),
        branch: located.branch.is_some(),
        blockers: &links.blockers,
        mission: links.mission.clone(),
        caller: &repo.caller,
        checkout,
    }
}

/// Writes each of `files`, a path in the checkout and its text, creating
/// its directory where there is none, and commits `paths`. When the commit
/// fails, each file is put back as it was, or removed with the directories
/// made for it where there was none, so the act can be run again.
pub fn write_and_commit(
    repo: &Repo,
    files: &[(String, String)],
    paths: &[&str],
    message: &str,
) -> Result<()> {
    let mut before = Vec::new();
    let mut made = Vec::new();
    for (path, _) in files {
        let path = repo.root.join(path);
        before.push(match std::fs::read(&path) {
            Ok(text) => Some(text),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => None,
            Err(error) => return Err(error.into()),
        });
        let missing = path.ancestors().skip(1).take_while(|dir| !dir.exists());
        made.extend(missing.map(Path::to_path_buf));
    }
    made.sort_by_key(|dir| std::cmp::Reverse(dir.components().count()));
    made.dedup();
    let put_back = || {
        for ((path, _), text) in files.iter().zip(&before) {
            let path = repo.root.join(path);
            let _ = match text {
                Some(text) => std::fs::write(path, text),
                None => std::fs::remove_file(path),
            };
        }
        for dir in &made {
            let _ = std::fs::remove_dir(dir);
        }
    };
    let written = files.iter().try_for_each(|(path, text)| {
        let path = repo.root.join(path);
        std::fs::create_dir_all(path.parent().expect("a record is in a directory"))?;
        std::fs::write(path, text)
    });
    if let Err(error) = written {
        put_back();
        return Err(error.into());
    }
    git::commit_paths(&repo.root, paths, message, repo.skip).map_err(|error| {
        put_back();
        error.context(say::not_committed())
    })
}

/// The directories planning is written in.
const PLANNING: [&str; 2] = [".atelier/missions", ".atelier/items"];

/// Whether `path`, from the root, is under a directory planning is
/// written in.
pub fn is_planning(path: &str) -> bool {
    PLANNING
        .iter()
        .any(|dir| path.starts_with(&format!("{dir}/")))
}

/// The record changes the checkout at `root` holds that no commit does:
/// changed and new files under `.atelier/missions/` and `.atelier/items/`.
pub fn planning(root: &Path) -> Result<Vec<String>> {
    let mut paths: Vec<String> = git::uncommitted(root)?
        .into_iter()
        .filter(|path| is_planning(path))
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

pub fn mission_dir(id: &str) -> String {
    format!(".atelier/missions/{id}")
}

pub fn mission_path(id: &str) -> String {
    format!(".atelier/missions/{id}/mission.md")
}
