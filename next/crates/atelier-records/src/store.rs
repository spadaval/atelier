use crate::{body, document, ids, Problem};
use atelier_core::{
    Config, Decision, Finding, Item, Mission, Response, Review, ReviewName, Settlement,
};
use serde::de::DeserializeOwned;
use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::path::{Path, PathBuf};

/// A checkout's `.atelier/` directory: `config.toml`,
/// `missions/<id>/mission.md`, and `items/<id>/item.md` with its
/// `reviews/<round>-<suffix>.md`.
pub struct Store {
    root: PathBuf,
}

/// An item record and its reviews, oldest round first.
#[derive(Debug, Clone)]
pub struct ItemDir {
    pub path: PathBuf,
    pub item: Item,
    pub reviews: Vec<ReviewFile>,
}

/// A mission record and its plan reviews, oldest round first. The other
/// files of its directory are planning documents with no required shape,
/// and are not read.
#[derive(Debug, Clone)]
pub struct MissionDir {
    pub path: PathBuf,
    pub mission: Mission,
    pub reviews: Vec<ReviewFile>,
}

#[derive(Debug, Clone)]
pub struct ReviewFile {
    pub path: PathBuf,
    pub name: ReviewName,
    pub review: Review,
}

impl Store {
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Store { root: root.into() }
    }

    pub fn config(&self) -> Result<Config, Problem> {
        let path = self.root.join("config.toml");
        parse_config(&path, &read(&path)?)
    }

    pub fn item(&self, id: &str) -> Result<ItemDir, Problem> {
        let dir = self.item_dir(id);
        let path = dir.join("item.md");
        let item = parse_item(&path, &read(&path)?)?;
        let reviews = self.reviews(&dir.join("reviews"))?;
        Ok(ItemDir {
            path,
            item,
            reviews,
        })
    }

    /// Every item, by ID.
    pub fn items(&self) -> Result<Vec<ItemDir>, Problem> {
        self.ids("items")?.iter().map(|id| self.item(id)).collect()
    }

    pub fn mission(&self, id: &str) -> Result<MissionDir, Problem> {
        let path = self.mission_path(id);
        let mission = parse_mission(&path, &read(&path)?)?;
        let reviews = self.reviews(&self.root.join("missions").join(id).join("reviews"))?;
        Ok(MissionDir {
            path,
            mission,
            reviews,
        })
    }

    /// Every mission, by ID.
    pub fn missions(&self) -> Result<Vec<MissionDir>, Problem> {
        self.ids("missions")?
            .iter()
            .map(|id| self.mission(id))
            .collect()
    }

    /// The names of the directories under `kind`, `items` or `missions`,
    /// sorted.
    pub fn ids(&self, kind: &str) -> Result<Vec<String>, Problem> {
        let mut ids = Vec::new();
        for path in entries(&self.root.join(kind))? {
            if path.is_dir() {
                ids.push(
                    path.file_name()
                        .unwrap_or_default()
                        .to_string_lossy()
                        .into_owned(),
                );
            }
        }
        ids.sort();
        Ok(ids)
    }

    pub fn write_item(&self, item: &Item) -> io::Result<()> {
        let dir = self.item_dir(&item.id);
        fs::create_dir_all(&dir)?;
        fs::write(dir.join("item.md"), render_item(item))
    }

    pub fn write_mission(&self, mission: &Mission) -> io::Result<()> {
        let path = self.mission_path(&mission.id);
        fs::create_dir_all(path.parent().expect("a mission's file is in its directory"))?;
        fs::write(path, render_mission(mission))
    }

    fn item_dir(&self, id: &str) -> PathBuf {
        self.root.join("items").join(id)
    }

    fn mission_path(&self, id: &str) -> PathBuf {
        self.root.join("missions").join(id).join("mission.md")
    }

    fn reviews(&self, dir: &Path) -> Result<Vec<ReviewFile>, Problem> {
        let mut reviews = Vec::new();
        for path in entries(dir)? {
            let Some(stem) = path
                .file_name()
                .unwrap_or_default()
                .to_str()
                .and_then(|n| n.strip_suffix(".md"))
                .map(str::to_owned)
            else {
                continue;
            };
            let text = read(&path)?;
            reviews.push(review_file(path, &stem, &text)?);
        }
        reviews.sort_by(|a, b| a.name.cmp(&b.name));
        Ok(reviews)
    }
}

