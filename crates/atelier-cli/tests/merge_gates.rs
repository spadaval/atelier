//! Boundary scenarios for immutable review records and gated integration.
use std::{fs, path::Path, process::Command};
use tempfile::{tempdir, TempDir};

fn git(dir: &Path, args: &[&str]) -> String {
    let output = Command::new("git")
        .current_dir(dir)
        .args(args)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "git {args:?}: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout).unwrap().trim().to_owned()
}
fn cli(dir: &Path, args: &[&str]) -> (bool, String) {
    let output = Command::new(env!("CARGO_BIN_EXE_atelier"))
        .current_dir(dir)
        .args(args)
        .output()
        .unwrap();
    (
        output.status.success(),
        format!(
            "{}{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        ),
    )
}
fn ok(dir: &Path, args: &[&str]) -> String {
    let (passed, output) = cli(dir, args);
    assert!(passed, "atelier {args:?}: {output}");
    output.trim().into()
}
fn commit(dir: &Path, message: &str) {
    git(dir, &["add", "."]);
    git(dir, &["commit", "-qm", message]);
}
struct Fixture {
    dir: TempDir,
    id: String,
    branch: String,
}
impl Fixture {
    fn new() -> Self {
        let dir = tempdir().unwrap();
        git(dir.path(), &["init", "-qb", "main"]);
        git(dir.path(), &["config", "user.name", "Author"]);
        git(dir.path(), &["config", "user.email", "author@example.test"]);
        ok(dir.path(), &["init", "--slug", "atelier"]);
        let id = ok(
            dir.path(),
            &[
                "-q",
                "issue",
                "create",
                "Owner",
                "--body",
                "## Goal\n\nShip change.\n\n## Done when\n\nScenario passes.",
            ],
        );
        commit(dir.path(), "base plan");
        ok(dir.path(), &["start", &id]);
        fs::write(dir.path().join("src.rs"), "tested code\n").unwrap();
        commit(dir.path(), "implementation and branch ownership");
        ok(
            dir.path(),
            &["issue", "transition", &id, "close", "--reason", "complete"],
        );
        commit(dir.path(), "completion claim");
        let branch = git(dir.path(), &["branch", "--show-current"]);
        Self { dir, id, branch }
    }
    fn path(&self) -> &Path {
        self.dir.path()
    }
    fn approval(&self, identity: &str) -> String {
        let event = ok(
            self.path(),
            &[
                "-q",
                "review",
                "approve",
                &self.id,
                "--identity",
                identity,
                "Reviewed the committed change",
            ],
        );
        commit(self.path(), "approval record");
        event
    }
    fn finding(&self) -> String {
        let event = ok(
            self.path(),
            &[
                "-q",
                "review",
                "finding",
                &self.id,
                "--identity",
                "Reviewer",
                "--recommendation",
                "fix_now",
                "Contract failure in src.rs",
            ],
        );
        commit(self.path(), "finding record");
        event
    }
    fn refusal(&self, expected: &str) {
        let before = git(self.path(), &["rev-parse", "main"]);
        let (passed, output) = cli(self.path(), &["merge", &self.id]);
        assert!(!passed && output.contains(expected), "{output}");
        assert_eq!(git(self.path(), &["rev-parse", "main"]), before);
        assert_eq!(git(self.path(), &["branch", "--show-current"]), self.branch);
    }
    fn issue(&self) -> atelier_records::CanonicalIssueRecord {
        atelier_records::RecordStore::new(self.path().join(".atelier"))
            .load_issue_by_id(&self.id)
            .unwrap()
    }
    fn write_issue(&self, record: &atelier_records::CanonicalIssueRecord) {
        atelier_records::RecordStore::new(self.path().join(".atelier"))
            .write_issue_atomic(record)
            .unwrap();
    }
}

#[test]
fn captured_checkout_refs_preserve_head_tags_aliases_and_base_worktrees() {
    let fixture = Fixture::new();
    fixture.approval("Reviewer");
    let head = git(fixture.path(), &["rev-parse", "HEAD"]);
    let base = git(fixture.path(), &["rev-parse", "refs/heads/main"]);
    // Git 2.53+ refuses `git tag HEAD`; older repositories may still hold
    // this ref, so write it directly.
    git(fixture.path(), &["update-ref", "refs/tags/HEAD", &base]);
    git(
        fixture.path(),
        &["tag", "-am", "shadow", &fixture.branch, &base],
    );
    git(
        fixture.path(),
        &[
            "symbolic-ref",
            "refs/heads/alias",
            &format!("refs/heads/{}", fixture.branch),
        ],
    );
    git(
        fixture.path(),
        &["symbolic-ref", "HEAD", "refs/heads/alias"],
    );
    #[cfg(unix)]
    {
        use std::os::unix::ffi::OsStringExt;
        let name = std::ffi::OsString::from_vec(vec![0xff]);
        fs::write(
            fixture.path().join(".git/refs/heads").join(name),
            format!("{base}\n"),
        )
        .unwrap();
    }
    let separate = tempdir().unwrap();
    let base_tree = separate.path().join("base");
    git(
        fixture.path(),
        &["worktree", "add", "-q", base_tree.to_str().unwrap(), "main"],
    );
    ok(fixture.path(), &["land", &fixture.id]);
    ok(fixture.path(), &["merge", &fixture.id, "--check"]);
    assert_eq!(git(fixture.path(), &["rev-parse", "HEAD"]), head);
    assert_eq!(git(&base_tree, &["rev-parse", "HEAD"]), base);
    // A non-head symbolic HEAD is never a valid owner checkout.
    git(
        fixture.path(),
        &[
            "symbolic-ref",
            "HEAD",
            &format!("refs/tags/{}", fixture.branch),
        ],
    );
    assert!(!cli(fixture.path(), &["land", &fixture.id]).0);
    assert!(!cli(fixture.path(), &["merge", &fixture.id, "--check"]).0);
}

#[test]
fn missing_self_approval_and_git_email_aliases_are_refused() {
    for identity in [
        None,
        Some("Author"),
        Some("author@example.test"),
        Some("Author <author@example.test>"),
    ] {
        let fixture = Fixture::new();
        if let Some(identity) = identity {
            fixture.approval(identity);
        }
        fixture.refusal("review.current");
    }
}

#[test]
fn approval_records_committed_by_author_remain_independent_and_merge_has_no_writes() {
    let fixture = Fixture::new();
    fixture.approval("Independent reviewer");
    let pin = git(fixture.path(), &["rev-parse", "HEAD"]);
    let tree = git(fixture.path(), &["rev-parse", "HEAD^{tree}"]);
    let count = git(fixture.path(), &["rev-list", "--count", "--all"]);
    let output = ok(fixture.path(), &["merge", &fixture.id]);
    assert!(
        output.contains("current independent approval")
            && output.contains("No commits or tracker records"),
        "{output}"
    );
    assert_eq!(git(fixture.path(), &["rev-parse", "HEAD"]), pin);
    assert_eq!(git(fixture.path(), &["rev-parse", "HEAD^{tree}"]), tree);
    assert_eq!(
        git(fixture.path(), &["rev-list", "--count", "--all"]),
        count
    );
    assert_eq!(git(fixture.path(), &["branch", "--show-current"]), "main");
    assert!(git(fixture.path(), &["status", "--porcelain"]).is_empty());
    assert!(!git(fixture.path(), &["branch", "--list", &fixture.branch]).contains(&fixture.branch));
    let index =
        atelier_app::record_index::RecordIndex::load(&fixture.path().join(".atelier")).unwrap();
    assert_eq!(index.require_issue(&fixture.id).unwrap().status, "done");
}

#[test]
fn statuses_and_new_evidence_or_review_events_do_not_stale_approval() {
    let fixture = Fixture::new();
    fixture.approval("Reviewer");
    let mut record = fixture.issue();
    record.issue.status = "review".into();
    record.issue.closed_at = None;
    record.sections.close_reason = None;
    fixture.write_issue(&record);
    commit(fixture.path(), "status change");
    record.issue.status = "done".into();
    record.sections.close_reason = Some("done".into());
    record.issue.closed_at = Some(chrono::Utc::now());
    record.issue.updated_at = chrono::Utc::now();
    fixture.write_issue(&record);
    commit(fixture.path(), "completion status change");
    ok(
        fixture.path(),
        &[
            "review",
            "record",
            &fixture.id,
            "--identity",
            "Reviewer",
            "Additional review context",
        ],
    );
    ok(
        fixture.path(),
        &[
            "evidence",
            "record",
            "--target",
            &format!("issue/{}", fixture.id),
            "--kind",
            "test",
            "--result",
            "pass",
            "Additional honest proof",
        ],
    );
    commit(fixture.path(), "new review and proof");
    let output = ok(fixture.path(), &["merge", &fixture.id, "--check", "--json"]);
    assert!(
        output.contains("workflow_validator")
            && output.contains("supports")
            && output.contains("review.current"),
        "{output}"
    );
}

#[test]
fn code_policy_acceptance_and_notes_changes_require_fresh_approval() {
    for field in [
        "code",
        "policy",
        "acceptance",
        "notes",
        "labels",
        "title",
        "unknown_front",
    ] {
        let fixture = Fixture::new();
        fixture.approval("Reviewer");
        let mut record = fixture.issue();
        match field {
            "code" => fs::write(fixture.path().join("src.rs"), "changed\n").unwrap(),
            "policy" => {
                let path = fixture.path().join(".atelier/workflow.yaml");
                let mut text = fs::read_to_string(&path).unwrap();
                text.push_str("\n# changed policy\n");
                fs::write(path, text).unwrap();
            }
            "acceptance" => {
                if let atelier_core::IssueBody::Issue { done_when, .. } = &mut record.sections.body
                {
                    *done_when = "Changed acceptance".into();
                }
                fixture.write_issue(&record);
            }
            "notes" => {
                record.sections.notes = Some("New notes".into());
                fixture.write_issue(&record);
            }
            "labels" => {
                record.labels.push("changed".into());
                fixture.write_issue(&record);
            }
            "title" => {
                record.issue.title = "New title".into();
                fixture.write_issue(&record);
            }
            _ => {
                let path = fixture
                    .path()
                    .join(format!(".atelier/issues/{}.md", fixture.id));
                let text = fs::read_to_string(&path).unwrap().replacen(
                    "schema:",
                    "unknown_acceptance: new\nschema:",
                    1,
                );
                fs::write(path, text).unwrap();
            }
        }
        commit(fixture.path(), "relevant change");
        fixture.refusal("review.current");
    }
}

#[test]
fn edit_then_revert_and_second_author_cannot_reuse_an_approval() {
    let fixture = Fixture::new();
    fixture.approval("Reviewer");
    fs::write(fixture.path().join("src.rs"), "broken\n").unwrap();
    commit(fixture.path(), "breaking change");
    fs::write(fixture.path().join("src.rs"), "tested code\n").unwrap();
    commit(fixture.path(), "revert");
    fixture.refusal("review.current");
    git(fixture.path(), &["config", "user.name", "Reviewer"]);
    fs::write(fixture.path().join("src.rs"), "Reviewer authored this\n").unwrap();
    commit(fixture.path(), "second author");
    fixture.approval("Reviewer");
    fixture.refusal("review.current");
}

#[test]
fn review_rewrites_deletions_and_modes_are_rejected_even_before_new_approval() {
    for action in ["edit", "delete", "mode"] {
        let fixture = Fixture::new();
        let finding = fixture.finding();
        let path = fixture
            .path()
            .join(format!(".atelier/reviews/{finding}.md"));
        match action {
            "edit" => {
                let text = fs::read_to_string(&path)
                    .unwrap()
                    .replace("Contract failure", "No contract failure");
                fs::write(&path, text).unwrap();
            }
            "delete" => fs::remove_file(&path).unwrap(),
            _ => {
                git(
                    fixture.path(),
                    &[
                        "update-index",
                        "--chmod=+x",
                        &format!(".atelier/reviews/{finding}.md"),
                    ],
                );
            }
        }
        if action == "mode" {
            git(fixture.path(), &["commit", "-qm", "changed review mode"]);
        } else {
            commit(fixture.path(), "erase prior finding");
        }
        fixture.approval("Other reviewer");
        fixture.refusal("review_append_only");
    }
}

#[test]
fn findings_require_dispositions_and_author_cannot_clear_fix_now() {
    let fixture = Fixture::new();
    fixture.approval("Reviewer");
    let finding = fixture.finding();
    fixture.refusal("findings.disposed");
    ok(
        fixture.path(),
        &[
            "review",
            "disposition",
            &finding,
            "--identity",
            "Author",
            "--decision",
            "no_action",
            "Author's unsupported rebuttal",
        ],
    );
    commit(fixture.path(), "author rebuttal");
    fixture.refusal("findings.disposed");
    ok(
        fixture.path(),
        &[
            "review",
            "disposition",
            &finding,
            "--identity",
            "Reviewer",
            "--decision",
            "no_action",
            "Reviewer accepts the code evidence",
        ],
    );
    commit(fixture.path(), "independent acceptance");
    ok(fixture.path(), &["merge", &fixture.id, "--check"]);
}

#[test]
fn fixed_finding_requires_a_resolution_commit_and_current_independent_approval() {
    let fixture = Fixture::new();
    let finding = fixture.finding();
    fs::write(fixture.path().join("src.rs"), "fixed code\n").unwrap();
    commit(fixture.path(), "fix contract");
    let resolution = git(fixture.path(), &["rev-parse", "HEAD"]);
    ok(
        fixture.path(),
        &[
            "review",
            "disposition",
            &finding,
            "--identity",
            "Author",
            "--decision",
            "fix_now",
            "--resolution-commit",
            &resolution,
            "Fixed current contract",
        ],
    );
    commit(fixture.path(), "resolution record");
    fixture.refusal("review.current");
    fixture.approval("Reviewer");
    ok(fixture.path(), &["merge", &fixture.id, "--check"]);
}

#[test]
fn trivial_exemption_is_explicit_visible_and_pinned() {
    let fixture = Fixture::new();
    ok(
        fixture.path(),
        &[
            "review",
            "trivial",
            &fixture.id,
            "--identity",
            "Author",
            "Correctness is evident from the formatting diff",
        ],
    );
    commit(fixture.path(), "explicit exemption");
    assert!(ok(fixture.path(), &["merge", &fixture.id, "--check"])
        .contains("EXPLICIT TRIVIAL EXEMPTION"));
    fs::write(fixture.path().join("src.rs"), "new behavior\n").unwrap();
    commit(fixture.path(), "new nontrivial work");
    fixture.refusal("review.current");
}

#[test]
fn base_policy_controls_shell_checks_and_candidate_cannot_loosen_them() {
    for command in ["exit 7", "test -f src.rs"] {
        let fixture = Fixture::new();
        // Rebuild a fixture with the required check already committed to base.
        git(fixture.path(), &["switch", "main"]);
        let path = fixture.path().join(".atelier/workflow.yaml");
        let policy = fs::read_to_string(&path).unwrap();
        fs::write(
            &path,
            format!("{policy}\nmerge_checks:\n  - shell.check: {{ command: '{command}' }}\n"),
        )
        .unwrap();
        commit(fixture.path(), "base check policy");
        git(fixture.path(), &["switch", &fixture.branch]);
        git(fixture.path(), &["rebase", "main"]);
        fs::write(&path, policy).unwrap();
        commit(fixture.path(), "candidate removes required check");
        fixture.approval("Reviewer");
        if command == "exit 7" {
            fixture.refusal("shell.check");
        } else {
            ok(fixture.path(), &["merge", &fixture.id, "--check"]);
        }
    }
}

#[test]
fn shell_checks_that_mutate_the_tree_fail_before_integration() {
    let fixture = Fixture::new();
    git(fixture.path(), &["switch", "main"]);
    let path = fixture.path().join(".atelier/workflow.yaml");
    let policy = fs::read_to_string(&path).unwrap();
    fs::write(
        &path,
        format!(
            "{policy}\nmerge_checks:\n  - shell.check: {{ command: 'printf dirty > src.rs' }}\n"
        ),
    )
    .unwrap();
    commit(fixture.path(), "mutating policy check");
    git(fixture.path(), &["switch", &fixture.branch]);
    git(fixture.path(), &["rebase", "main"]);
    fixture.approval("Reviewer");
    fixture.refusal("merge_checkout_dirty");
}

#[test]
fn invalid_candidate_policy_is_rejected_and_base_fallback_is_visible() {
    let fixture = Fixture::new();
    fixture.approval("Reviewer");
    fs::write(fixture.path().join(".atelier/workflow.yaml"), "invalid: [").unwrap();
    commit(fixture.path(), "malformed policy");
    fixture.refusal("candidate workflow policy is invalid");
    let output = cli(fixture.path(), &["issue", "show", &fixture.id]).1;
    assert!(
        output.contains("Invalid working policy") && output.contains("Using last valid policy"),
        "{output}"
    );
    assert!(!cli(fixture.path(), &["check"]).0);
    let other = Fixture::new();
    git(other.path(), &["switch", "main"]);
    fs::write(other.path().join(".atelier/workflow.yaml"), "invalid: [").unwrap();
    commit(other.path(), "invalid base policy");
    git(other.path(), &["switch", &other.branch]);
    git(other.path(), &["merge", "--no-edit", "main"]);
    fs::write(
        other.path().join(".atelier/workflow.yaml"),
        atelier_workflow::STARTER_POLICY_YAML,
    )
    .unwrap();
    commit(other.path(), "repair candidate policy");
    other.approval("Reviewer");
    let output = ok(other.path(), &["merge", &other.id, "--check"]);
    assert!(
        output.contains("Invalid policy at") && output.contains("Using last valid policy"),
        "{output}"
    );
}

#[test]
fn divergence_and_dirty_candidate_or_base_checkout_are_refused() {
    let fixture = Fixture::new();
    fixture.approval("Reviewer");
    fs::write(fixture.path().join("uncommitted"), "dirty").unwrap();
    fixture.refusal("merge_checkout_dirty");
    fs::remove_file(fixture.path().join("uncommitted")).unwrap();
    let base = tempdir().unwrap();
    git(
        fixture.path(),
        &["worktree", "add", base.path().to_str().unwrap(), "main"],
    );
    fs::write(base.path().join("dirty"), "base dirty").unwrap();
    fixture.refusal("merge_checkout_dirty");
    fs::remove_file(base.path().join("dirty")).unwrap();
    fs::write(base.path().join("new-base-code"), "diverged").unwrap();
    commit(base.path(), "base divergence");
    fixture.refusal("merge_diverged");
    let (_, output) = cli(fixture.path(), &["merge", &fixture.id]);
    assert!(
        output.contains("`git merge main`") && output.contains("Do not rebase"),
        "{output}"
    );
}

#[test]
fn rebasing_a_reviewed_branch_explains_the_cause_and_recovery() {
    let fixture = Fixture::new();
    fixture.approval("Reviewer");
    let reviewed = git(fixture.path(), &["rev-parse", "HEAD"]);
    git(fixture.path(), &["switch", "-q", "main"]);
    fs::write(fixture.path().join("other.rs"), "landed on the base\n").unwrap();
    commit(fixture.path(), "other work landed on the base");
    git(fixture.path(), &["switch", "-q", &fixture.branch]);
    git(fixture.path(), &["rebase", "-q", "main"]);
    fixture.refusal("pins a commit outside the work branch");
    let (_, output) = cli(fixture.path(), &["merge", &fixture.id]);
    assert!(
        output.contains("rebased or reset") && output.contains("git reflog"),
        "{output}"
    );

    // The advised recovery: restore the reviewed tip, then merge the base.
    git(fixture.path(), &["reset", "-q", "--hard", &reviewed]);
    git(fixture.path(), &["merge", "-q", "--no-edit", "main"]);
    ok(fixture.path(), &["merge", &fixture.id]);
}

#[test]
fn review_references_and_identity_are_validated_without_partial_writes() {
    let fixture = Fixture::new();
    let finding = fixture.finding();
    for args in [
        vec![
            "review",
            "approve",
            &fixture.id,
            "--identity",
            "",
            "no identity",
        ],
        vec![
            "review",
            "disposition",
            &finding,
            "--identity",
            "Reviewer",
            "--decision",
            "defer",
            "missing followup",
        ],
        vec![
            "review",
            "disposition",
            &finding,
            "--identity",
            "Reviewer",
            "--decision",
            "fix_now",
            "--resolution-commit",
            "0000000000000000000000000000000000000000",
            "missing commit",
        ],
    ] {
        let before = fs::read_dir(fixture.path().join(".atelier/reviews"))
            .unwrap()
            .count();
        assert!(!cli(fixture.path(), &args).0);
        assert_eq!(
            fs::read_dir(fixture.path().join(".atelier/reviews"))
                .unwrap()
                .count(),
            before
        );
    }
}

#[test]
fn git_hooks_are_explicitly_disabled_in_the_final_integration_fence() {
    let fixture = Fixture::new();
    fixture.approval("Reviewer");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let hook = fixture.path().join(".git/hooks/post-merge");
        fs::write(&hook, "#!/bin/sh\nprintf unexpected > hook-side-effect\n").unwrap();
        fs::set_permissions(&hook, fs::Permissions::from_mode(0o755)).unwrap();
        let output = ok(fixture.path(), &["merge", &fixture.id]);
        assert!(output.contains("Git hooks are disabled"));
        assert!(!fixture.path().join("hook-side-effect").exists());
    }
}

