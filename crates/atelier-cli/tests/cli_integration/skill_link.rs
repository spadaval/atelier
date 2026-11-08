//! `setup --link` and `--unlink` in a scratch home, laid out as `skills` 1.7.6
//! lays out a global install: real files in `~/.agents/skills/<name>`, a
//! relative symlink to them from `~/.claude/skills/<name>`, and a lock file.
use std::{
    fs,
    os::unix::fs::symlink,
    path::{Path, PathBuf},
    process::Command,
};
use tempfile::{tempdir, TempDir};

struct Home {
    root: TempDir,
    /// Extra environment, such as `CLAUDE_CONFIG_DIR`.
    env: Vec<(&'static str, PathBuf)>,
}

impl Home {
    fn new() -> Self {
        let home = Self {
            root: tempdir().unwrap(),
            env: Vec::new(),
        };
        fs::create_dir_all(home.home()).unwrap();
        home
    }
    fn home(&self) -> PathBuf {
        self.root.path().join("home")
    }
    /// A checkout holding `skills/agent-factory/SKILL.md`.
    fn checkout(&self, name: &str) -> PathBuf {
        let skill = self.root.path().join(name).join("skills/agent-factory");
        fs::create_dir_all(&skill).unwrap();
        fs::write(skill.join("SKILL.md"), "---\nname: agent-factory\n---\n").unwrap();
        self.root.path().join(name)
    }
    fn agents(&self) -> PathBuf {
        self.home().join(".agents/skills/agent-factory")
    }
    fn claude(&self) -> PathBuf {
        self.env
            .iter()
            .find(|(key, _)| *key == "CLAUDE_CONFIG_DIR")
            .map_or_else(|| self.home().join(".claude"), |(_, dir)| dir.clone())
            .join("skills/agent-factory")
    }
    fn lock(&self) -> PathBuf {
        self.env
            .iter()
            .find(|(key, _)| *key == "XDG_STATE_HOME")
            .map_or_else(
                || self.home().join(".agents"),
                |(_, dir)| dir.join("skills"),
            )
            .join(".skill-lock.json")
    }
    /// What `skills add spadaval/atelier --skill agent-factory -g` leaves,
    /// beside another installed skill.
    fn install_with_skills(&self) -> serde_json::Value {
        fs::create_dir_all(self.agents()).unwrap();
        fs::write(self.agents().join("SKILL.md"), "installed copy\n").unwrap();
        fs::create_dir_all(self.claude().parent().unwrap()).unwrap();
        symlink("../../.agents/skills/agent-factory", self.claude()).unwrap();
        let other = serde_json::json!({
            "source": "anthropics/skills",
            "sourceType": "github",
            "skillFolderHash": "3cf9a8db",
        });
        let lock = serde_json::json!({
            "version": 3,
            "skills": {
                "agent-factory": {"source": "spadaval/atelier", "sourceType": "github"},
                "skill-creator": other,
            },
            "dismissed": {},
            "lastSelectedAgents": ["claude-code"],
        });
        fs::create_dir_all(self.lock().parent().unwrap()).unwrap();
        fs::write(self.lock(), serde_json::to_vec_pretty(&lock).unwrap()).unwrap();
        lock
    }
    fn run(&self, dir: &Path, args: &[&str]) -> (bool, String, String) {
        let mut command = Command::new(env!("CARGO_BIN_EXE_atelier"));
        command
            .current_dir(dir)
            .args(args)
            .env("HOME", self.home())
            .env_remove("CLAUDE_CONFIG_DIR")
            .env_remove("XDG_STATE_HOME");
        for (key, value) in &self.env {
            command.env(key, value);
        }
        let out = command.output().unwrap();
        (
            out.status.success(),
            String::from_utf8_lossy(&out.stdout).into(),
            String::from_utf8_lossy(&out.stderr).into(),
        )
    }
    fn ok(&self, args: &[&str]) -> String {
        let (pass, stdout, stderr) = self.run(self.root.path(), args);
        assert!(pass, "atelier {args:?}: {stdout}{stderr}");
        stdout
    }
    fn refused(&self, args: &[&str]) -> String {
        let (pass, stdout, stderr) = self.run(self.root.path(), args);
        assert!(!pass, "atelier {args:?} passed: {stdout}");
        stderr
    }
    fn lock_value(&self) -> serde_json::Value {
        serde_json::from_slice(&fs::read(self.lock()).unwrap()).unwrap()
    }
}

fn target(path: &Path) -> PathBuf {
    fs::read_link(path).unwrap()
}

#[test]
fn link_replaces_hand_made_symlinks_and_reports_their_old_targets() {
    let h = Home::new();
    let checkout = h.checkout("checkout");
    let old = h.root.path().join("dotagents/skills/agent-factory");
    fs::create_dir_all(&old).unwrap();
    fs::create_dir_all(h.agents().parent().unwrap()).unwrap();
    fs::create_dir_all(h.claude().parent().unwrap()).unwrap();
    symlink(&old, h.agents()).unwrap();
    symlink(&old, h.claude()).unwrap();

    let out = h.ok(&["setup", "--link", checkout.to_str().unwrap()]);
    let skill = fs::canonicalize(&checkout)
        .unwrap()
        .join("skills/agent-factory");
    assert_eq!(target(&h.agents()), skill);
    assert_eq!(target(&h.claude()), skill);
    assert!(
        out.contains(&format!(
            "{}: was a link to {}",
            h.agents().display(),
            old.display()
        )) && out.contains(&format!(
            "{}: was a link to {}",
            h.claude().display(),
            old.display()
        )),
        "{out}"
    );
    assert!(out.contains("keep it on master"), "{out}");
    // The old target is untouched, and no lock file is made.
    assert!(old.is_dir());
    assert!(!h.lock().exists());
    // Linking again reports the links already in place.
    let out = h.ok(&["setup", "--link", checkout.to_str().unwrap()]);
    assert_eq!(out.matches("already linked").count(), 2, "{out}");
}

#[test]
fn link_replaces_a_lock_tracked_copy_and_drops_only_its_lock_entry() {
    let h = Home::new();
    let checkout = h.checkout("checkout");
    let mut lock = h.install_with_skills();

    let out = h.ok(&["setup", "--link", checkout.to_str().unwrap()]);
    let skill = fs::canonicalize(&checkout)
        .unwrap()
        .join("skills/agent-factory");
    assert_eq!(target(&h.agents()), skill);
    assert_eq!(target(&h.claude()), skill);
    assert!(
        out.contains("was a copy skills add installed, now removed")
            && out.contains("was a link to ../../.agents/skills/agent-factory")
            && out.contains("Removed agent-factory from"),
        "{out}"
    );
    lock["skills"]
        .as_object_mut()
        .unwrap()
        .remove("agent-factory");
    assert_eq!(h.lock_value(), lock);
}

#[test]
fn link_honours_claude_config_dir_and_xdg_state_home() {
    let mut h = Home::new();
    h.env
        .push(("CLAUDE_CONFIG_DIR", h.root.path().join("claude-config")));
    h.env.push(("XDG_STATE_HOME", h.root.path().join("state")));
    let checkout = h.checkout("checkout");
    let mut lock = h.install_with_skills();
    assert!(h.claude().starts_with(h.root.path().join("claude-config")));

    h.ok(&["setup", "--link", checkout.to_str().unwrap()]);
    let skill = fs::canonicalize(&checkout)
        .unwrap()
        .join("skills/agent-factory");
    assert_eq!(target(&h.agents()), skill);
    assert_eq!(target(&h.claude()), skill);
    assert!(!h.home().join(".claude").exists());
    lock["skills"]
        .as_object_mut()
        .unwrap()
        .remove("agent-factory");
    assert_eq!(h.lock_value(), lock);
}

#[test]
fn link_refuses_an_untracked_directory_and_changes_nothing() {
    let h = Home::new();
    let checkout = h.checkout("checkout");
    let lock = h.install_with_skills();
    // A lock without the entry makes the agents copy someone's own directory.
    let mut untracked = lock.clone();
    untracked["skills"]
        .as_object_mut()
        .unwrap()
        .remove("agent-factory");
    let bytes = serde_json::to_vec_pretty(&untracked).unwrap();
    fs::write(h.lock(), &bytes).unwrap();

    let err = h.refused(&["setup", "--link", checkout.to_str().unwrap()]);
    assert!(
        err.contains(&format!("{} is a directory", h.agents().display()))
            && err.contains("does not list")
            && err.contains(&format!(
                "Move it aside (`mv {0} {0}.bak`)",
                h.agents().display()
            ))
            && err.contains("nothing changed"),
        "{err}"
    );
    assert!(h.agents().join("SKILL.md").is_file());
    assert_eq!(
        target(&h.claude()),
        Path::new("../../.agents/skills/agent-factory")
    );
    assert_eq!(fs::read(h.lock()).unwrap(), bytes);
}

#[test]
fn link_refuses_a_checkout_without_the_skill_and_changes_nothing() {
    let h = Home::new();
    let lock_bytes = {
        h.install_with_skills();
        fs::read(h.lock()).unwrap()
    };
    let empty = h.root.path().join("empty");
    fs::create_dir_all(&empty).unwrap();

    let err = h.refused(&["setup", "--link", empty.to_str().unwrap()]);
    assert!(
        err.contains("skills/agent-factory/SKILL.md does not exist")
            && err.contains("nothing changed"),
        "{err}"
    );
    let err = h.refused(&["setup", "--link", "/nonexistent/checkout"]);
    assert!(err.contains("does not exist"), "{err}");
    assert!(h.agents().join("SKILL.md").is_file());
    assert_eq!(
        target(&h.claude()),
        Path::new("../../.agents/skills/agent-factory")
    );
    assert_eq!(fs::read(h.lock()).unwrap(), lock_bytes);
}

#[test]
fn link_defaults_to_the_current_repository_root() {
    let h = Home::new();
    let checkout = h.checkout("repo");
    fs::create_dir_all(checkout.join(".atelier")).unwrap();
    let nested = checkout.join("crates/deep");
    fs::create_dir_all(&nested).unwrap();

    let (pass, stdout, stderr) = h.run(&nested, &["-q", "setup", "--link"]);
    assert!(pass, "{stdout}{stderr}");
    let skill = fs::canonicalize(&checkout)
        .unwrap()
        .join("skills/agent-factory");
    // The quiet token is the linked skill directory.
    assert_eq!(stdout, format!("{}\n", skill.display()));
    assert_eq!(target(&h.agents()), skill);
    assert_eq!(target(&h.claude()), skill);
}

#[test]
fn unlink_removes_the_links_and_leaves_a_skills_install_alone() {
    let h = Home::new();
    let checkout = h.checkout("checkout");
    h.ok(&["setup", "--link", checkout.to_str().unwrap()]);

    let out = h.ok(&["setup", "--unlink"]);
    assert!(fs::symlink_metadata(h.agents()).is_err());
    assert!(fs::symlink_metadata(h.claude()).is_err());
    assert_eq!(out.matches("removed its link to").count(), 2, "{out}");
    assert!(
        out.contains("npx skills add spadaval/atelier --skill agent-factory -g"),
        "{out}"
    );
    // Nothing left to unlink.
    let out = h.ok(&["setup", "--unlink"]);
    assert_eq!(out.matches(": no link").count(), 2, "{out}");

    // Quiet prints each removed path.
    h.ok(&["setup", "--link", checkout.to_str().unwrap()]);
    let out = h.ok(&["-q", "setup", "--unlink"]);
    assert_eq!(
        out,
        format!("{}\n{}\n", h.agents().display(), h.claude().display())
    );

    // A skill that `skills add` installed is not a link to undo.
    h.install_with_skills();
    let out = h.ok(&["setup", "--unlink"]);
    assert_eq!(
        out.matches("left alone: skills add installed it").count(),
        2,
        "{out}"
    );
    assert!(h.agents().join("SKILL.md").is_file());
    assert!(fs::symlink_metadata(h.claude()).is_ok());
}

#[test]
fn setup_still_needs_a_harness_without_link_flags() {
    let h = Home::new();
    let err = h.refused(&["setup"]);
    assert!(err.contains("<HARNESS>"), "{err}");
    let err = h.refused(&["setup", "claude", "--link"]);
    assert!(err.contains("cannot be used with"), "{err}");
    let err = h.refused(&["setup", "--link", "--unlink"]);
    assert!(err.contains("cannot be used with"), "{err}");
    let (_, help, _) = h.run(h.root.path(), &["setup", "--help"]);
    assert!(
        help.contains("Link a checkout that stays on master"),
        "{help}"
    );
}
