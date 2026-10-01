use super::*;

fn count_markdown_records(dir: &Path, kind: &str) -> usize {
    std::fs::read_dir(dir.join(".atelier").join(kind))
        .unwrap()
        .filter_map(Result::ok)
        .filter(|entry| entry.path().extension().is_some_and(|ext| ext == "md"))
        .count()
}

fn proof_fixture(gated: bool) -> (tempfile::TempDir, String) {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    init_git_repo(dir.path());
    if gated {
        write_evidence_gated_workflow(dir.path());
    }
    let body = "## Goal\n\nVerify evidence freshness.\n\n## Done when\n\nThe focused test proves the expected behavior.\n\n## Scope\n\nsrc.rs\n\n## Context\n\nFresh explicit proof is required.";
    let (ok, _, error) = run_atelier_raw(
        dir.path(),
        &["issue", "create", "Fresh proof", "--body", body],
    );
    assert!(ok, "{error}");
    let id = issue_id_by_title(dir.path(), "Fresh proof");
    std::fs::write(dir.path().join("src.rs"), "before\n").unwrap();
    commit_all(dir.path(), "acceptance and code before proof");
    for transition in ["start", "request_review"] {
        let (ok, _, error) = run_atelier_raw(dir.path(), &["issue", "transition", &id, transition]);
        assert!(ok, "{error}");
    }
    commit_all(dir.path(), "work ready for proof");
    (dir, id)
}

fn manual(dir: &Path, id: &str, result: &str, title: &str) -> String {
    let (ok, _, error) = run_atelier_raw(
        dir,
        &[
            "evidence",
            "record",
            "--target",
            &format!("issue/{id}"),
            "--kind",
            "validation",
            "--result",
            result,
            title,
        ],
    );
    assert!(ok, "{error}");
    record_id_by_title(dir, "evidence", title)
}
fn close(dir: &Path, id: &str) -> (bool, String, String) {
    run_atelier_raw(
        dir,
        &[
            "issue",
            "transition",
            id,
            "close",
            "--reason",
            "fresh proof",
        ],
    )
}

#[test]
fn manual_proof_requires_explicit_result_and_full_commit() {
    let (dir, id) = proof_fixture(true);
    let count = count_markdown_records(dir.path(), "evidence");
    let (ok, _, error) = run_atelier_raw(
        dir.path(),
        &[
            "evidence",
            "record",
            "--kind",
            "validation",
            "--target",
            &format!("issue/{id}"),
            "Result omitted",
        ],
    );
    assert!(
        !ok && error.contains("manual evidence requires --result"),
        "{error}"
    );
    assert_eq!(count_markdown_records(dir.path(), "evidence"), count);
    for result in ["blocked", "not-applicable", "fail"] {
        let evidence_id = manual(dir.path(), &id, result, result);
        let metadata = canonical_evidence_front_matter(dir.path(), &evidence_id);
        assert_eq!(metadata["result"], result);
        assert_eq!(metadata["commit"], git_rev_parse(dir.path(), "HEAD").trim());
        assert!(
            !close(dir.path(), &id).0,
            "{result} must not count as a pass"
        );
    }
    let (ok, list, error) = run_atelier_raw(dir.path(), &["evidence", "list", "--result", "fail"]);
    assert!(
        ok && list.contains("1 total") && list.contains("fail"),
        "{list} {error}"
    );
    manual(dir.path(), &id, "pass", "Fresh explicit pass");
    let (ok, _, error) = close(dir.path(), &id);
    assert!(ok, "{error}");
}

#[test]
fn captured_results_pin_commit_and_report_pass_fail_and_blocked() {
    let (dir, id) = proof_fixture(true);
    for (command, result, summary) in [
        ("exit 0", "pass", "Command pass"),
        ("exit 2", "fail", "Command fail"),
    ] {
        let (ok, _, error) = run_atelier_raw(
            dir.path(),
            &[
                "evidence",
                "record",
                "--target",
                &format!("issue/{id}"),
                "--kind",
                "test",
                "--summary",
                summary,
                "--",
                "sh",
                "-c",
                command,
            ],
        );
        assert!(ok, "{error}");
        let record = canonical_evidence_front_matter(
            dir.path(),
            &record_id_by_title(dir.path(), "evidence", summary),
        );
        assert_eq!(record["result"], result);
        assert_eq!(record["commit"], git_rev_parse(dir.path(), "HEAD").trim());
    }
    let (ok, _, error) = run_atelier_raw(
        dir.path(),
        &[
            "evidence",
            "record",
            "--kind",
            "test",
            "--summary",
            "Cannot start",
            "--",
            "atelier-impossible-executable-xyz",
        ],
    );
    assert!(ok, "{error}");
    let record = canonical_evidence_front_matter(
        dir.path(),
        &record_id_by_title(dir.path(), "evidence", "Cannot start"),
    );
    assert_eq!(record["result"], "blocked");
    let (ok, _, error) = run_atelier_raw(
        dir.path(),
        &[
            "evidence", "record", "--kind", "test", "--result", "pass", "--", "sh", "-c", "exit 2",
        ],
    );
    assert!(!ok && error.contains("derives its result"), "{error}");
}