#[test]
fn scoped_children_and_open_transitive_blockers_prevent_merge() {
    let fixture = Fixture::new();
    let child = ok(fixture.path(), &["-q", "issue", "create", "Open child"]);
    let mut owner = fixture.issue();
    owner.issue.issue_type = "feature".into();
    owner.sections.change_type("feature");
    owner
        .relationships
        .children
        .push(atelier_records::issue_relationship_target(&child));
    fixture.write_issue(&owner);
    commit(fixture.path(), "epic owns an open child");
    fixture.approval("Reviewer");
    fixture.refusal("work.terminal");
    let mut active_owner = fixture.issue();
    active_owner.issue.status = "in_progress".into();
    active_owner.issue.closed_at = None;
    active_owner.sections.close_reason = None;
    fixture.write_issue(&active_owner);
    commit(fixture.path(), "scope status active");
    ok(fixture.path(), &["issue", "transition", &child, "start"]);
    ok(
        fixture.path(),
        &["issue", "transition", &child, "close", "--reason", "done"],
    );
    commit(fixture.path(), "child complete");
    ok(
        fixture.path(),
        &[
            "issue",
            "transition",
            &fixture.id,
            "close",
            "--reason",
            "epic complete",
        ],
    );
    commit(fixture.path(), "epic done claim");
    let output = ok(fixture.path(), &["merge", &fixture.id]);
    assert!(output.contains("owner and all scoped work are terminal"));
    let index =
        atelier_app::record_index::RecordIndex::load(&fixture.path().join(".atelier")).unwrap();
    assert_eq!(index.require_issue(&child).unwrap().status, "done");

    let other = Fixture::new();
    let blocker = ok(other.path(), &["-q", "issue", "create", "Open dependency"]);
    let indirect = ok(
        other.path(),
        &["-q", "issue", "create", "Indirect dependency"],
    );
    // Terminal direct blocker still has an open transitive dependency.
    let store = atelier_records::RecordStore::new(other.path().join(".atelier"));
    let mut record = store.load_issue_by_id(&blocker).unwrap();
    record.issue.status = "done".into();
    record.sections.close_reason = Some("terminal direct blocker".into());
    record
        .relationships
        .blocks
        .push(atelier_records::issue_relationship_target(&other.id));
    store.write_issue_atomic(&record).unwrap();
    let mut record = store.load_issue_by_id(&indirect).unwrap();
    record
        .relationships
        .blocks
        .push(atelier_records::issue_relationship_target(&blocker));
    store.write_issue_atomic(&record).unwrap();
    commit(other.path(), "declared dependencies");
    other.approval("Reviewer");
    other.refusal("blockers.none_open");
}

