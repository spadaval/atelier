//! The repository an act runs in, and how it finds items: on the base
//! branch's tip, and on the branch named for each, local or the forge's
//! remote-tracking one.

use crate::acts::{approval, mission_of, Checkout, Facts, SinceApproval};
use crate::view::Record;
use crate::{git, say, Branch, Located, Skip};
use anyhow::{bail, Context, Result};
use atelier_core::{Config, Named, Status, Thread};
use atelier_records::{allocate_id, parse_config, CommitReader, ItemDir, MissionDir, Store};
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::path::{Path, PathBuf};

/// A branch the convention names, as this clone has it: the local branch,
/// the forge's remote-tracking branch, or both.
struct Pair {
    /// Its name, as `git branch` gives it.
    name: String,
    /// What the convention names it for; a branch it does not name is
    /// kept only for the record whose own branch name it has.
    named: Option<Named>,
    local: Option<git::Ref>,
    remote: Option<git::Ref>,
}

/// The tip of a [`Pair`] that is read.
struct Tip {
    tip: String,
    remote: bool,
    merged: bool,
    diverged: bool,
}

/// Every named branch of this clone, by name: one listing of the refs.
struct Refs {
    pairs: BTreeMap<String, Pair>,
    /// The forge's remote, which a remote-tracking branch is shown under.
    remote: Option<String>,
}

/// The kind of record, read the same way at a commit or in the checkout.
trait Stored: Record + Sized {
    const DIR: &'static str;
    fn at(reader: &mut CommitReader, commit: &str, id: &str) -> Result<Option<Self>>;
    fn in_checkout(repo: &Repo, id: &str) -> Result<Option<Self>>;
}

impl Stored for ItemDir {
    const DIR: &'static str = ".atelier/items/";
    fn at(reader: &mut CommitReader, commit: &str, id: &str) -> Result<Option<Self>> {
        Ok(reader.item(commit, id)?)
    }
    fn in_checkout(repo: &Repo, id: &str) -> Result<Option<Self>> {
        repo.checkout_item(id)
    }
}

impl Stored for MissionDir {
    const DIR: &'static str = ".atelier/missions/";
    fn at(reader: &mut CommitReader, commit: &str, id: &str) -> Result<Option<Self>> {
        Ok(reader.mission(commit, id)?)
    }
    fn in_checkout(repo: &Repo, id: &str) -> Result<Option<Self>> {
        if !repo.root.join(mission_path(id)).exists() {
            return Ok(None);
        }
        Ok(Some(repo.store().mission(id)?))
    }
}