#[test]
fn stale_code_and_acceptance_changes_require_new_proof_including_reverts() {
    for scope in ["code", "acceptance", "policy"] {
        let (dir, id) = proof_fixture(true);
        manual(dir.path(), &id, "pass", "Original proof");
        commit_all(dir.path(), "original passing proof");
        let path = if scope == "code" {
            dir.path().join("src.rs")
        } else if scope == "policy" {
            dir.path().join(".atelier/workflow.yaml")
        } else {
            canonical_issue_path(dir.path(), &id)
        };
        let original = std::fs::read_to_string(&path).unwrap();
        let changed = if scope == "code" {
            "after\n".to_string()
        } else if scope == "policy" {
            format!("{original}\n# changed policy\n")
        } else {
            original.replace(
                "The focused test proves the expected behavior.",
                "A different acceptance check proves the behavior.",
            )
        };
        std::fs::write(&path, &changed).unwrap();
        assert!(
            !close(dir.path(), &id).0,
            "uncommitted {scope} must make proof stale"
        );
        commit_all(dir.path(), "relevant change after proof");
        assert!(
            !close(dir.path(), &id).0,
            "committed {scope} must make proof stale"
        );
        std::fs::write(&path, &original).unwrap();
        commit_all(dir.path(), "revert relevant change");
        assert!(
            !close(dir.path(), &id).0,
            "reverting {scope} must not revive old proof"
        );
        manual(
            dir.path(),
            &id,
            "pass",
            "Proof after latest relevant change",
        );
        let (ok, _, error) = close(dir.path(), &id);
        assert!(ok, "fresh proof after {scope} should satisfy gate: {error}");
    }
}

#[test]
fn newer_failure_blocks_completion_without_presence_gate_and_stale_pass_cannot_clear_it() {
    let (dir, id) = proof_fixture(false);
    manual(dir.path(), &id, "pass", "First pass");
    manual(dir.path(), &id, "fail", "New failure");
    let (ok, _, error) = close(dir.path(), &id);
    assert!(!ok && error.contains("newer failing evidence"), "{error}");
    manual(dir.path(), &id, "pass", "Pass before code changed");
    std::fs::write(dir.path().join("src.rs"), "after failure\n").unwrap();
    commit_all(dir.path(), "changed code after latest pass");
    assert!(
        !close(dir.path(), &id).0,
        "a stale pass cannot clear the failure"
    );
    manual(dir.path(), &id, "pass", "Fresh recovery");
    let (ok, _, error) = close(dir.path(), &id);
    assert!(ok, "{error}");
}

#[test]
fn notes_status_and_new_evidence_records_do_not_stale_passing_proof() {
    let (dir, id) = proof_fixture(true);
    manual(dir.path(), &id, "pass", "Passing proof");
    let (ok, _, error) = run_atelier_raw(
        dir.path(),
        &[
            "issue",
            "note",
            &id,
            "Handoff after verification",
            "--kind",
            "handoff",
        ],
    );
    assert!(ok, "{error}");
    commit_all(dir.path(), "proof and handoff notes");
    let (ok, _, error) = run_atelier_raw(dir.path(), &["issue", "transition", &id, "revise"]);
    assert!(ok, "{error}");
    commit_all(dir.path(), "status bookkeeping");
    let (ok, _, error) =
        run_atelier_raw(dir.path(), &["issue", "transition", &id, "request_review"]);
    assert!(ok, "{error}");
    manual(
        dir.path(),
        &id,
        "not-applicable",
        "Another recorded observation",
    );
    commit_all(dir.path(), "more evidence bookkeeping");
    let (ok, _, error) = close(dir.path(), &id);
    assert!(ok, "{error}");
}