#[test]
fn out_of_scope_statuses_relationship_changes_and_existing_proof_edits_stale_approval() {
    for change in ["outside_status", "relationships", "proof_edit"] {
        let fixture = Fixture::new();
        let outside = ok(fixture.path(), &["-q", "issue", "create", "Outside scope"]);
        let _proof_output = ok(
            fixture.path(),
            &[
                "-q",
                "evidence",
                "record",
                "--target",
                &format!("issue/{}", fixture.id),
                "--kind",
                "test",
                "--result",
                "pass",
                "Proof record",
            ],
        );
        let proof = fs::read_dir(fixture.path().join(".atelier/evidence"))
            .unwrap()
            .next()
            .unwrap()
            .unwrap()
            .path()
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap()
            .to_owned();
        commit(fixture.path(), "other issue and proof before approval");
        fixture.approval("Reviewer");
        match change {
            "outside_status" => {
                ok(fixture.path(), &["issue", "transition", &outside, "start"]);
            }
            "relationships" => {
                let mut record = fixture.issue();
                record
                    .relationships
                    .relates
                    .push(atelier_records::issue_relates_relationship(
                        &outside, "related",
                    ));
                fixture.write_issue(&record);
            }
            _ => {
                let path = fixture.path().join(format!(".atelier/evidence/{proof}.md"));
                let text = fs::read_to_string(&path)
                    .unwrap()
                    .replace("Proof record", "Changed proof record");
                fs::write(path, text).unwrap();
            }
        }
        commit(fixture.path(), "post-approval relevant tracker edit");
        fixture.refusal("review.current");
    }
}

