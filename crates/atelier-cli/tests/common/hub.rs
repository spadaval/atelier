//! Several clones of one bare remote, the forge's, for what crosses
//! clones: a claim, a drop, a merge and the cleanup after it.

use super::*;

/// The URL the clones' `origin` names, which `insteadOf` sends to the bare
/// repository, so the forge reads a GitHub repository from it.
pub const URL: &str = "git@github.com:owner/repo.git";

/// Clones of one bare remote, whose base config names it as the forge's
/// remote, with a fake `gh` that every command finds first on `PATH`.
pub struct Hub {
    pub dir: TempDir,
    pub remote: PathBuf,
    pub gh: PathBuf,
}

impl Hub {
    /// A remote whose `main` holds `items`, each an ID with its status.
    pub fn new(items: &[(&str, &str)]) -> Hub {
        let dir = TempDir::new().unwrap();
        let seed = dir.path().join("seed");
        git(&seed, &["init", "-q", "-b", "main"]);
        fs::create_dir_all(seed.join(".atelier")).unwrap();
        fs::write(
            seed.join(".atelier/config.toml"),
            "schema_version = 3\nslug = \"demo\"\nbase_branch = \"main\"\n\n\
             [forge]\nprovider = \"github\"\nremote = \"origin\"\n",
        )
        .unwrap();
        for (id, status) in items {
            write_item(&seed, id, status, &[], &[], "");
        }
        git(&seed, &["add", "-A"]);
        git(&seed, &["commit", "-qm", "Plan"]);
        let remote = dir.path().join("remote.git");
        git(
            dir.path(),
            &[
                "init",
                "-q",
                "--bare",
                "-b",
                "main",
                remote.to_str().unwrap(),
            ],
        );
        git(&seed, &["push", "-q", remote.to_str().unwrap(), "main"]);
        let gh = dir.path().join("gh");
        fs::create_dir_all(gh.join("bin")).unwrap();
        fs::write(gh.join("bin/gh"), FAKE_GH).unwrap();
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(gh.join("bin/gh"), fs::Permissions::from_mode(0o755)).unwrap();
        fs::write(gh.join("remote"), remote.to_str().unwrap()).unwrap();
        Hub { dir, remote, gh }
    }

    /// A clone named `name`, detached at the remote's `main`.
    pub fn clone(&self, name: &str) -> PathBuf {
        let at = self.dir.path().join(name);
        let from = self.remote.to_str().unwrap();
        git(
            self.dir.path(),
            &["clone", "-q", from, at.to_str().unwrap()],
        );
        git(&at, &["remote", "set-url", "origin", URL]);
        git(&at, &["config", &format!("url.{from}.insteadOf"), URL]);
        git(&at, &["checkout", "-q", "--detach", "origin/main"]);
        git(&at, &["branch", "-qD", "main"]);
        git(&at, &["branch", "-q", "--track", "main", "origin/main"]);
        at
    }

    pub fn command(&self, at: &Path, thread: &str, args: &[&str]) -> Command {
        let path = format!(
            "{}:{}",
            self.gh.join("bin").display(),
            std::env::var("PATH").unwrap()
        );
        let mut command = atelier_command(at, thread, args);
        command.env("PATH", path).env("FAKE_GH", &self.gh);
        command
    }

    pub fn atelier(&self, at: &Path, thread: &str, args: &[&str]) -> Run {
        let output = self.command(at, thread, args).output().unwrap();
        Run {
            code: output.status.code().unwrap_or(-1),
            stdout: String::from_utf8(output.stdout).unwrap(),
            stderr: String::from_utf8(output.stderr).unwrap(),
        }
    }

    /// The remote's copy of `branch`, if it has one.
    pub fn tip(&self, branch: &str) -> Option<String> {
        let output = git_output(
            &self.remote,
            &[
                "rev-parse",
                "--verify",
                "--quiet",
                &format!("refs/heads/{branch}"),
            ],
        );
        output
            .status
            .success()
            .then(|| String::from_utf8(output.stdout).unwrap().trim().to_string())
    }

    /// The item `ID`'s front-matter field `key` on the remote's `branch`.
    pub fn field(&self, branch: &str, key: &str) -> String {
        front(
            &git(
                &self.remote,
                &["show", &format!("{branch}:{}", item_path(ID))],
            ),
            key,
        )
    }

    /// `id` started by `thread` in `at`, with a change committed, submitted,
    /// and approved by `r1`, whose commit is in `at`.
    pub fn approved(&self, at: &Path, thread: &str) {
        ok(self.atelier(at, thread, &["start", ID]));
        fs::write(at.join("hello.txt"), "hello\n").unwrap();
        git(at, &["add", "hello.txt"]);
        git(at, &["commit", "-qm", "Say hello"]);
        let path = at.join(item_path(ID));
        let ticked = fs::read_to_string(&path).unwrap().replace("- [ ]", "- [x]");
        fs::write(&path, ticked + "It prints hello.\n").unwrap();
        ok(self.atelier(at, thread, &["submit"]));
        ok(self.atelier(at, "r1", &["review", ID]));
        let run = self.atelier(at, "r1", &["verdict", "approve"]);
        ok(run.clone());
        run_then(at, &run.stdout);
    }
}

pub fn ok(run: Run) {
    assert_eq!(run.code, 0, "{}{}", run.stdout, run.stderr);
}
