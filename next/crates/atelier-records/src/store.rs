use crate::{body, document, ids, Problem};
use atelier_core::{Config, Decision, Finding, Item, Response, Review, ReviewName};
use serde::de::DeserializeOwned;
use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::path::{Path, PathBuf};

/// A checkout's `.atelier/` directory: `config.toml`, and
/// `items/<id>/item.md` with its `reviews/<round>-<suffix>.md`.
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
        let mut ids = Vec::new();
        for path in entries(&self.root.join("items"))? {
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
        ids.iter().map(|id| self.item(id)).collect()
    }

    pub fn write_item(&self, item: &Item) -> io::Result<()> {
        let dir = self.item_dir(&item.id);
        fs::create_dir_all(&dir)?;
        fs::write(dir.join("item.md"), render_item(item))
    }

    /// Writes a new review of the item, under a file name no other review
    /// of its round has, and returns that name.
    pub fn create_review(&self, item_id: &str, review: &Review) -> io::Result<ReviewName> {
        let dir = self.item_dir(item_id).join("reviews");
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

    pub fn write_review(
        &self,
        item_id: &str,
        name: &ReviewName,
        review: &Review,
    ) -> io::Result<()> {
        let path = self
            .item_dir(item_id)
            .join("reviews")
            .join(format!("{name}.md"));
        fs::write(path, render_review(review))
    }

    fn item_dir(&self, id: &str) -> PathBuf {
        self.root.join("items").join(id)
    }

    fn reviews(&self, dir: &Path) -> Result<Vec<ReviewFile>, Problem> {
        let mut reviews = Vec::new();
        for path in entries(dir)? {
            let Some(stem) = path
                .file_name()
                .unwrap_or_default()
                .to_str()
                .and_then(|n| n.strip_suffix(".md"))
            else {
                continue;
            };
            let name = stem
                .parse()
                .map_err(|message| Problem::new(&path, "file name", message))?;
            let review = parse_review(&path, &read(&path)?)?;
            reviews.push(ReviewFile { path, name, review });
        }
        reviews.sort_by(|a, b| a.name.cmp(&b.name));
        Ok(reviews)
    }
}

impl ItemDir {
    /// Every finding of every review, oldest round first.
    pub fn findings(&self) -> Result<Vec<Finding>, Problem> {
        let mut findings = Vec::new();
        for review in &self.reviews {
            findings.extend(review.findings()?);
        }
        Ok(findings)
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

impl ReviewFile {
    pub fn findings(&self) -> Result<Vec<Finding>, Problem> {
        body::findings(&self.path, &self.name, &self.review.body)
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