#[test]
fn clean_merge_of_the_base_keeps_approval_but_resolved_or_edited_merges_do_not() {
    for case in ["clean", "conflict", "edited", "later_code"] {
        let fixture = Fixture::new();
        fixture.approval("Reviewer");
        git(fixture.path(), &["switch", "-q", "main"]);
        let landed = if case == "conflict" {
            "src.rs"
        } else {
            "other.rs"
        };
        fs::write(fixture.path().join(landed), "landed on the base\n").unwrap();
        commit(fixture.path(), "other work landed on the base");
        git(fixture.path(), &["switch", "-q", &fixture.branch]);
        match case {
            "conflict" => {
                let merged = Command::new("git")
                    .current_dir(fixture.path())
                    .args(["merge", "--no-edit", "main"])
                    .output()
                    .unwrap();
                assert!(!merged.status.success());
                fs::write(fixture.path().join("src.rs"), "tested code\n").unwrap();
                git(fixture.path(), &["add", "."]);
                git(fixture.path(), &["commit", "-q", "--no-edit"]);
            }
            "edited" => {
                git(fixture.path(), &["merge", "-q", "--no-commit", "main"]);
                fs::write(fixture.path().join("src.rs"), "edited while merging\n").unwrap();
                git(fixture.path(), &["add", "."]);
                git(fixture.path(), &["commit", "-q", "--no-edit"]);
            }
            _ => {
                git(fixture.path(), &["merge", "-q", "--no-edit", "main"]);
            }
        }
        if case == "later_code" {
            fs::write(fixture.path().join("src.rs"), "changed after merging\n").unwrap();
            commit(fixture.path(), "code after the base merge");
        }
        if case == "clean" {
            let output = ok(fixture.path(), &["merge", &fixture.id, "--check"]);
            assert!(output.contains("pass review.current"), "{output}");
            let head = git(fixture.path(), &["rev-parse", "HEAD"]);
            ok(fixture.path(), &["merge", &fixture.id]);
            assert_eq!(git(fixture.path(), &["rev-parse", "main"]), head);
        } else {
            fixture.refusal("FAIL review.current");
        }
    }
}

#[test]
fn base_merge_record_resolutions_keep_approval_only_when_they_lose_nothing() {
    for case in [
        "union",
        "drop_side",
        "edit_while_merging",
        "base_scope_edit",
    ] {
        let fixture = Fixture::new();
        git(fixture.path(), &["switch", "-q", "main"]);
        let mission = ok(
            fixture.path(),
            &[
                "-q",
                "issue",
                "create",
                "Mission",
                "--issue-type",
                "mission",
            ],
        );
        let outside = ok(fixture.path(), &["-q", "issue", "create", "Outside"]);
        commit(fixture.path(), "a mission and an outside issue");
        git(fixture.path(), &["switch", "-q", &fixture.branch]);
        git(fixture.path(), &["merge", "-q", "--no-edit", "main"]);
        // Planning on the branch adds work to the mission before approval.
        let branch_work = ok(fixture.path(), &["-q", "issue", "create", "Branch work"]);
        ok(fixture.path(), &["issue", "link", &branch_work, &mission]);
        commit(fixture.path(), "branch planning");
        fixture.approval("Reviewer");

        // Planning on the base adds other work to the same mission.
        git(fixture.path(), &["switch", "-q", "main"]);
        let base_work = ok(fixture.path(), &["-q", "issue", "create", "Base work"]);
        ok(fixture.path(), &["issue", "link", &base_work, &mission]);
        if case == "base_scope_edit" {
            // The owner's Goal, far from the branch's own edits, so Git
            // merges it cleanly into the approved work.
            let owner = fixture
                .path()
                .join(format!(".atelier/issues/{}.md", fixture.id));
            let text = fs::read_to_string(&owner).unwrap();
            fs::write(
                owner,
                text.replace("Ship change.", "Ship a different change."),
            )
            .unwrap();
        }
        commit(fixture.path(), "base planning");
        git(fixture.path(), &["switch", "-q", &fixture.branch]);
        let merged = Command::new("git")
            .current_dir(fixture.path())
            .args(["merge", "--no-edit", "main"])
            .output()
            .unwrap();
        assert!(!merged.status.success(), "the mission record conflicts");
        assert_eq!(
            git(fixture.path(), &["diff", "--name-only", "--diff-filter=U"]),
            format!(".atelier/issues/{mission}.md"),
            "only the mission record conflicts"
        );
        let path = format!(".atelier/issues/{mission}.md");
        if case == "drop_side" {
            git(fixture.path(), &["checkout", "--ours", "--", &path]);
        } else {
            git(
                fixture.path(),
                &["checkout", "--conflict=merge", "--", &path],
            );
            let stage = |n: u8| git(fixture.path(), &["show", &format!(":{n}:{path}")]) + "\n";
            let dir = tempdir().unwrap();
            for (n, name) in [(1, "base"), (2, "ours"), (3, "theirs")] {
                fs::write(dir.path().join(name), stage(n)).unwrap();
            }
            let union = Command::new("git")
                .args(["merge-file", "--union", "-p"])
                .args(["ours", "base", "theirs"].map(|name| dir.path().join(name)))
                .output()
                .unwrap();
            fs::write(fixture.path().join(&path), union.stdout).unwrap();
        }
        if case == "edit_while_merging" {
            ok(fixture.path(), &["issue", "transition", &outside, "start"]);
        }
        git(fixture.path(), &["add", "."]);
        git(fixture.path(), &["commit", "-q", "--no-edit"]);
        if case != "drop_side" {
            ok(fixture.path(), &["check"]);
        }
        if case == "union" {
            let output = ok(fixture.path(), &["merge", &fixture.id, "--check"]);
            assert!(output.contains("pass review.current"), "{output}");
        } else {
            fixture.refusal("FAIL review.current");
        }
    }
}