#[test]
fn capture_refuses_uncommitted_code_and_head_changes_without_partial_evidence() {
    let (dir, id) = proof_fixture(true);
    std::fs::write(dir.path().join("src.rs"), "dirty\n").unwrap();
    let before = count_markdown_records(dir.path(), "evidence");
    let (ok, _, error) = run_atelier_raw(
        dir.path(),
        &[
            "evidence", "record", "--kind", "test", "--", "sh", "-c", "exit 0",
        ],
    );
    assert!(!ok && error.contains("requires committed code"), "{error}");
    assert_eq!(count_markdown_records(dir.path(), "evidence"), before);
    commit_all(dir.path(), "code before capture");
    let (ok, _, error) = run_atelier_raw(
        dir.path(),
        &[
            "evidence",
            "record",
            "--kind",
            "test",
            "--target",
            &format!("issue/{id}"),
            "--",
            "git",
            "commit",
            "--allow-empty",
            "-m",
            "changed HEAD during capture",
        ],
    );
    assert!(!ok && error.contains("changed HEAD"), "{error}");
    assert_eq!(count_markdown_records(dir.path(), "evidence"), before);
}

#[test]
fn concurrent_label_updates_preserve_every_read_modify_write() {
    let (dir, id) = proof_fixture(false);
    let mut children = Vec::new();
    for number in 0..24 {
        children.push(
            Command::new(env!("CARGO_BIN_EXE_atelier"))
                .current_dir(dir.path())
                .args([
                    "issue",
                    "update",
                    &id,
                    "--label",
                    &format!("concurrent-{number}"),
                ])
                .stdout(std::process::Stdio::null())
                .stderr(std::process::Stdio::piped())
                .spawn()
                .unwrap(),
        );
    }
    for child in children {
        let result = child.wait_with_output().unwrap();
        assert!(
            result.status.success(),
            "{}",
            String::from_utf8_lossy(&result.stderr)
        );
    }
    let metadata = canonical_record_front_matter(dir.path(), "issues", &id);
    for number in 0..24 {
        assert!(
            metadata["labels"]
                .as_array()
                .unwrap()
                .contains(&serde_json::json!(format!("concurrent-{number}"))),
            "lost label {number}"
        );
    }
}

#[test]
fn public_bundle_fixtures_preview_and_apply_current_record_contract() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    let fixture = workspace_root().join("docs/spec/bundle/fixtures/valid-bundle.json");
    let (ok, preview, error) = run_atelier_raw(
        dir.path(),
        &["bundle", "preview", fixture.to_str().unwrap()],
    );
    assert!(ok, "{error}");
    assert!(preview.contains("issues: 4"), "{preview}");
    assert!(preview.contains("evidence: 1"), "{preview}");
    let (ok, _, error) = run_atelier_raw(
        dir.path(),
        &["bundle", "apply", fixture.to_str().unwrap(), "--yes"],
    );
    assert!(ok, "{error}");
    let (ok, _, error) = run_atelier_raw(dir.path(), &["check"]);
    assert!(ok, "{error}");
    let invalid = workspace_root().join("docs/spec/bundle/fixtures/invalid-bundle.json");
    let (ok, _, error) = run_atelier_raw(
        dir.path(),
        &["bundle", "preview", invalid.to_str().unwrap()],
    );
    assert!(
        !ok && error.contains("unknown field `milestones`"),
        "{error}"
    );
}

#[test]
fn objective_readiness_requires_agent_factory_acceptance_content() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    for (kind, heading) in [("epic", "Proof"), ("mission", "Success criteria")] {
        let (ok, _, error) = run_atelier_raw(
            dir.path(),
            &[
                "issue",
                "create",
                kind,
                "--issue-type",
                kind,
                "--body",
                "## Outcome\n\nObservable increment.",
            ],
        );
        assert!(ok, "{error}");
        let id = issue_id_by_title(dir.path(), kind);
        let (ok, _, error) = run_atelier_raw(dir.path(), &["issue", "transition", &id, "ready"]);
        assert!(!ok && error.contains(heading), "{error}");
        edit_canonical_issue(dir.path(), &id, |markdown| {
            format!("{markdown}\n## {heading}\n\nThe focused acceptance scenario passes.\n")
        });
        let (ok, _, error) = run_atelier_raw(dir.path(), &["issue", "transition", &id, "ready"]);
        assert!(ok, "{error}");
    }
}
