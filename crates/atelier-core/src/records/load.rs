//! Every record among a set of files, read one file at a time, so that a
//! file that does not read leaves the others read: what `check` and the
//! pre-commit hook examine.

use crate::records::store::review_file;
use crate::records::ReviewFile;
use crate::records::{
    parse_config, parse_item, parse_mission, validate, ItemDir, MissionDir, Problem,
};
use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

/// The config's path from the repository's root.
pub const CONFIG: &str = ".atelier/config.toml";

/// What [`load`] read.
#[derive(Debug, Default)]
pub struct Loaded {
    pub items: Vec<ItemDir>,
    pub missions: Vec<MissionDir>,
    /// A problem for each file that does not read. This breaks no rule:
    /// there is nothing to check a rule against.
    pub unread: Vec<Problem>,
    /// What [`validate`] finds in the records that read.
    pub invalid: Vec<Problem>,
}

/// Where a record file sits under `.atelier/`.
enum Place<'a> {
    Config,
    Item(&'a str),
    Mission(&'a str),
    /// A review of the item, or with `mission` the mission, `id`.
    Review {
        mission: bool,
        id: &'a str,
        stem: &'a str,
    },
}

fn place(path: &str) -> Option<Place<'_>> {
    let parts: Vec<&str> = path.strip_prefix(".atelier/")?.split('/').collect();
    match parts.as_slice() {
        ["config.toml"] => Some(Place::Config),
        ["items", id, "item.md"] => Some(Place::Item(id)),
        ["missions", id, "mission.md"] => Some(Place::Mission(id)),
        [kind @ ("items" | "missions"), id, "reviews", name] => Some(Place::Review {
            mission: *kind == "missions",
            id,
            stem: name.strip_suffix(".md")?,
        }),
        _ => None,
    }
}

/// Whether the file at `path`, from the repository's root, is a record:
/// `config.toml`, an `item.md`, a `mission.md` or a review. A mission's
/// documents and an item's artifacts are not.
pub fn is_record(path: &str) -> bool {
    place(path).is_some()
}

/// The records among `files`, each a path from the repository's root and
/// its content, and what is wrong with them. Files that are not records are
/// passed over. A record's problems name it by its path.
pub fn load(files: impl IntoIterator<Item = (String, Vec<u8>)>) -> Loaded {
    type Dir<T> = (Option<T>, Vec<ReviewFile>);
    let mut items: BTreeMap<String, Dir<_>> = BTreeMap::new();
    let mut missions: BTreeMap<String, Dir<_>> = BTreeMap::new();
    let mut present = BTreeSet::new();
    let mut unread = Vec::new();
    for (path, bytes) in files {
        let Some(place) = place(&path) else { continue };
        let file = Path::new(&path);
        present.insert(path.clone());
        let Ok(text) = String::from_utf8(bytes) else {
            unread.push(Problem::new(file, "", "the file is not UTF-8"));
            continue;
        };
        let read = match place {
            Place::Config => parse_config(file, &text).map(drop),
            Place::Item(id) => parse_item(file, &text).map(|item| {
                items.entry(id.to_string()).or_default().0 = Some(item);
            }),
            Place::Mission(id) => parse_mission(file, &text).map(|mission| {
                missions.entry(id.to_string()).or_default().0 = Some(mission);
            }),
            Place::Review { mission, id, stem } => review_file(file.to_path_buf(), stem, &text)
                .map(|review| {
                    let id = id.to_string();
                    match mission {
                        true => missions.entry(id).or_default().1.push(review),
                        false => items.entry(id).or_default().1.push(review),
                    }
                }),
        };
        unread.extend(read.err());
    }
    if !present.contains(CONFIG) {
        unread.push(Problem::new(Path::new(CONFIG), "", "the file is missing"));
    }
    // A review is validated with the record it belongs to, so one whose
    // record is missing or does not read is a problem of its own, named by
    // its path, as a commit of it is judged.
    let mut orphans = |owner: PathBuf, reviews: Vec<ReviewFile>| {
        let shown = owner.to_string_lossy().into_owned();
        let message = match present.contains(&shown) {
            true => format!("not validated, since {shown} does not read"),
            false => format!("{shown} is missing"),
        };
        for review in reviews {
            unread.push(Problem::new(&review.path, "", message.clone()));
        }
    };
    let mut loaded = Loaded::default();
    for (id, (item, mut reviews)) in items {
        reviews.sort_by(|a, b| a.name.cmp(&b.name));
        let path = PathBuf::from(format!(".atelier/items/{id}/item.md"));
        match item {
            Some(item) => loaded.items.push(ItemDir {
                path,
                item,
                reviews,
            }),
            None => orphans(path, reviews),
        }
    }
    for (id, (mission, mut reviews)) in missions {
        reviews.sort_by(|a, b| a.name.cmp(&b.name));
        let path = PathBuf::from(format!(".atelier/missions/{id}/mission.md"));
        match mission {
            Some(mission) => loaded.missions.push(MissionDir {
                path,
                mission,
                reviews,
            }),
            None => orphans(path, reviews),
        }
    }
    loaded.unread = unread;
    loaded.invalid = validate(&loaded.items, &loaded.missions);
    loaded
}