#[test]
fn new_out_of_scope_follow_up_issue_keeps_approval_but_new_child_does_not() {
    let fixture = Fixture::new();
    let finding = fixture.finding();
    fixture.approval("Reviewer");
    let follow_up = ok(
        fixture.path(),
        &["-q", "issue", "create", "Follow up the deferred finding"],
    );
    ok(
        fixture.path(),
        &[
            "review",
            "disposition",
            &finding,
            "--identity",
            "Reviewer",
            "--decision",
            "defer",
            "--follow-up",
            &follow_up,
            "Follow-up accepted",
        ],
    );
    commit(fixture.path(), "file follow-up and defer after approval");
    let output = ok(fixture.path(), &["merge", &fixture.id, "--check"]);
    assert!(output.contains("pass review.current"), "{output}");

    ok(
        fixture.path(),
        &[
            "-q",
            "issue",
            "create",
            "New child",
            "--parent",
            &fixture.id,
        ],
    );
    // Adding a child edits the owner's children, which is reviewed scope.
    commit(fixture.path(), "new in-scope child after approval");
    fixture.refusal("FAIL review.current");
}

#[test]
fn policy_without_any_valid_history_cannot_authorize_merge() {
    let fixture = Fixture::new();
    // Replace the base ref with an orphan carrying no valid policy.
    git(fixture.path(), &["checkout", "--orphan", "no-policy"]);
    git(fixture.path(), &["rm", "-qrf", "."]);
    fs::write(fixture.path().join("README"), "orphan base").unwrap();
    commit(fixture.path(), "base without policy");
    let invalid = git(fixture.path(), &["rev-parse", "HEAD"]);
    git(fixture.path(), &["switch", &fixture.branch]);
    git(
        fixture.path(),
        &[
            "merge",
            "--allow-unrelated-histories",
            "--no-edit",
            "no-policy",
        ],
    );
    git(fixture.path(), &["branch", "-f", "main", &invalid]);
    fixture.approval("Reviewer");
    fixture.refusal("No valid policy");
}

#[test]
#[cfg(feature = "test-hooks")]
fn prune_apply_waits_for_read_transactions_and_failure_restores_linked_bytes() {
    use std::time::Duration;
    let fixture = Fixture::new();
    fixture.approval("Reviewer");
    ok(fixture.path(), &["merge", &fixture.id]);
    let second = ok(
        fixture.path(),
        &["-q", "issue", "create", "Second terminal"],
    );
    let store = atelier_records::RecordStore::new(fixture.path().join(".atelier"));
    let mut ids = [fixture.id.clone(), second];
    ids.sort();
    let old = chrono::Utc::now() - chrono::Duration::days(45);
    for id in &ids {
        let mut record = store.load_issue_by_id(id).unwrap();
        record.issue.status = "done".into();
        record.issue.updated_at = old;
        record.issue.closed_at = Some(old);
        record.sections.close_reason = Some("old terminal record".into());
        if id == &ids[1] {
            record
                .relationships
                .blocks
                .push(atelier_records::issue_relationship_target(&ids[0]));
        }
        store.write_issue_atomic(&record).unwrap();
    }
    for entry in fs::read_dir(fixture.path().join(".atelier/reviews")).unwrap() {
        let path = entry.unwrap().path();
        let text = fs::read_to_string(&path).unwrap();
        let (front, body) = atelier_records::split_document(&text).unwrap();
        let mut yaml: serde_yaml::Value = serde_yaml::from_str(front).unwrap();
        yaml["created_at"] = serde_yaml::Value::String(old.to_rfc3339());
        fs::write(
            path,
            format!(
                "---\n{}---\n\n{body}",
                serde_yaml::to_string(&yaml).unwrap()
            ),
        )
        .unwrap();
    }
    commit(fixture.path(), "old linked terminal cluster");
    let before = ids
        .iter()
        .map(|id| fs::read(fixture.path().join(format!(".atelier/issues/{id}.md"))).unwrap())
        .collect::<Vec<_>>();
    let reviews = fs::read_dir(fixture.path().join(".atelier/reviews"))
        .unwrap()
        .map(|entry| {
            let path = entry.unwrap().path();
            let bytes = fs::read(&path).unwrap();
            (path, bytes)
        })
        .collect::<Vec<_>>();
    let lock = atelier_records::mutation_lock::CanonicalMutationLock::shared(
        &fixture.path().join(".atelier"),
    )
    .unwrap();
    let mut child = Command::new(env!("CARGO_BIN_EXE_atelier"))
        .current_dir(fixture.path())
        .args(["prune", "--apply", "--retention-days", "30"])
        .env("ATELIER_TEST_PRUNE_REMOVE_FAILURE", &ids[1])
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .unwrap();
    std::thread::sleep(Duration::from_millis(150));
    assert!(
        child.try_wait().unwrap().is_none(),
        "prune apply must wait for existing readers before loading/planning"
    );
    drop(lock);
    let output = child.wait_with_output().unwrap();
    assert!(
        !output.status.success()
            && String::from_utf8_lossy(&output.stderr).contains("every staged record was restored")
    );
    for (id, bytes) in ids.iter().zip(before) {
        assert_eq!(
            fs::read(fixture.path().join(format!(".atelier/issues/{id}.md"))).unwrap(),
            bytes
        );
    }
    for (path, bytes) in reviews {
        assert_eq!(fs::read(path).unwrap(), bytes);
    }
    ok(fixture.path(), &["check"]);
    assert!(git(fixture.path(), &["status", "--porcelain"]).is_empty());
    ok(
        fixture.path(),
        &["prune", "--apply", "--retention-days", "30"],
    );
    ok(fixture.path(), &["check"]);
    assert!(fs::read_dir(fixture.path().join(".atelier/reviews"))
        .unwrap()
        .next()
        .is_none());
}

#[test]
fn candidate_cannot_remove_base_completion_checks_for_transitions_or_direct_done_claims() {
    let fixture = Fixture::new();
    git(fixture.path(), &["switch", "main"]);
    let path = fixture.path().join(".atelier/workflow.yaml");
    let policy = fs::read_to_string(&path).unwrap();
    fs::write(
        &path,
        policy.replace(
            "        - blockers.none_open\n",
            "        - blockers.none_open\n        - evidence.attached: { min_count: 1 }\n",
        ),
    )
    .unwrap();
    commit(
        fixture.path(),
        "base requires honest evidence for completion",
    );
    git(fixture.path(), &["switch", &fixture.branch]);
    git(fixture.path(), &["rebase", "main"]);
    fs::write(&path, &policy).unwrap();
    commit(fixture.path(), "candidate loosens close checks");
    let mut record = fixture.issue();
    record.issue.status = "in_progress".into();
    record.issue.closed_at = None;
    record.sections.close_reason = None;
    fixture.write_issue(&record);
    commit(fixture.path(), "active status");
    let (passed, output) = cli(
        fixture.path(),
        &[
            "issue",
            "transition",
            &fixture.id,
            "close",
            "--reason",
            "claim without proof",
        ],
    );
    assert!(!passed && output.contains("evidence.attached"), "{output}");
    record.issue.status = "done".into();
    record.issue.closed_at = Some(chrono::Utc::now());
    record.sections.close_reason = Some("direct edit done".into());
    fixture.write_issue(&record);
    commit(fixture.path(), "direct terminal claim");
    fixture.approval("Reviewer");
    fixture.refusal("evidence.attached");
}