/// Writes a new review into `reviews/` of the item or mission directory
/// `dir`, under a file name no other review of its round has, and returns
/// that name.
pub fn create_review(dir: &Path, review: &Review) -> io::Result<ReviewName> {
    let dir = dir.join("reviews");
    fs::create_dir_all(&dir)?;
    for _ in 0..ids::MAX_ALLOC_ATTEMPTS {
        let name = ReviewName {
            round: review.round,
            suffix: ids::random_suffix()?,
        };
        let file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(dir.join(format!("{name}.md")));
        match file {
            Ok(mut file) => {
                file.write_all(render_review(review).as_bytes())?;
                return Ok(name);
            }
            Err(error) if error.kind() == io::ErrorKind::AlreadyExists => continue,
            Err(error) => return Err(error),
        }
    }
    Err(io::Error::other("no free review file name"))
}

/// A review read from the file at `path`, whose name without `.md` is
/// `stem`.
pub(crate) fn review_file(path: PathBuf, stem: &str, text: &str) -> Result<ReviewFile, Problem> {
    let name = stem
        .parse()
        .map_err(|message| Problem::new(&path, "file name", message))?;
    let review = parse_review(&path, text)?;
    Ok(ReviewFile { path, name, review })
}

impl ItemDir {
    /// Each checklist line of `## Done when`: whether it is ticked, and its
    /// text.
    pub fn done_when(&self) -> Vec<(bool, String)> {
        body::done_when(&self.item.body)
    }

    /// The content under the item's `## <heading>`, if it has one.
    pub fn section(&self, heading: &str) -> Option<&str> {
        body::section(&self.item.body, 2, heading)
    }

    /// Whether `## Report` holds anything besides headings and blank lines.
    pub fn report_written(&self) -> bool {
        body::written(&self.item.body, "Report")
    }

    /// Why the item was dropped, from the `Dropped:` line `drop` writes
    /// first under `## Report`.
    pub fn dropped(&self) -> Option<&str> {
        body::dropped(&self.item.body, "Report")
    }

    /// Every finding of every review, oldest round first.
    pub fn findings(&self) -> Result<Vec<Finding>, Problem> {
        findings(&self.reviews)
    }

    pub fn decisions(&self) -> Result<Vec<Decision>, Problem> {
        body::decisions(&self.path, &self.item.body)
    }

    pub fn responses(&self) -> Result<Vec<Response>, Problem> {
        body::responses(&self.path, &self.item.body)
    }

    /// The fix-now findings no later round or decision has closed.
    pub fn open_findings(&self) -> Result<Vec<Finding>, Problem> {
        let findings = self.findings()?;
        let decisions = self.decisions()?;
        Ok(findings
            .iter()
            .filter(|f| atelier_core::is_open(f, &findings, &decisions))
            .cloned()
            .collect())
    }
}

impl MissionDir {
    /// The content under the mission's `## <heading>`, if it has one.
    pub fn section(&self, heading: &str) -> Option<&str> {
        body::section(&self.mission.body, 2, heading)
    }

    /// Whether `## Result` holds anything besides headings and blank lines.
    pub fn result_written(&self) -> bool {
        body::written(&self.mission.body, "Result")
    }

    /// Why the mission was dropped, from the `Dropped:` line `drop` writes
    /// first under `## Result`.
    pub fn dropped(&self) -> Option<&str> {
        body::dropped(&self.mission.body, "Result")
    }

    /// Every finding of every plan review, oldest round first.
    pub fn findings(&self) -> Result<Vec<Finding>, Problem> {
        findings(&self.reviews)
    }
}

fn findings(reviews: &[ReviewFile]) -> Result<Vec<Finding>, Problem> {
    let mut findings = Vec::new();
    for review in reviews {
        findings.extend(review.findings()?);
    }
    Ok(findings)
}