/// An unmerged named branch other than a record's own that touches it.
struct Touching {
    /// The branch's name.
    branch: String,
    tip: Tip,
    /// The records it changes, by whether each is an item and its ID.
    ids: BTreeSet<(bool, String)>,
}

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
    /// The checkout's config, which names its branches.
    pub config: Config,
    reader: CommitReader,
    /// Opened with [`Repo::open_committed`]: no record is read from the
    /// checkout's files.
    pub(crate) committed: bool,
    /// The record paths each branch tip changes since it forked from the
    /// base's tip, by tip: two commits' diff never changes.
    changed: std::cell::RefCell<HashMap<String, Vec<String>>>,
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
            base: config.base_branch.clone(),
            slug: config.slug.clone(),
            config,
            base_tip,
            root,
            dir: dir.to_path_buf(),
            committed,
            changed: Default::default(),
        })
    }

    /// The root when the directory it was opened from is not the root, so
    /// that a printed Git command naming paths from the root needs `git -C
    /// <root>`.
    pub fn git_at(&self) -> Option<&Path> {
        let dir = std::fs::canonicalize(&self.dir).ok();
        (dir.is_none() || dir != std::fs::canonicalize(&self.root).ok()).then_some(&self.root)
    }

    /// The reader of the repository's commits, whose one `git cat-file`
    /// process every read shares.
    pub(crate) fn reader(&mut self) -> &mut CommitReader {
        &mut self.reader
    }

    pub fn store(&self) -> Store {
        Store::new(self.root.join(".atelier"))
    }

    /// The item `id`: its record at the base's tip, and its branch, as the
    /// acts find it. A copy on any other branch is not read: an item a
    /// branch adds is not on the base until that branch merges.
    pub fn locate(&mut self, id: &str) -> Result<Located> {
        let refs = self.refs()?;
        let base = self.reader.item(&self.base_tip, id)?;
        self.located(&refs, id, base, Vec::new())
    }

    /// The item `id` wherever it is: [`Repo::locate`], and its copies on
    /// the other unmerged named branches, as `show` reads it.
    pub fn find(&mut self, id: &str) -> Result<Located> {
        let refs = self.refs()?;
        let touching = self.touching(&refs)?;
        let base = self.reader.item(&self.base_tip, id)?;
        let elsewhere = self.elsewhere(&touching, id)?;
        self.located(&refs, id, base, elsewhere)
    }

    /// The item `id` as a planning command finds it: its record in this
    /// checkout, where planning is written before it is committed, and its
    /// branch.
    pub fn locate_planned(&mut self, id: &str) -> Result<Located> {
        let refs = self.refs()?;
        let base = self.checkout_item(id)?;
        self.located(&refs, id, base, Vec::new())
    }

    /// The branch named for the item or mission `id`.
    pub fn branch_of(&self, id: &str) -> String {
        self.config.record_branch(id)
    }

    /// The remote `start`, `submit` and `merge` push to: the forge's.
    pub fn remote(&self) -> Option<&str> {
        self.config
            .forge
            .as_ref()
            .map(|forge| forge.remote.as_str())
    }

    /// Every branch the convention names, local and the forge's
    /// remote-tracking ones, with whether each is merged into the base.
    fn refs(&self) -> Result<Refs> {
        let remote = self.remote().map(str::to_string);
        let listed = git::refs(&self.root, remote.as_deref(), &self.base_tip)?;
        let tracking = remote
            .as_ref()
            .map(|remote| format!("refs/remotes/{remote}/"));
        let mut pairs: BTreeMap<String, Pair> = BTreeMap::new();
        for listed in listed {
            let (name, remote) = match listed.name.strip_prefix("refs/heads/") {
                Some(name) => (name.to_string(), false),
                None => match tracking
                    .as_deref()
                    .and_then(|t| listed.name.strip_prefix(t))
                {
                    Some(name) => (name.to_string(), true),
                    None => continue,
                },
            };
            if name == self.base {
                continue;
            }
            let named = self.config.named(&name);
            if named.is_none() && !remote {
                // Kept for a lookup by name: a record's own branch.
            } else if named.is_none() {
                continue;
            }
            let pair = pairs.entry(name.clone()).or_insert(Pair {
                name,
                named,
                local: None,
                remote: None,
            });
            match remote {
                true => pair.remote = Some(listed),
                false => pair.local = Some(listed),
            }
        }
        Ok(Refs { pairs, remote })
    }

    /// The tip of `pair` that is read: the one that contains the other, or
    /// where they have diverged, the local one.
    fn tip(&self, pair: &Pair) -> Result<Tip> {
        let tip = |found: &git::Ref, remote: bool, diverged: bool| Tip {
            tip: found.tip.clone(),
            remote,
            merged: found.merged,
            diverged,
        };
        Ok(match (&pair.local, &pair.remote) {
            (Some(local), None) => tip(local, false, false),
            (None, Some(remote)) => tip(remote, true, false),
            (Some(local), Some(remote)) if local.tip == remote.tip => tip(local, false, false),
            (Some(local), Some(remote)) => {
                if git::is_ancestor(&self.root, &local.tip, &remote.tip)? {
                    // The remote is ahead, as after a fetch of another
                    // clone's take-over or drop.
                    tip(remote, true, false)
                } else if git::is_ancestor(&self.root, &remote.tip, &local.tip)? {
                    tip(local, false, false)
                } else {
                    tip(local, false, true)
                }
            }
            (None, None) => unreachable!("a listed branch has a ref"),
        })
    }

    /// `record`'s copy at `tip` of the branch `name`: from the checkout's
    /// files where it is the local branch checked out here.
    fn read<R: Stored>(&mut self, name: &str, tip: &Tip, id: &str) -> Result<Option<R>> {
        let here = !self.committed && !tip.remote && self.branch.as_deref() == Some(name);
        match here {
            true => R::in_checkout(self, id),
            false => R::at(&mut self.reader, &tip.tip, id),
        }
    }

    /// `id`, found as a record of kind `R`, from its base copy `base`, the
    /// branch named for it among `refs`, and its copies `elsewhere`.
    fn located<R: Stored>(
        &mut self,
        refs: &Refs,
        id: &str,
        base: Option<R>,
        elsewhere: Vec<Branch<R>>,
    ) -> Result<Located<R>> {
        // A final status on the base wins, so the branch's record is not
        // read, and one that does not parse cannot hide it.
        let terminal = base.as_ref().is_some_and(R::finished);
        let name = self.branch_of(id);
        let branch = match refs.pairs.get(&name) {
            None => None,
            Some(pair) => {
                let mut tip = self.tip(pair)?;
                if tip.diverged && !terminal {
                    // Diverged: a drop on either wins, else the local tip.
                    let remote = pair.remote.as_ref().expect("diverged from a remote");
                    let at = |repo: &mut Self, tip: &str| -> Result<bool> {
                        Ok(R::at(&mut repo.reader, tip, id)?.is_some_and(|r| r.is_dropped()))
                    };
                    if at(self, &remote.tip)? && !at(self, &tip.tip)? {
                        tip = Tip {
                            tip: remote.tip.clone(),
                            remote: true,
                            merged: remote.merged,
                            diverged: true,
                        };
                    }
                }
                let record = match terminal {
                    true => None,
                    false => self.read(&name, &tip, id)?,
                };
                Some(Branch {
                    name: shown(refs, &name, tip.remote),
                    merged: tip.merged,
                    tip: tip.tip,
                    remote: tip.remote,
                    diverged: tip.diverged,
                    record,
                })
            }
        };
        Ok(Located {
            id: id.to_string(),
            base,
            branch,
            elsewhere,
        })
    }

    /// The unmerged named branches, each with the records it changes since
    /// it forked from the base, by kind and ID, less its own: one `git
    /// diff` for each, and none for a branch that changes no record.
    fn touching(&self, refs: &Refs) -> Result<Vec<Touching>> {
        let mut found = Vec::new();
        for pair in refs.pairs.values() {
            let Some(named) = &pair.named else {
                continue;
            };
            // On the branch checked out here its files count, committed or
            // not, so a record `new` just wrote is found before any commit.
            let here = !self.committed && self.branch.as_deref() == Some(pair.name.as_str());
            let (tip, paths) = match (here, &pair.local) {
                (true, Some(local)) => {
                    let tip = Tip {
                        tip: local.tip.clone(),
                        remote: false,
                        merged: local.merged,
                        diverged: false,
                    };
                    let fork = git::run(&self.root, &["merge-base", &self.base_tip, &local.tip])?;
                    let dirs = [".atelier/items", ".atelier/missions"];
                    (tip, git::changed_since(&self.root, &fork, &dirs)?)
                }
                _ => {
                    let tip = self.tip(pair)?;
                    if tip.merged {
                        continue;
                    }
                    let cached = self.changed.borrow().get(&tip.tip).cloned();
                    let paths = match cached {
                        Some(paths) => paths,
                        None => {
                            let paths = git::changed_records(&self.root, &self.base_tip, &tip.tip)?;
                            let mut changed = self.changed.borrow_mut();
                            changed.entry(tip.tip.clone()).or_insert(paths).clone()
                        }
                    };
                    (tip, paths)
                }
            };
            let own = match named {
                Named::Record(id) => Some(id.as_str()),
                Named::Backlog(_) => None,
            };
            let mut ids = BTreeSet::new();
            for path in paths {
                let kinds = [(true, ItemDir::DIR), (false, MissionDir::DIR)];
                for (item, dir) in kinds {
                    let id = path
                        .strip_prefix(dir)
                        .and_then(|rest| rest.split('/').next());
                    if let Some(id) = id.filter(|id| Some(*id) != own) {
                        ids.insert((item, id.to_string()));
                    }
                }
            }
            if !ids.is_empty() {
                let branch = pair.name.clone();
                found.push(Touching { branch, tip, ids });
            }
        }
        Ok(found)
    }

    /// The copies of `id` on the branches of `touching` that change it.
    fn elsewhere<R: Stored>(&mut self, touching: &[Touching], id: &str) -> Result<Vec<Branch<R>>> {
        let item = R::DIR == ItemDir::DIR;
        let mut found = Vec::new();
        for on in touching {
            if !on.ids.contains(&(item, id.to_string())) {
                continue;
            }
            let record = self.read(&on.branch, &on.tip, id)?;
            found.push(Branch {
                name: shown_on(self.remote(), &on.branch, on.tip.remote),
                merged: false,
                tip: on.tip.tip.clone(),
                remote: on.tip.remote,
                diverged: on.tip.diverged,
                record,
            });
        }
        Ok(found)
    }

    /// Every record of kind `R` on the base, on a branch named for one, and
    /// on an unmerged named branch that adds one.
    fn all<R: Stored>(&mut self, base_ids: Vec<String>) -> Result<Vec<Located<R>>> {
        let refs = self.refs()?;
        let touching = self.touching(&refs)?;
        let item = R::DIR == ItemDir::DIR;
        let mut ids: BTreeSet<String> = base_ids.into_iter().collect();
        ids.extend(refs.pairs.values().filter_map(|pair| match &pair.named {
            Some(Named::Record(id)) => Some(id.clone()),
            _ => None,
        }));
        for touched in &touching {
            let of_kind = touched.ids.iter().filter(|(kind, _)| *kind == item);
            ids.extend(of_kind.map(|(_, id)| id.clone()));
        }
        let mut found = Vec::new();
        for id in ids {
            let base = R::at(&mut self.reader, &self.base_tip, &id)?;
            let elsewhere = self.elsewhere(&touching, &id)?;
            let located = self.located(&refs, &id, base, elsewhere)?;
            // A branch named for an ID is an item's or a mission's: keep
            // only the kind it holds.
            if item || located.base.is_some() || located.record().is_some() {
                found.push(located);
            }
        }
        Ok(found)
    }

    /// What the convention names the checked-out branch for, if anything.
    pub fn named(&self) -> Option<Named> {
        let branch = self.branch.as_deref()?;
        (branch != self.base)
            .then(|| self.config.named(branch))
            .flatten()
    }

    /// The item of the checked-out branch, when it is named for an item.
    pub fn current(&mut self) -> Result<Option<Located>> {
        let Some(Named::Record(id)) = self.named() else {
            return Ok(None);
        };
        let located = self.locate(&id)?;
        Ok(located.record().is_some().then_some(located))
    }

    /// The IDs that branches are named for, local and the forge's
    /// remote-tracking ones.
    fn branch_ids(&self) -> Result<BTreeSet<String>> {
        let refs = self.refs()?;
        Ok(refs
            .pairs
            .into_values()
            .filter_map(|pair| match pair.named {
                Some(Named::Record(id)) => Some(id),
                _ => None,
            })
            .collect())
    }

    /// Every item on the base, on a branch named for one, and on an
    /// unmerged named branch that adds or changes one.
    pub fn everything(&mut self) -> Result<Vec<Located>> {
        let ids = self.reader.item_ids(&self.base_tip)?;
        self.all(ids)
    }

    /// Every item in this checkout and on a branch named for one, as a
    /// planning command finds them (see [`Repo::locate_planned`]).
    pub fn planned(&mut self) -> Result<Vec<Located>> {
        let mut ids: BTreeSet<String> = self.store().ids("items")?.into_iter().collect();
        ids.extend(self.branch_ids()?);
        ids.iter().map(|id| self.locate_planned(id)).collect()
    }

    /// Whether the commit `tip` is in the history of the base, local or as
    /// the forge's remote-tracking branch has it.
    pub fn merged_anywhere(&self, tip: &str) -> Result<bool> {
        if git::is_ancestor(&self.root, tip, &self.base_tip)? {
            return Ok(true);
        }
        let remote = match self.remote() {
            Some(remote) => git::remote_tip(&self.root, remote, &self.base)?,
            None => None,
        };
        match remote {
            Some(remote) => git::is_ancestor(&self.root, tip, &remote),
            None => Ok(false),
        }
    }

    /// Every mission as [`Repo::all_missions`] finds them, each as its
    /// record that counts.
    pub fn missions(&mut self) -> Result<Vec<MissionDir>> {
        let located = self.all_missions()?;
        Ok(located
            .iter()
            .filter_map(|located| located.record().cloned())
            .collect())
    }

    /// Every mission on the base, on its own branch, and on an unmerged
    /// named branch that adds or changes one.
    pub fn all_missions(&mut self) -> Result<Vec<Located<MissionDir>>> {
        let ids = self.reader.mission_ids(&self.base_tip)?;
        self.all(ids)
    }

    /// The mission `id` wherever it is, as `show` reads it.
    pub fn find_mission(&mut self, id: &str) -> Result<Located<MissionDir>> {
        let refs = self.refs()?;
        let touching = self.touching(&refs)?;
        let base = self.reader.mission(&self.base_tip, id)?;
        let elsewhere = self.elsewhere(&touching, id)?;
        self.located(&refs, id, base, elsewhere)
    }

    /// A new ID that no item or mission has, in this checkout, at the
    /// base's tip or as a branch, local or the forge's.
    pub fn new_id(&mut self) -> Result<String> {
        let store = self.store();
        let mut taken: BTreeSet<String> = store.ids("items")?.into_iter().collect();
        taken.extend(store.ids("missions")?);
        taken.extend(self.reader.item_ids(&self.base_tip)?);
        taken.extend(self.reader.mission_ids(&self.base_tip)?);
        taken.extend(self.branch_ids()?);
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
        let (mut mission, lead) = match &item.item.mission {
            None => (None, None),
            Some(id) => {
                let found = self.reader.mission(&self.base_tip, id)?;
                let lead = found.as_ref().and_then(|dir| dir.mission.lead.clone());
                (mission_of(item, found.as_slice()), lead)
            }
        };
        // A stop is published once it is on the base, the forge's or this
        // clone's: a clone that has fetched it refuses the mission's items
        // before its own base has moved.
        if let Some((id, status)) = mission
            .as_mut()
            .filter(|(_, s)| *s != Some(Status::Dropped))
        {
            if self.dropped_on_forge(id)? {
                *status = Some(Status::Dropped);
            }
        }
        Ok(Links {
            blockers,
            mission,
            lead,
        })
    }

    /// Whether the forge's base, as this clone last fetched it, holds
    /// mission `id` dropped.
    pub fn dropped_on_forge(&mut self, id: &str) -> Result<bool> {
        let Some(remote) = self.remote().map(str::to_string) else {
            return Ok(false);
        };
        let Some(tip) = git::remote_tip(&self.root, &remote, &self.base)? else {
            return Ok(false);
        };
        if tip == self.base_tip {
            return Ok(false);
        }
        let found = self.reader.mission(&tip, id)?;
        Ok(found.is_some_and(|dir| dir.mission.status == atelier_core::MissionStatus::Dropped))
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

/// The ref `name` is read from, as it is shown: the branch's name, or
/// `<remote>/<branch>` for a remote-tracking one.
fn shown(refs: &Refs, name: &str, remote: bool) -> String {
    shown_on(refs.remote.as_deref(), name, remote)
}

fn shown_on(remote: Option<&str>, name: &str, tracking: bool) -> String {
    match (remote, tracking) {
        (Some(remote), true) => format!("{remote}/{name}"),
        _ => name.to_string(),
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