#[test]
fn separately_checked_out_clean_base_is_fast_forwarded_and_source_branch_is_cleaned() {
    let fixture = Fixture::new();
    fixture.approval("Reviewer");
    let head = git(fixture.path(), &["rev-parse", "HEAD"]);
    let base = tempdir().unwrap();
    git(
        fixture.path(),
        &["worktree", "add", base.path().to_str().unwrap(), "main"],
    );
    let output = ok(fixture.path(), &["merge", &fixture.id]);
    assert!(
        output.contains("deleted local work branch")
            && output.contains("work checkout is detached"),
        "{output}"
    );
    assert_eq!(git(base.path(), &["rev-parse", "HEAD"]), head);
    assert!(git(base.path(), &["status", "--porcelain"]).is_empty());
    assert!(git(fixture.path(), &["status", "--porcelain"]).is_empty());
    assert!(git(base.path(), &["branch", "--list", &fixture.branch]).is_empty());
}

#[test]
fn branch_cleanup_does_not_require_an_outdated_upstream_to_contain_merged_work() {
    let fixture = Fixture::new();
    fixture.approval("Reviewer");
    let base = git(fixture.path(), &["rev-parse", "main"]);
    let remote = tempdir().unwrap();
    git(remote.path(), &["init", "--bare", "-q"]);
    git(
        fixture.path(),
        &["remote", "add", "origin", remote.path().to_str().unwrap()],
    );
    git(
        fixture.path(),
        &["update-ref", "refs/remotes/origin/old-work", &base],
    );
    git(
        fixture.path(),
        &[
            "config",
            &format!("branch.{}.remote", fixture.branch),
            "origin",
        ],
    );
    git(
        fixture.path(),
        &[
            "config",
            &format!("branch.{}.merge", fixture.branch),
            "refs/heads/old-work",
        ],
    );
    ok(fixture.path(), &["merge", &fixture.id]);
    assert!(git(fixture.path(), &["branch", "--list", &fixture.branch]).is_empty());
}

#[test]
fn review_only_side_branch_merge_preserves_approval_in_either_parent_context() {
    for approved_second_parent in [false, true] {
        let fixture = Fixture::new();
        git(fixture.path(), &["branch", "old-side", "main"]);
        fixture.approval("Reviewer");
        git(fixture.path(), &["switch", "old-side"]);
        ok(
            fixture.path(),
            &[
                "review",
                "record",
                &fixture.id,
                "--identity",
                "Other reviewer",
                "Side branch review context",
            ],
        );
        commit(fixture.path(), "record-only side branch");
        if approved_second_parent {
            git(fixture.path(), &["merge", "--no-edit", &fixture.branch]);
            git(fixture.path(), &["switch", &fixture.branch]);
            git(fixture.path(), &["merge", "--ff-only", "old-side"]);
        } else {
            git(fixture.path(), &["switch", &fixture.branch]);
            git(fixture.path(), &["merge", "--no-edit", "old-side"]);
        }
        let output = ok(fixture.path(), &["merge", &fixture.id, "--check", "--json"]);
        let nodes: serde_json::Value = serde_json::from_str(&output).unwrap();
        let ids = nodes
            .as_array()
            .unwrap()
            .iter()
            .map(|node| node["id"].as_str().unwrap())
            .collect::<std::collections::BTreeSet<_>>();
        assert_eq!(
            ids.len(),
            nodes.as_array().unwrap().len(),
            "derived gate node IDs must be unique"
        );
    }
}

#[test]
fn introduced_side_code_edit_then_revert_stales_approval_after_merge() {
    let fixture = Fixture::new();
    git(fixture.path(), &["branch", "side-before-review", "HEAD"]);
    fixture.approval("Reviewer");
    git(fixture.path(), &["switch", "side-before-review"]);
    fs::write(fixture.path().join("src.rs"), "broken side code\n").unwrap();
    commit(fixture.path(), "side change");
    fs::write(fixture.path().join("src.rs"), "tested code\n").unwrap();
    commit(fixture.path(), "side revert");
    git(fixture.path(), &["switch", &fixture.branch]);
    git(
        fixture.path(),
        &["merge", "--no-edit", "side-before-review"],
    );
    fixture.refusal("review.current");
}

#[test]
fn workflow_named_merge_cannot_admit_transition_shell_checks() {
    let fixture = Fixture::new();
    let path = fixture.path().join(".atelier/workflow.yaml");
    let mut policy: serde_yaml::Value =
        serde_yaml::from_str(&fs::read_to_string(&path).unwrap()).unwrap();
    let workflows = policy["workflows"].as_mapping_mut().unwrap();
    let mut work = workflows.remove(serde_yaml::Value::from("work")).unwrap();
    work["transitions"]["start"]["validators"] =
        serde_yaml::from_str("[ { shell.check: { command: 'touch transition-shell-ran' } } ]")
            .unwrap();
    workflows.insert(serde_yaml::Value::from("merge"), work);
    let candidate = serde_yaml::to_string(&policy).unwrap();
    let error = atelier_workflow::parse_policy_text(&candidate, "fixture").unwrap_err();
    assert!(
        error.to_string().contains("available only in merge_checks"),
        "{error}"
    );
    fs::write(&path, candidate).unwrap();
    let (passed, output) = cli(fixture.path(), &["check"]);
    assert!(
        !passed && output.contains("available only in merge_checks"),
        "{output}"
    );
    assert!(!fixture.path().join("transition-shell-ran").exists());

    policy["workflows"]["merge"]["transitions"]["start"]["validators"] =
        serde_yaml::Value::Sequence(Vec::new());
    policy["merge_checks"] =
        serde_yaml::from_str("[ { shell.check: { command: 'test -f src.rs' } } ]").unwrap();
    let accepted =
        atelier_workflow::parse_policy_text(&serde_yaml::to_string(&policy).unwrap(), "fixture")
            .unwrap();
    assert_eq!(accepted.merge_checks[0].builtin, "shell.check");
}

#[test]
fn same_commit_base_checkout_branch_switch_during_check_refuses_integration() {
    let fixture = Fixture::new();
    let base = tempdir().unwrap();
    git(fixture.path(), &["switch", "main"]);
    let path = fixture.path().join(".atelier/workflow.yaml");
    let policy = fs::read_to_string(&path).unwrap();
    fs::write(
        &path,
        format!(
        "{policy}\nmerge_checks:\n  - shell.check: {{ command: 'git -C {} switch alternate' }}\n",
        base.path().display(),
    ),
    )
    .unwrap();
    commit(fixture.path(), "base checkout switch check");
    git(fixture.path(), &["branch", "alternate", "main"]);
    git(fixture.path(), &["switch", &fixture.branch]);
    git(fixture.path(), &["rebase", "main"]);
    fixture.approval("Reviewer");
    git(
        fixture.path(),
        &["worktree", "add", base.path().to_str().unwrap(), "main"],
    );
    let base_head = git(fixture.path(), &["rev-parse", "main"]);
    let work_head = git(fixture.path(), &["rev-parse", "HEAD"]);
    fixture.refusal("base checkout ownership changed");
    assert_eq!(git(fixture.path(), &["rev-parse", "alternate"]), base_head);
    assert_eq!(
        git(fixture.path(), &["rev-parse", &fixture.branch]),
        work_head
    );
    assert_eq!(git(base.path(), &["branch", "--show-current"]), "alternate");
    assert!(git(fixture.path(), &["status", "--porcelain"]).is_empty());
}