impl ReviewFile {
    /// Writes the review back to its file.
    pub fn write(&self) -> io::Result<()> {
        fs::write(&self.path, render_review(&self.review))
    }

    pub fn findings(&self) -> Result<Vec<Finding>, Problem> {
        body::findings(&self.path, &self.name, &self.review.body)
    }

    /// Records `settlement` in finding `number`, on the line after its
    /// Recommendation; false when the review has no such finding.
    pub fn settle(&mut self, number: u32, settlement: &Settlement) -> bool {
        match body::settle(&self.review.body, number, settlement) {
            Some(body) => {
                self.review.body = body;
                true
            }
            None => false,
        }
    }

    /// The text under the heading of finding `number`, as the reviewer
    /// wrote it.
    pub fn finding_text(&self, number: u32) -> Option<&str> {
        let prefix = format!("Finding {number}: ");
        document::sections(&self.review.body, 2)
            .into_iter()
            .find(|(heading, _)| heading.starts_with(&prefix))
            .map(|(_, content)| content)
    }
}

pub fn parse_item(path: &Path, text: &str) -> Result<Item, Problem> {
    let (mut item, body): (Item, _) = parse_front(path, text)?;
    item.body = body.to_string();
    Ok(item)
}

pub fn render_item(item: &Item) -> String {
    document::join(&yaml(item), &item.body)
}

pub fn parse_mission(path: &Path, text: &str) -> Result<Mission, Problem> {
    let (mut mission, body): (Mission, _) = parse_front(path, text)?;
    mission.body = body.to_string();
    Ok(mission)
}

pub fn render_mission(mission: &Mission) -> String {
    document::join(&yaml(mission), &mission.body)
}

pub fn parse_review(path: &Path, text: &str) -> Result<Review, Problem> {
    let (mut review, body): (Review, _) = parse_front(path, text)?;
    review.body = body.to_string();
    Ok(review)
}

pub fn render_review(review: &Review) -> String {
    document::join(&yaml(review), &review.body)
}

pub fn parse_config(path: &Path, text: &str) -> Result<Config, Problem> {
    let config: Config = toml::from_str(text).map_err(|error| {
        let line = error
            .span()
            .map(|span| text[..span.start].matches('\n').count() + 1)
            .unwrap_or(1);
        Problem::new(path, format!("line {line}"), error.message().to_string())
    })?;
    ids::validate_slug(&config.slug).map_err(|message| Problem::new(path, "slug", message))?;
    Ok(config)
}

pub fn render_config(config: &Config) -> String {
    toml::to_string(config).expect("a config always serializes")
}

fn parse_front<'a, T: DeserializeOwned>(
    path: &Path,
    text: &'a str,
) -> Result<(T, &'a str), Problem> {
    let (front_matter, body) = document::split(text).ok_or_else(|| {
        Problem::new(
            path,
            "front matter",
            "the file does not start with front matter between `---` lines",
        )
    })?;
    let record = serde_yaml::from_str(front_matter)
        .map_err(|error| Problem::new(path, "front matter", error.to_string()))?;
    Ok((record, body))
}

fn yaml(record: &impl serde::Serialize) -> String {
    serde_yaml::to_string(record).expect("a record always serializes")
}

fn read(path: &Path) -> Result<String, Problem> {
    fs::read_to_string(path).map_err(|error| Problem::new(path, "", error.to_string()))
}

/// The entries of `dir`, or none when it does not exist.
fn entries(dir: &Path) -> Result<Vec<PathBuf>, Problem> {
    let listing = match fs::read_dir(dir) {
        Ok(listing) => listing,
        Err(error) if error.kind() == io::ErrorKind::NotFound => return Ok(Vec::new()),
        Err(error) => return Err(Problem::new(dir, "", error.to_string())),
    };
    listing
        .map(|entry| {
            entry
                .map(|e| e.path())
                .map_err(|error| Problem::new(dir, "", error.to_string()))
        })
        .collect()
}
