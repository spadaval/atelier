use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{anyhow, bail, Context, Result};

use crate::storage_layout::StorageLayout;

pub fn project_config_toml(project_slug: &str) -> String {
    format!(
        r#"schema = "atelier.project_config"
schema_version = 1
project_slug = "{project_slug}"

[paths]
state_root = ".atelier"

[prune]
canonical_retention_days = 7
"#
    )
}

/// The command that commits a new install on the base, where `land`
/// exempts it from review because it predates the gates.
pub const INSTALL_COMMIT: &str = "git add .atelier .gitignore && git commit -m \"Install Atelier\"";

pub const ROOT_GITIGNORE_ENTRIES: &[&str] = &[
    "/.atelier/agent.json",
    "/.atelier/runtime/",
    "/.atelier-bundle-stage-*/",
];

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InitRequest {
    pub path: PathBuf,
    pub force: bool,
    /// Overrides the slug derived from the origin remote or main worktree.
    pub project_slug: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InitView {
    pub created_paths: Vec<PathBuf>,
    /// Existing files init added to, such as `.gitignore`.
    pub updated_paths: Vec<PathBuf>,
    /// The slug written to a new config, and where it came from.
    pub project_slug: Option<(String, &'static str)>,
    /// The base branch a new workflow policy names, and why init chose it.
    pub base_branch: Option<(String, &'static str)>,
    /// The checkout's branch, born or not; `None` when HEAD is detached.
    pub current_branch: Option<String>,
}

pub fn initialize(input: InitRequest) -> Result<InitView> {
    // Atelier's records live in Git, and its base branch is a Git branch.
    let inside =
        crate::git_freshness::git_output(&input.path, &["rev-parse", "--is-inside-work-tree"])
            .is_ok_and(|out| out.trim_ascii() == b"true");
    if !inside {
        bail!(
            "init_no_git: {} is not in a Git repository, and Atelier keeps its records in Git. Next: run `git init`, then `atelier init`",
            input.path.display()
        );
    }
    let current_branch = git_text(&input.path, &["symbolic-ref", "--quiet", "--short", "HEAD"]);
    let layout = StorageLayout::new(&input.path);
    let atelier_dir = layout.atelier_dir();
    let mut created_paths = Vec::new();
    let mut project_slug = None;
    let project_config_path = layout.config_path();
    let new_config = if project_config_path.exists() {
        if let Some(requested) = &input.project_slug {
            let existing = crate::project_config::ProjectConfig::load(&input.path)?.project_slug;
            if *requested != existing {
                bail!(
                    "{} already sets project_slug = \"{existing}\"; edit it there instead of passing --slug",
                    project_config_path.display()
                );
            }
        }
        None
    } else {
        let (slug, source) = match &input.project_slug {
            Some(slug) => (slug.clone(), "--slug"),
            None => default_project_slug(&input.path)?,
        };
        atelier_records::validate_project_slug(&slug)?;
        project_slug = Some((slug.clone(), source));
        Some(project_config_toml(&slug))
    };

    let workflow_path = input
        .path
        .join(crate::workflow_policy::WORKFLOW_POLICY_PATH);
    let new_policy = if workflow_path.exists() {
        None
    } else {
        let (base, source) = base_branch(&input.path, current_branch.as_deref())?;
        let text = crate::workflow_policy::starter_policy_yaml(&base);
        atelier_workflow::parse_policy_text(&text, crate::workflow_policy::WORKFLOW_POLICY_PATH)
            .with_context(|| format!("cannot write a workflow policy naming {base} as the base branch. Next: check out the base branch, then rerun `atelier init`"))?;
        Some((text, (base, source)))
    };

    if !atelier_dir.exists() {
        fs::create_dir_all(&atelier_dir).context("Failed to create .atelier directory")?;
        created_paths.push(atelier_dir.clone());
    }

    for dir in atelier_records::canonical_record_dirs() {
        fs::create_dir_all(atelier_dir.join(dir))
            .with_context(|| format!("Failed to create .atelier/{} directory", dir))?;
    }

    if let Some(config) = new_config {
        fs::write(&project_config_path, config).context("Failed to write .atelier/config.toml")?;
        created_paths.push(project_config_path);
    }

    let base_branch = match new_policy {
        Some((text, base)) => {
            fs::write(&workflow_path, text).context("Failed to write .atelier/workflow.yaml")?;
            created_paths.push(workflow_path);
            Some(base)
        }
        None => None,
    };

    let gitignore = input.path.join(".gitignore");
    let gitignore_existed = gitignore.exists();
    let mut updated_paths = Vec::new();
    if ensure_root_gitignore(&input.path, input.force)? {
        if gitignore_existed {
            updated_paths.push(gitignore);
        } else {
            created_paths.push(gitignore);
        }
    }

    Ok(InitView {
        created_paths,
        updated_paths,
        project_slug,
        base_branch,
        current_branch,
    })
}

/// The trimmed output of a Git command, or `None` when it fails or prints
/// nothing.
fn git_text(path: &Path, args: &[&str]) -> Option<String> {
    crate::git_freshness::git_output(path, args)
        .ok()
        .and_then(|out| String::from_utf8(out).ok())
        .map(|out| out.trim().to_owned())
        .filter(|out| !out.is_empty())
}

/// The base branch a new policy names, and why: origin's HEAD branch, else
/// whichever one of `main` and `master` exists, else the current branch,
/// born or not (atelier-l457). Refuses only a detached HEAD that no other
/// rule settles, before anything is written.
fn base_branch(path: &Path, current: Option<&str>) -> Result<(String, &'static str)> {
    if let Some(remote) = git_text(
        path,
        &[
            "symbolic-ref",
            "--quiet",
            "--short",
            "refs/remotes/origin/HEAD",
        ],
    ) {
        if let Some(branch) = remote
            .strip_prefix("origin/")
            .filter(|branch| !branch.is_empty())
        {
            return Ok((branch.to_owned(), "origin's HEAD branch"));
        }
    }
    let exists = |branch: &str| {
        crate::git_freshness::git_output(
            path,
            &[
                "rev-parse",
                "--verify",
                "--quiet",
                &format!("refs/heads/{branch}"),
            ],
        )
        .is_ok()
    };
    match (exists("main"), exists("master")) {
        (true, false) => return Ok(("main".to_owned(), "the existing main branch")),
        (false, true) => return Ok(("master".to_owned(), "the existing master branch")),
        _ => {}
    }
    match current {
        Some(branch) => Ok((branch.to_owned(), "the current branch")),
        None => bail!(
            "init_base_unknown: HEAD is detached, origin names no HEAD branch, and there is not exactly one of main and master, so init cannot tell which branch is the base. Next: check out the base branch, then rerun `atelier init`"
        ),
    }
}

/// The repository's name, normalized to a valid slug: the origin remote's
/// repository name, else the main worktree's directory, so a linked worktree
/// such as `project-feature` still mints `project-*` IDs. Git names count
/// only at a repository's top level; elsewhere, such as a new project inside
/// a tracked parent, the directory name is used.
fn default_project_slug(path: &Path) -> Result<(String, &'static str)> {
    let path =
        fs::canonicalize(path).with_context(|| format!("failed to resolve {}", path.display()))?;
    let git = |args: &[&str]| {
        crate::git_freshness::git_output(&path, args)
            .ok()
            .and_then(|out| String::from_utf8(out).ok())
            .map(|out| out.trim().to_owned())
            .filter(|out| !out.is_empty())
    };
    let top_level = git(&["rev-parse", "--show-toplevel"])
        .and_then(|top| fs::canonicalize(top).ok())
        .is_some_and(|top| top == path);
    let remote = top_level
        .then(|| git(&["remote", "get-url", "origin"]))
        .flatten()
        .and_then(|url| repository_name(&url));
    let common = top_level
        .then(|| git(&["rev-parse", "--path-format=absolute", "--git-common-dir"]))
        .flatten()
        .map(PathBuf::from);
    // A non-bare repository keeps its common dir at <main worktree>/.git.
    let main_worktree = common
        .as_ref()
        .filter(|common| common.file_name().is_some_and(|name| name == ".git"))
        .and_then(|common| common.parent()?.file_name())
        .map(|name| name.to_string_lossy().into_owned());
    // Worktrees of a bare repository are named after it (`proj.git`), or
    // after its parent for a hidden layout such as `proj/.bare`.
    let bare = common
        .as_ref()
        .filter(|common| {
            crate::git_freshness::git_output(
                &path,
                &[
                    "--git-dir",
                    &common.to_string_lossy(),
                    "rev-parse",
                    "--is-bare-repository",
                ],
            )
            .is_ok_and(|out| out.trim_ascii() == b"true")
        })
        .and_then(|common| {
            let hidden = common
                .file_name()
                .is_some_and(|name| name.to_string_lossy().starts_with('.'));
            let named = if hidden { common.parent()? } else { common };
            repository_name(&named.to_string_lossy())
        });
    let directory = path
        .file_name()
        .map(|name| name.to_string_lossy().into_owned());
    [
        (remote, "the origin remote"),
        (main_worktree, "the main worktree's directory"),
        (bare, "the bare repository's name"),
        (directory, "the directory name"),
    ]
    .into_iter()
    .find_map(|(name, source)| {
        name.and_then(|name| atelier_records::derive_project_slug(&name))
            .map(|slug| (slug, source))
    })
    .ok_or_else(|| {
        anyhow!(
            "cannot derive a project slug from {}; pass --slug <slug>",
            path.display()
        )
    })
}

/// The last component of a remote URL or repository path, without `.git`.
fn repository_name(location: &str) -> Option<String> {
    let location = location.trim_end_matches('/');
    let location = location
        .strip_suffix("/.git")
        .unwrap_or(location)
        .trim_end_matches('/');
    let name = location.rsplit(['/', ':']).next()?;
    let name = match name.len().checked_sub(4) {
        Some(at) if name.is_char_boundary(at) && name[at..].eq_ignore_ascii_case(".git") => {
            &name[..at]
        }
        _ => name,
    };
    (!name.is_empty()).then(|| name.to_owned())
}

/// Adds Atelier's local paths to the root `.gitignore`, and says whether it
/// wrote the file.
pub fn ensure_root_gitignore(path: &Path, force: bool) -> Result<bool> {
    let gitignore_path = path.join(".gitignore");
    let mut existing = match fs::read_to_string(&gitignore_path) {
        Ok(content) => content,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => String::new(),
        Err(e) => return Err(anyhow::Error::from(e).context("Failed to read .gitignore")),
    };
    let mut changed = force && !gitignore_path.exists();
    if !existing.is_empty() && !existing.ends_with('\n') {
        existing.push('\n');
        changed = true;
    }

    let missing = ROOT_GITIGNORE_ENTRIES
        .iter()
        .copied()
        .filter(|entry| !existing.lines().any(|line| line.trim() == *entry))
        .collect::<Vec<_>>();
    if !missing.is_empty() {
        if !existing.is_empty() {
            existing.push('\n');
        }
        existing.push_str("# Atelier local runtime state\n");
        for entry in missing {
            existing.push_str(entry);
            existing.push('\n');
        }
        changed = true;
    }

    if changed {
        fs::write(&gitignore_path, existing).context("Failed to write .gitignore")?;
    }
    Ok(changed)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn git(dir: &Path, args: &[&str]) {
        crate::git_freshness::git_output(dir, args)
            .unwrap_or_else(|error| panic!("git {args:?}: {error:#}"));
    }

    /// A repository on `branch` with one commit, holding the branches in
    /// `others` at that commit.
    fn repository(branch: &str, others: &[&str]) -> tempfile::TempDir {
        let dir = tempfile::tempdir().unwrap();
        git(dir.path(), &["init", "-q", "-b", branch]);
        git(dir.path(), &["config", "user.name", "b"]);
        git(dir.path(), &["config", "user.email", "b@b"]);
        git(
            dir.path(),
            &["commit", "-q", "--allow-empty", "-m", "first"],
        );
        for other in others {
            git(dir.path(), &["branch", other]);
        }
        dir
    }

    fn init(dir: &Path) -> Result<InitView> {
        initialize(InitRequest {
            path: dir.to_path_buf(),
            force: false,
            project_slug: Some("atelier".into()),
        })
    }

    fn written_base(dir: &Path) -> String {
        crate::workflow_policy::load(dir)
            .unwrap()
            .branch_policy
            .base_branch
    }

    /// The base is origin's HEAD branch, else whichever one of main and
    /// master exists, else the current branch, born or not; the policy
    /// names it and init says why (atelier-l457).
    #[test]
    fn init_detects_the_base_branch_in_order() {
        let cases: [(&str, &[&str], &str, &str); 4] = [
            (
                "feature",
                &["master"],
                "master",
                "the existing master branch",
            ),
            ("feature", &["main"], "main", "the existing main branch"),
            (
                "feature",
                &["main", "master"],
                "feature",
                "the current branch",
            ),
            ("master", &[], "master", "the existing master branch"),
        ];
        for (current, others, base, source) in cases {
            let dir = repository(current, others);
            let view = init(dir.path()).unwrap();
            assert_eq!(
                view.base_branch,
                Some((base.to_owned(), source)),
                "{current} {others:?}"
            );
            assert_eq!(written_base(dir.path()), base);
        }

        // Origin's HEAD branch comes first.
        let dir = repository("main", &["master"]);
        git(
            dir.path(),
            &["update-ref", "refs/remotes/origin/trunk", "HEAD"],
        );
        git(
            dir.path(),
            &[
                "symbolic-ref",
                "refs/remotes/origin/HEAD",
                "refs/remotes/origin/trunk",
            ],
        );
        let view = init(dir.path()).unwrap();
        assert_eq!(
            view.base_branch,
            Some(("trunk".to_owned(), "origin's HEAD branch"))
        );
        assert_eq!(written_base(dir.path()), "trunk");

        // A branch with no commits yet, as right after `git init`.
        for branch in ["master", "main", "trunk"] {
            let dir = tempfile::tempdir().unwrap();
            git(dir.path(), &["init", "-q", "-b", branch]);
            let view = init(dir.path()).unwrap();
            assert_eq!(
                view.base_branch,
                Some((branch.to_owned(), "the current branch"))
            );
            assert_eq!(view.current_branch.as_deref(), Some(branch));
            assert_eq!(written_base(dir.path()), branch);
        }

        // A re-run keeps the policy and chooses nothing.
        assert_eq!(init(dir.path()).unwrap().base_branch, None);
    }

    /// A detached HEAD that no other rule settles is refused before
    /// anything is written.
    #[test]
    fn init_refuses_a_base_it_cannot_tell_and_writes_nothing() {
        let dir = repository("main", &["master"]);
        git(dir.path(), &["switch", "-q", "--detach"]);
        let error = init(dir.path()).unwrap_err().to_string();
        assert!(error.starts_with("init_base_unknown: "), "{error}");
        assert!(!dir.path().join(".atelier").exists());
        assert!(!dir.path().join(".gitignore").exists());
    }

    #[test]
    fn repository_names_come_from_any_remote_or_path_form() {
        for (location, name) in [
            ("https://github.com/o/elite-tools.git", "elite-tools"),
            ("https://github.com/o/elite-tools/", "elite-tools"),
            ("git@github.com:o/elite-tools.git", "elite-tools"),
            ("ssh://git@host:2222/o/elite-tools.GIT", "elite-tools"),
            ("file:///srv/elite-tools.git/", "elite-tools"),
            ("/srv/elite-tools/.git", "elite-tools"),
            ("/srv/elite-tools", "elite-tools"),
        ] {
            assert_eq!(
                repository_name(location).as_deref(),
                Some(name),
                "{location}"
            );
        }
        assert_eq!(repository_name("/.git"), None);
    }
}