#[test]
fn ignored_uncommitted_approval_issue_and_evidence_are_not_merge_inputs() {
    for kind in ["reviews", "issues", "evidence"] {
        let fixture = Fixture::new();
        if kind != "reviews" {
            fixture.approval("Reviewer");
        }
        fs::write(
            fixture.path().join(".git/info/exclude"),
            format!(".atelier/{kind}/\n"),
        )
        .unwrap();
        match kind {
            "reviews" => {
                ok(
                    fixture.path(),
                    &[
                        "review",
                        "approve",
                        &fixture.id,
                        "--identity",
                        "Reviewer",
                        "Ignored approval",
                    ],
                );
            }
            "issues" => {
                ok(fixture.path(), &["issue", "create", "Ignored issue"]);
            }
            "evidence" => {
                ok(
                    fixture.path(),
                    &[
                        "evidence",
                        "record",
                        "--target",
                        &format!("issue/{}", fixture.id),
                        "--kind",
                        "test",
                        "--result",
                        "pass",
                        "Ignored proof",
                    ],
                );
            }
            _ => unreachable!(),
        }
        assert!(git(fixture.path(), &["status", "--porcelain"]).is_empty());
        fixture.refusal("canonical_snapshot_uncommitted");
    }
}

#[test]
fn ignored_canonical_file_created_by_shell_is_rejected_at_final_fence() {
    let fixture = Fixture::new();
    git(fixture.path(), &["switch", "main"]);
    let path = fixture.path().join(".atelier/workflow.yaml");
    let policy = fs::read_to_string(&path).unwrap();
    fs::write(&path, format!(
        "{policy}\nmerge_checks:\n  - shell.check: {{ command: 'mkdir -p .atelier/reviews; printf hidden > .atelier/reviews/hidden.md' }}\n"
    )).unwrap();
    commit(fixture.path(), "ignored record shell check");
    git(fixture.path(), &["switch", &fixture.branch]);
    git(fixture.path(), &["rebase", "main"]);
    fixture.approval("Reviewer");
    fs::write(
        fixture.path().join(".git/info/exclude"),
        ".atelier/reviews/\n",
    )
    .unwrap();
    fixture.refusal("canonical_snapshot_uncommitted");
    assert!(fixture.path().join(".atelier/reviews/hidden.md").exists());
    assert!(git(fixture.path(), &["status", "--porcelain"]).is_empty());
}

#[test]
fn git_index_flags_cannot_hide_uncommitted_completion_claim_bytes_or_modes() {
    for mode_change in [false, true] {
        let fixture = Fixture::new();
        fixture.approval("Reviewer");
        let record = fixture.issue();
        let relative = atelier_records::issue_record_path(&record.issue.id);
        let path = fixture.path().join(".atelier").join(&relative);
        if mode_change {
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                git(fixture.path(), &["config", "core.filemode", "false"]);
                fs::set_permissions(&path, fs::Permissions::from_mode(0o755)).unwrap();
            }
        } else {
            git(
                fixture.path(),
                &[
                    "update-index",
                    "--assume-unchanged",
                    &format!(".atelier/{}", relative.display()),
                ],
            );
            let mut record = record;
            record.sections.close_reason = None;
            fixture.write_issue(&record);
        }
        assert!(git(fixture.path(), &["status", "--porcelain"]).is_empty());
        fixture.refusal("canonical_snapshot_uncommitted");
    }
}

#[test]
fn direct_terminal_claim_requires_trusted_completion_fields() {
    for status in ["done", "superseded"] {
        let fixture = Fixture::new();
        let mut record = fixture.issue();
        record.issue.status = status.into();
        record.sections.close_reason = None;
        fixture.write_issue(&record);
        commit(
            fixture.path(),
            "direct terminal claim missing required reason",
        );
        fixture.approval("Reviewer");
        fixture.refusal("required_field.close_reason");
        record.sections.close_reason = Some("Completed claim with required reason".into());
        fixture.write_issue(&record);
        commit(fixture.path(), "record required completion reason");
        ok(fixture.path(), &["merge", &fixture.id, "--check"]);
    }
}

#[test]
fn ignored_canonical_changes_in_base_checkout_refuse_before_and_after_checks() {
    for during_check in [false, true] {
        let fixture = Fixture::new();
        let base = tempdir().unwrap();
        if during_check {
            git(fixture.path(), &["switch", "main"]);
            let path = fixture.path().join(".atelier/workflow.yaml");
            let policy = fs::read_to_string(&path).unwrap();
            fs::write(&path, format!(
                "{policy}\nmerge_checks:\n  - shell.check: {{ command: 'mkdir -p {}/.atelier/reviews; printf hidden > {}/.atelier/reviews/hidden.md' }}\n",
                base.path().display(), base.path().display(),
            )).unwrap();
            commit(fixture.path(), "base canonical mutation check");
            git(fixture.path(), &["switch", &fixture.branch]);
            git(fixture.path(), &["rebase", "main"]);
        }
        fixture.approval("Reviewer");
        fs::write(
            fixture.path().join(".git/info/exclude"),
            ".atelier/reviews/\n",
        )
        .unwrap();
        git(
            fixture.path(),
            &["worktree", "add", base.path().to_str().unwrap(), "main"],
        );
        if !during_check {
            fs::create_dir_all(base.path().join(".atelier/reviews")).unwrap();
            fs::write(base.path().join(".atelier/reviews/hidden.md"), "hidden").unwrap();
        }
        fixture.refusal("canonical_snapshot_uncommitted");
        assert_eq!(git(base.path(), &["branch", "--show-current"]), "main");
        assert!(git(base.path(), &["status", "--porcelain"]).is_empty());
    }
}

#[test]
fn shadow_tags_do_not_change_source_or_base_branch_identity() {
    for separate_base in [false, true] {
        let fixture = Fixture::new();
        fixture.approval("Reviewer");
        let head = git(fixture.path(), &["rev-parse", "HEAD"]);
        git(fixture.path(), &["tag", &fixture.branch, "HEAD"]);
        git(fixture.path(), &["tag", "main", "refs/heads/main"]);
        let base = tempdir().unwrap();
        if separate_base {
            git(
                fixture.path(),
                &["worktree", "add", base.path().to_str().unwrap(), "main"],
            );
        }
        ok(fixture.path(), &["merge", &fixture.id]);
        let target = if separate_base {
            base.path()
        } else {
            fixture.path()
        };
        assert_eq!(git(target, &["symbolic-ref", "HEAD"]), "refs/heads/main");
        assert_eq!(git(target, &["rev-parse", "refs/heads/main"]), head);
        assert_eq!(
            git(
                target,
                &["rev-parse", &format!("refs/tags/{}", fixture.branch)]
            ),
            head
        );
        let output = Command::new("git")
            .current_dir(target)
            .args([
                "show-ref",
                "--verify",
                &format!("refs/heads/{}", fixture.branch),
            ])
            .output()
            .unwrap();
        assert!(!output.status.success());
        assert!(git(target, &["status", "--porcelain"]).is_empty());
    }
}

#[test]
fn hidden_flagged_code_is_rejected_before_merge_and_after_shell_checks_in_both_checkouts() {
    for flag in ["--assume-unchanged", "--skip-worktree"] {
        for base_target in [false, true] {
            for during_check in [false, true] {
                let fixture = Fixture::new();
                let base = tempdir().unwrap();
                git(fixture.path(), &["switch", "main"]);
                fs::write(fixture.path().join("base.rs"), "base code\n").unwrap();
                if during_check {
                    let path = fixture.path().join(".atelier/workflow.yaml");
                    let policy = fs::read_to_string(&path).unwrap();
                    let command = if base_target {
                        format!(
                            "git -C {} update-index {flag} base.rs; printf hidden > {}/base.rs",
                            base.path().display(),
                            base.path().display()
                        )
                    } else {
                        format!("git update-index {flag} src.rs; printf hidden > src.rs")
                    };
                    fs::write(&path, format!("{policy}\nmerge_checks:\n  - shell.check: {{ command: '{command}' }}\n")).unwrap();
                }
                commit(
                    fixture.path(),
                    "base code and optional hidden mutation check",
                );
                git(fixture.path(), &["switch", &fixture.branch]);
                git(fixture.path(), &["rebase", "main"]);
                fixture.approval("Reviewer");
                if base_target {
                    git(
                        fixture.path(),
                        &["worktree", "add", base.path().to_str().unwrap(), "main"],
                    );
                }
                let target = if base_target {
                    base.path()
                } else {
                    fixture.path()
                };
                let code_path = if base_target { "base.rs" } else { "src.rs" };
                if !during_check {
                    git(target, &["update-index", flag, code_path]);
                    fs::write(target.join(code_path), "hidden").unwrap();
                    assert!(git(target, &["status", "--porcelain"]).is_empty());
                }
                let head = git(fixture.path(), &["rev-parse", "HEAD"]);
                fixture.refusal("tracked_snapshot_uncommitted");
                assert_eq!(
                    git(
                        fixture.path(),
                        &["rev-parse", &format!("refs/heads/{}", fixture.branch)]
                    ),
                    head
                );
                assert!(git(target, &["status", "--porcelain"]).is_empty());
            }
        }
    }
}

#[cfg(unix)]
#[test]
fn flagged_code_executable_mode_changes_are_rejected_despite_core_filemode_false() {
    use std::os::unix::fs::PermissionsExt;
    let fixture = Fixture::new();
    fixture.approval("Reviewer");
    git(fixture.path(), &["config", "core.filemode", "false"]);
    git(
        fixture.path(),
        &["update-index", "--assume-unchanged", "src.rs"],
    );
    fs::set_permissions(
        fixture.path().join("src.rs"),
        fs::Permissions::from_mode(0o755),
    )
    .unwrap();
    assert!(git(fixture.path(), &["status", "--porcelain"]).is_empty());
    fixture.refusal("tracked_snapshot_uncommitted");
}

#[test]
fn evidence_capture_rejects_hidden_flagged_code_before_and_during_command_without_append() {
    for flag in ["--assume-unchanged", "--skip-worktree"] {
        for during_command in [false, true] {
            let fixture = Fixture::new();
            if !during_command {
                git(fixture.path(), &["update-index", flag, "src.rs"]);
                fs::write(fixture.path().join("src.rs"), "hidden").unwrap();
            }
            let script = if during_command {
                format!("git update-index {flag} src.rs; printf hidden > src.rs")
            } else {
                "touch capture-child-ran".into()
            };
            let (passed, output) = cli(
                fixture.path(),
                &[
                    "evidence",
                    "record",
                    "--kind",
                    "test",
                    "--target",
                    &format!("issue/{}", fixture.id),
                    "--",
                    "sh",
                    "-c",
                    &script,
                ],
            );
            let expected = if during_command {
                "command changed code or policy"
            } else {
                "requires committed code and policy"
            };
            assert!(!passed && output.contains(expected), "{output}");
            assert!(!fixture.path().join("capture-child-ran").exists());
            let evidence = fixture.path().join(".atelier/evidence");
            assert!(!evidence.exists() || fs::read_dir(evidence).unwrap().next().is_none());
            assert!(git(fixture.path(), &["status", "--porcelain"]).is_empty());
        }
    }
}

#[test]
fn unchanged_flagged_files_allow_merge() {
    let fixture = Fixture::new();
    fixture.approval("Reviewer");
    git(
        fixture.path(),
        &["update-index", "--assume-unchanged", "src.rs"],
    );
    ok(fixture.path(), &["merge", &fixture.id, "--check"]);
}

#[cfg(unix)]
#[test]
fn execute_bits_hidden_by_core_filemode_do_not_block_merge_in_either_checkout() {
    use std::os::unix::fs::PermissionsExt;
    for base_target in [false, true] {
        for during_check in [false, true] {
            let fixture = Fixture::new();
            let base = tempdir().unwrap();
            git(fixture.path(), &["switch", "main"]);
            fs::write(fixture.path().join("base.rs"), "base code\n").unwrap();
            if during_check {
                let path = fixture.path().join(".atelier/workflow.yaml");
                let policy = fs::read_to_string(&path).unwrap();
                let command = if base_target {
                    format!("chmod +x {}/base.rs", base.path().display())
                } else {
                    "chmod +x src.rs".into()
                };
                fs::write(
                    &path,
                    format!(
                        "{policy}\nmerge_checks:\n  - shell.check: {{ command: '{command}' }}\n"
                    ),
                )
                .unwrap();
            }
            commit(fixture.path(), "base code and optional hidden mode check");
            git(fixture.path(), &["switch", &fixture.branch]);
            git(fixture.path(), &["rebase", "main"]);
            fixture.approval("Reviewer");
            git(fixture.path(), &["config", "core.filemode", "false"]);
            if base_target {
                git(
                    fixture.path(),
                    &["worktree", "add", base.path().to_str().unwrap(), "main"],
                );
            }
            let target = if base_target {
                base.path()
            } else {
                fixture.path()
            };
            let file = if base_target { "base.rs" } else { "src.rs" };
            assert_eq!(
                git(target, &["ls-files", "-v", "--", file]),
                format!("H {file}")
            );
            if !during_check {
                fs::set_permissions(target.join(file), fs::Permissions::from_mode(0o755)).unwrap();
                assert!(git(target, &["status", "--porcelain"]).is_empty());
            }
            let work_head = git(fixture.path(), &["rev-parse", "HEAD"]);
            // Git ignores the execute bit here, so merge does too.
            ok(fixture.path(), &["merge", &fixture.id, "--check"]);
            assert_eq!(
                git(
                    fixture.path(),
                    &["rev-parse", &format!("refs/heads/{}", fixture.branch)]
                ),
                work_head
            );
            assert!(git(target, &["status", "--porcelain"]).is_empty());
            assert_eq!(
                git(target, &["ls-files", "-v", "--", file]),
                format!("H {file}")
            );
        }
    }
}

#[cfg(unix)]
#[test]
fn evidence_capture_follows_core_filemode_for_execute_bits() {
    use std::os::unix::fs::PermissionsExt;
    for during_command in [false, true] {
        let fixture = Fixture::new();
        git(fixture.path(), &["config", "core.filemode", "false"]);
        if !during_command {
            fs::set_permissions(
                fixture.path().join("src.rs"),
                fs::Permissions::from_mode(0o755),
            )
            .unwrap();
        }
        let command = if during_command {
            "chmod +x src.rs"
        } else {
            "true"
        };
        let (passed, output) = cli(
            fixture.path(),
            &[
                "evidence",
                "record",
                "--kind",
                "test",
                "--target",
                &format!("issue/{}", fixture.id),
                "--",
                "sh",
                "-c",
                command,
            ],
        );
        assert!(passed, "{output}");
        let evidence = fixture.path().join(".atelier/evidence");
        assert_eq!(fs::read_dir(evidence).unwrap().count(), 1);
        assert_eq!(
            git(fixture.path(), &["ls-files", "-v", "--", "src.rs"]),
            "H src.rs"
        );
        assert_eq!(
            git(fixture.path(), &["status", "--porcelain"]),
            "?? .atelier/evidence/"
        );
    }
}
