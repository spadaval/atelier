use super::*;
use atelier_records::activity::{
    create_issue_activity, create_mission_plan_review_activity,
    create_workflow_transition_activity, ActivityEventType, MissionPlanStartAuthorization,
    WorkflowTransitionActivity,
};
use atelier_records::mission_plan_review::{
    mission_graph_revision, MissionPlanFindingSeverity, MissionPlanReviewEvent,
};

const PLANNER: &str = "actor-v1:tests.atelier.local/planner-1";
const REVIEWER: &str = "actor-v1:tests.atelier.local/reviewer-1";

fn run_plan_review(
    dir: &Path,
    mission_id: &str,
    actor: &str,
    action: &[&str],
) -> (bool, String, String) {
    let mut args = vec!["issue", "plan-review", mission_id];
    args.extend_from_slice(action);
    run_atelier_with_env(dir, &args, &[("ATELIER_AUTHENTICATED_ACTOR", actor)])
}

fn create_mission(dir: &Path, title: &str) -> String {
    let (success, _, stderr) =
        run_atelier(dir, &["issue", "create", title, "--issue-type", "mission"]);
    assert!(success, "mission creation failed: {stderr}");
    issue_id_by_title(dir, title)
}

fn mission_activity_count(dir: &Path, mission_id: &str) -> usize {
    let activity_dir = dir
        .join(".atelier/issues")
        .join(format!("{mission_id}.activity"));
    std::fs::read_dir(activity_dir)
        .map(|entries| entries.count())
        .unwrap_or(0)
}

fn assert_check_and_rebuild_green(dir: &Path) {
    for args in [vec!["check"], vec!["rebuild"]] {
        let (success, _, stderr) = run_atelier(dir, &args);
        assert!(success, "{} failed: {stderr}", args.join(" "));
    }
}

fn append_review_event(dir: &Path, mission_id: &str, actor: &str, event: MissionPlanReviewEvent) {
    create_mission_plan_review_activity(
        &dir.join(".atelier"),
        mission_id,
        actor,
        chrono::Utc::now(),
        "Mission-plan review fixture event",
        event,
        "Public lifecycle integration fixture.",
    )
    .unwrap();
}

fn current_revision(
    dir: &Path,
    mission_id: &str,
) -> atelier_records::mission_plan_review::MissionGraphRevision {
    mission_graph_revision(&dir.join(".atelier"), mission_id).unwrap()
}

fn append_request(
    dir: &Path,
    mission_id: &str,
) -> atelier_records::mission_plan_review::MissionGraphRevision {
    let revision = current_revision(dir, mission_id);
    append_review_event(
        dir,
        mission_id,
        PLANNER,
        MissionPlanReviewEvent::Request {
            graph_revision: revision.clone(),
            authors: vec![PLANNER.to_string()],
            material_editors: vec![PLANNER.to_string()],
        },
    );
    revision
}

fn request_transition(dir: &Path, mission_id: &str) {
    let (success, stdout, stderr) = run_atelier(
        dir,
        &["issue", "transition", mission_id, "request_plan_review"],
    );
    assert!(success, "request_plan_review failed: {stderr}");
    assert!(stdout.contains("To:       plan_review"), "{stdout}");
}

#[test]
fn mission_plan_review_public_transitions_reject_missing_and_non_independent_approval() {
    let not_requested = tempdir().unwrap();
    init_atelier(not_requested.path());
    let mission_id = create_mission(not_requested.path(), "Review not requested");
    request_transition(not_requested.path(), &mission_id);
    let (success, _, stderr) = run_atelier(
        not_requested.path(),
        &["issue", "transition", &mission_id, "ready"],
    );
    assert!(!success);
    assert!(
        stderr.contains("mission-plan review was not requested"),
        "{stderr}"
    );

    let missing = tempdir().unwrap();
    init_atelier(missing.path());
    let mission_id = create_mission(missing.path(), "Approval missing");
    let (success, _, stderr) = run_plan_review(missing.path(), &mission_id, PLANNER, &["request"]);
    assert!(success, "public review request failed: {stderr}");
    let (success, _, stderr) = run_atelier(
        missing.path(),
        &["issue", "transition", &mission_id, "ready"],
    );
    assert!(!success);
    assert!(stderr.contains("has no independent approval"), "{stderr}");

    let self_approved = tempdir().unwrap();
    init_atelier(self_approved.path());
    let mission_id = create_mission(self_approved.path(), "Self approval rejected");
    let (success, _, stderr) =
        run_plan_review(self_approved.path(), &mission_id, PLANNER, &["request"]);
    assert!(success, "public review request failed: {stderr}");
    let (success, _, stderr) =
        run_plan_review(self_approved.path(), &mission_id, PLANNER, &["approve"]);
    assert!(!success);
    assert!(
        stderr.contains("Reviewer") && stderr.contains("not independent"),
        "{stderr}"
    );
}

#[test]
fn mission_plan_review_public_transitions_reject_findings_changes_and_stale_approval() {
    let finding = tempdir().unwrap();
    init_atelier(finding.path());
    let mission_id = create_mission(finding.path(), "Blocking finding");
    let revision = append_request(finding.path(), &mission_id);
    request_transition(finding.path(), &mission_id);
    append_review_event(
        finding.path(),
        &mission_id,
        REVIEWER,
        MissionPlanReviewEvent::Approval {
            graph_revision: revision.clone(),
        },
    );
    append_review_event(
        finding.path(),
        &mission_id,
        REVIEWER,
        MissionPlanReviewEvent::Finding {
            graph_revision: revision,
            finding_id: "finding-1".to_string(),
            severity: MissionPlanFindingSeverity::Blocking,
            affected_issue_ids: vec![mission_id.clone()],
            dependency_path: Vec::new(),
        },
    );
    let (success, _, stderr) = run_atelier(
        finding.path(),
        &["issue", "transition", &mission_id, "ready"],
    );
    assert!(!success);
    assert!(
        stderr.contains("unresolved blocking findings: finding-1"),
        "{stderr}"
    );

    let changes = tempdir().unwrap();
    init_atelier(changes.path());
    let mission_id = create_mission(changes.path(), "Changes requested");
    let revision = append_request(changes.path(), &mission_id);
    request_transition(changes.path(), &mission_id);
    append_review_event(
        changes.path(),
        &mission_id,
        REVIEWER,
        MissionPlanReviewEvent::Approval {
            graph_revision: revision.clone(),
        },
    );
    append_review_event(
        changes.path(),
        &mission_id,
        REVIEWER,
        MissionPlanReviewEvent::ChangeRequest {
            graph_revision: revision,
            request_id: "change-1".to_string(),
            affected_issue_ids: vec![mission_id.clone()],
            dependency_path: Vec::new(),
        },
    );
    let (success, _, stderr) = run_atelier(
        changes.path(),
        &["issue", "transition", &mission_id, "ready"],
    );
    assert!(!success);
    assert!(
        stderr.contains("unresolved change requests: change-1"),
        "{stderr}"
    );

    let stale = tempdir().unwrap();
    init_atelier(stale.path());
    let mission_id = create_mission(stale.path(), "Stale plan approval");
    let revision = append_request(stale.path(), &mission_id);
    request_transition(stale.path(), &mission_id);
    append_review_event(
        stale.path(),
        &mission_id,
        REVIEWER,
        MissionPlanReviewEvent::Approval {
            graph_revision: revision,
        },
    );
    edit_canonical_issue(stale.path(), &mission_id, |markdown| {
        replace_front_matter_scalar(&markdown, "title", "Materially revised mission")
    });
    let (success, _, stderr) =
        run_atelier(stale.path(), &["issue", "transition", &mission_id, "ready"]);
    assert!(!success);
    assert!(
        stderr.contains("mission-plan approval is stale"),
        "{stderr}"
    );
}

#[test]
fn public_plan_review_surface_records_findings_resolutions_changes_and_approval() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    let mission_id = create_mission(dir.path(), "Public review decisions");
    let (success, _, stderr) = run_plan_review(dir.path(), &mission_id, PLANNER, &["request"]);
    assert!(success, "request failed: {stderr}");
    let (success, _, stderr) = run_plan_review(
        dir.path(),
        &mission_id,
        REVIEWER,
        &["finding", "public-finding", "--affected", &mission_id],
    );
    assert!(success, "finding failed: {stderr}");
    let (success, _, stderr) = run_plan_review(dir.path(), &mission_id, REVIEWER, &["approve"]);
    assert!(!success);
    assert!(
        stderr.contains("blocking finding public-finding"),
        "{stderr}"
    );
    let (success, _, stderr) = run_plan_review(
        dir.path(),
        &mission_id,
        PLANNER,
        &[
            "resolve",
            "public-finding",
            "--disposition",
            "clarified scope",
        ],
    );
    assert!(success, "resolution failed: {stderr}");
    let (success, _, stderr) = run_plan_review(
        dir.path(),
        &mission_id,
        REVIEWER,
        &["change-request", "public-change", "--affected", &mission_id],
    );
    assert!(success, "change request failed: {stderr}");
    let (success, _, stderr) = run_plan_review(
        dir.path(),
        &mission_id,
        PLANNER,
        &["resolve", "public-change", "--disposition", "addressed"],
    );
    assert!(success, "change resolution failed: {stderr}");
    let (success, _, stderr) = run_plan_review(dir.path(), &mission_id, REVIEWER, &["approve"]);
    assert!(success, "approval failed: {stderr}");
    let (success, _, stderr) =
        run_atelier(dir.path(), &["issue", "transition", &mission_id, "ready"]);
    assert!(success, "ready transition failed: {stderr}");
}

#[test]
fn public_review_reference_failures_are_atomic_and_leave_canonical_state_green() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    let mission_id = create_mission(dir.path(), "Reference validation");
    let (success, _, stderr) = run_plan_review(dir.path(), &mission_id, PLANNER, &["request"]);
    assert!(success, "request failed: {stderr}");
    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "create", "Unrelated issue"]);
    assert!(success, "unrelated issue creation failed: {stderr}");
    let unrelated_id = issue_id_by_title(dir.path(), "Unrelated issue");
    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "create", "Scoped issue"]);
    assert!(success, "scoped issue creation failed: {stderr}");
    let scoped_id = issue_id_by_title(dir.path(), "Scoped issue");
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "link",
            &mission_id,
            &scoped_id,
            "--role",
            "advances",
        ],
    );
    assert!(success, "mission scope link failed: {stderr}");
    let baseline = mission_activity_count(dir.path(), &mission_id);

    for (actor, action, expected) in [
        (
            REVIEWER,
            vec![
                "finding",
                "missing-affected",
                "--affected",
                "atelier-does-not-exist",
            ],
            "outside the current mission graph",
        ),
        (
            REVIEWER,
            vec![
                "change-request",
                "missing-path",
                "--affected",
                mission_id.as_str(),
                "--dependency-path",
                mission_id.as_str(),
                "--dependency-path",
                "atelier-does-not-exist",
            ],
            "outside the current mission graph",
        ),
        (
            REVIEWER,
            vec![
                "finding",
                "unreachable-affected",
                "--affected",
                unrelated_id.as_str(),
            ],
            "outside the current mission graph",
        ),
        (
            REVIEWER,
            vec![
                "change-request",
                "non-edge-path",
                "--affected",
                mission_id.as_str(),
                "--dependency-path",
                mission_id.as_str(),
                "--dependency-path",
                scoped_id.as_str(),
            ],
            "contains non-edge",
        ),
    ] {
        let (success, _, stderr) =
            run_plan_review(dir.path(), &mission_id, actor, action.as_slice());
        assert!(!success, "invalid reference unexpectedly succeeded");
        assert!(stderr.contains(expected), "{stderr}");
        assert_eq!(mission_activity_count(dir.path(), &mission_id), baseline);
        assert_check_and_rebuild_green(dir.path());
    }
}

#[test]
fn public_rework_attributes_material_edits_and_resubmits_exact_revision() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    let mission_id = create_mission(dir.path(), "Rework lifecycle");
    let (success, _, stderr) = run_plan_review(dir.path(), &mission_id, PLANNER, &["request"]);
    assert!(success, "request failed: {stderr}");
    let (success, _, stderr) = run_plan_review(
        dir.path(),
        &mission_id,
        REVIEWER,
        &["change-request", "rework-change", "--affected", &mission_id],
    );
    assert!(success, "change request failed: {stderr}");
    let baseline = mission_activity_count(dir.path(), &mission_id);

    let (success, _, stderr) = run_plan_review(
        dir.path(),
        &mission_id,
        REVIEWER,
        &[
            "resolve",
            "rework-change",
            "--disposition",
            "reviewer cannot resolve own request",
        ],
    );
    assert!(!success);
    assert!(stderr.contains("cannot resolve rework-change"), "{stderr}");
    assert_eq!(mission_activity_count(dir.path(), &mission_id), baseline);
    let (success, _, stderr) = run_plan_review(
        dir.path(),
        &mission_id,
        PLANNER,
        &["resolve", "unknown-change", "--disposition", "not valid"],
    );
    assert!(!success);
    assert!(stderr.contains("unknown mission-plan decision"), "{stderr}");
    assert_eq!(mission_activity_count(dir.path(), &mission_id), baseline);

    let (success, _, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "update",
            &mission_id,
            "--title",
            "Reworked mission lifecycle",
        ],
    );
    assert!(success, "material edit failed: {stderr}");
    let (success, _, stderr) = run_plan_review(dir.path(), &mission_id, REVIEWER, &["approve"]);
    assert!(!success);
    assert!(
        stderr.contains("incomplete author/material-editor provenance"),
        "{stderr}"
    );

    let (success, _, stderr) = run_plan_review(
        dir.path(),
        &mission_id,
        PLANNER,
        &[
            "resolve",
            "rework-change",
            "--disposition",
            "material edit addresses requested change",
        ],
    );
    assert!(success, "cross-revision resolution failed: {stderr}");
    let (success, stdout, stderr) = run_plan_review(dir.path(), &mission_id, PLANNER, &["request"]);
    assert!(success, "re-request failed: {stderr}");
    assert!(stdout.contains("Revision:"), "{stdout}");
    let (success, _, stderr) = run_plan_review(dir.path(), &mission_id, PLANNER, &["approve"]);
    assert!(!success);
    assert!(stderr.contains("not independent"), "{stderr}");
    let (success, _, stderr) = run_plan_review(dir.path(), &mission_id, REVIEWER, &["approve"]);
    assert!(success, "independent approval failed: {stderr}");
    let (success, _, stderr) =
        run_atelier(dir.path(), &["issue", "transition", &mission_id, "ready"]);
    assert!(success, "ready after rework failed: {stderr}");
    assert_check_and_rebuild_green(dir.path());
}

#[test]
fn rework_does_not_retarget_stale_prior_approval() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    let mission_id = create_mission(dir.path(), "Stale approval rework");
    let (success, _, stderr) = run_plan_review(dir.path(), &mission_id, PLANNER, &["request"]);
    assert!(success, "request failed: {stderr}");
    let (success, _, stderr) = run_plan_review(dir.path(), &mission_id, REVIEWER, &["approve"]);
    assert!(success, "initial approval failed: {stderr}");
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "update",
            &mission_id,
            "--title",
            "Stale approval reworked",
        ],
    );
    assert!(success, "material edit failed: {stderr}");
    let (success, _, stderr) = run_plan_review(dir.path(), &mission_id, PLANNER, &["rework"]);
    assert!(success, "rework failed: {stderr}");
    let (success, _, stderr) =
        run_atelier(dir.path(), &["issue", "transition", &mission_id, "ready"]);
    assert!(!success);
    assert!(
        stderr.contains("mission-plan approval is stale"),
        "{stderr}"
    );
    let (success, _, stderr) = run_plan_review(dir.path(), &mission_id, REVIEWER, &["approve"]);
    assert!(success, "fresh approval failed: {stderr}");
}

#[test]
fn public_plan_review_requires_authenticated_actor_binding() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    let mission_id = create_mission(dir.path(), "Missing actor binding");
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["issue", "plan-review", &mission_id, "request"],
    );
    assert!(!success);
    assert!(stderr.contains("ATELIER_AUTHENTICATED_ACTOR"), "{stderr}");
}

#[test]
fn exact_revision_approval_and_closed_dependencies_allow_ready_and_start() {
    let dir = tempdir().unwrap();
    init_git_repo(dir.path());
    init_atelier(dir.path());
    let mission_id = create_mission(dir.path(), "Approved mission");
    request_and_approve_mission_plan(dir.path(), &mission_id);
    let (success, stdout, stderr) =
        run_atelier(dir.path(), &["issue", "transition", &mission_id, "ready"]);
    assert!(success, "ready failed: {stderr}");
    assert!(stdout.contains("To:       ready"), "{stdout}");
    commit_all(dir.path(), "approved mission ready");
    let (success, stdout, stderr) =
        run_atelier(dir.path(), &["issue", "transition", &mission_id, "start"]);
    assert!(success, "start failed: {stderr}");
    assert!(stdout.contains("To:       in_progress"), "{stdout}");
}

#[test]
fn review_approval_does_not_override_open_dependency_blockers() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    let mission_id = create_mission(dir.path(), "Approved but blocked mission");
    let (success, _, stderr) = run_atelier(dir.path(), &["issue", "create", "Open prerequisite"]);
    assert!(success, "blocker creation failed: {stderr}");
    let blocker_id = issue_id_by_title(dir.path(), "Open prerequisite");
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &[
            "issue",
            "link",
            &mission_id,
            &blocker_id,
            "--role",
            "blocked_by",
        ],
    );
    assert!(success, "blocker link failed: {stderr}");
    request_and_approve_mission_plan(dir.path(), &mission_id);
    let (success, _, stderr) =
        run_atelier(dir.path(), &["issue", "transition", &mission_id, "ready"]);
    assert!(!success);
    assert!(
        stderr.contains(&format!("{mission_id} -> {blocker_id}")),
        "{stderr}"
    );
}

#[test]
fn canonical_and_supported_status_mutations_cannot_bypass_plan_review() {
    for status in ["ready", "in_progress"] {
        let dir = tempdir().unwrap();
        init_atelier(dir.path());
        let mission_id = create_mission(dir.path(), &format!("Direct {status} bypass"));
        edit_canonical_issue(dir.path(), &mission_id, |markdown| {
            replace_front_matter_scalar(&markdown, "status", status)
        });
        let (success, _, stderr) = run_atelier(dir.path(), &["rebuild"]);
        assert!(!success);
        assert!(
            stderr.contains("workflow_mission_plan_review_bypass"),
            "{stderr}"
        );
    }

    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    let mission_id = create_mission(dir.path(), "Update status bypass");
    let (success, _, stderr) = run_atelier(
        dir.path(),
        &["issue", "update", &mission_id, "--status", "ready"],
    );
    assert!(!success);
    assert!(
        stderr.contains("issue status changes use `atelier issue transition"),
        "{stderr}"
    );
}

#[test]
fn forged_generic_transition_activity_cannot_authorize_stale_direct_start() {
    let dir = tempdir().unwrap();
    init_atelier(dir.path());
    let mission_id = create_mission(dir.path(), "Forged generic start");
    request_and_approve_mission_plan(dir.path(), &mission_id);
    edit_canonical_issue(dir.path(), &mission_id, |markdown| {
        let markdown = replace_front_matter_scalar(&markdown, "title", "Changed after approval");
        replace_front_matter_scalar(&markdown, "status", "in_progress")
    });
    create_issue_activity(
        &dir.path().join(".atelier"),
        &mission_id,
        ActivityEventType::TransitionApplied,
        "attacker",
        chrono::Utc::now(),
        "Applied transition start (ready -> in_progress)",
        "transition: \"start\"\nfrom: \"ready\"\nto: \"in_progress\"",
    )
    .unwrap();
    let (success, _, stderr) = run_atelier(dir.path(), &["rebuild"]);
    assert!(!success);
    assert!(
        stderr.contains("workflow_mission_plan_review_bypass"),
        "{stderr}"
    );
}

#[test]
fn genuine_typed_start_receipt_remains_rebuild_safe_after_material_change() {
    let dir = tempdir().unwrap();
    init_git_repo(dir.path());
    init_atelier(dir.path());
    let mission_id = create_mission(dir.path(), "Genuine typed start");
    move_reviewed_mission_to_ready(dir.path(), &mission_id);
    commit_all(dir.path(), "reviewed mission ready");
    let (success, _, stderr) =
        run_atelier(dir.path(), &["issue", "transition", &mission_id, "start"]);
    assert!(success, "genuine start failed: {stderr}");
    edit_canonical_issue(dir.path(), &mission_id, |markdown| {
        replace_front_matter_scalar(&markdown, "title", "Changed after genuine start")
    });
    let (success, _, stderr) = run_atelier(dir.path(), &["rebuild"]);
    assert!(
        success,
        "typed start receipt did not survive rebuild: {stderr}"
    );
}

#[test]
fn canonical_executable_statuses_cannot_bypass_direct_or_transitive_blockers() {
    for (status, transitive) in [("ready", false), ("in_progress", true)] {
        let dir = tempdir().unwrap();
        init_atelier(dir.path());
        let mission_id = create_mission(dir.path(), &format!("{status} blocker bypass"));
        let (success, _, stderr) = run_atelier(dir.path(), &["issue", "create", "Direct gate"]);
        assert!(success, "direct blocker create failed: {stderr}");
        let direct_id = issue_id_by_title(dir.path(), "Direct gate");
        let (success, _, stderr) = run_atelier(
            dir.path(),
            &[
                "issue",
                "link",
                &mission_id,
                &direct_id,
                "--role",
                "blocked_by",
            ],
        );
        assert!(success, "direct blocker link failed: {stderr}");
        let expected_path = if transitive {
            let (success, _, stderr) =
                run_atelier(dir.path(), &["issue", "create", "Transitive gate"]);
            assert!(success, "transitive blocker create failed: {stderr}");
            let transitive_id = issue_id_by_title(dir.path(), "Transitive gate");
            let (success, _, stderr) = run_atelier(
                dir.path(),
                &[
                    "issue",
                    "link",
                    &direct_id,
                    &transitive_id,
                    "--role",
                    "blocked_by",
                ],
            );
            assert!(success, "transitive blocker link failed: {stderr}");
            format!("{mission_id} -> {direct_id} -> {transitive_id}")
        } else {
            format!("{mission_id} -> {direct_id}")
        };
        request_and_approve_mission_plan(dir.path(), &mission_id);
        edit_canonical_issue(dir.path(), &mission_id, |markdown| {
            replace_front_matter_scalar(&markdown, "status", status)
        });
        if status == "in_progress" {
            let state = atelier_records::mission_plan_review::mission_plan_review_state(
                &dir.path().join(".atelier"),
                &mission_id,
            )
            .unwrap();
            let authorization = state.authorization.unwrap();
            create_workflow_transition_activity(
                &dir.path().join(".atelier"),
                &mission_id,
                "workflow",
                chrono::Utc::now(),
                "Gated mission start",
                WorkflowTransitionActivity {
                    transition: "start".to_string(),
                    from: "ready".to_string(),
                    to: "in_progress".to_string(),
                    mission_plan_start: Some(MissionPlanStartAuthorization {
                        graph_revision: authorization.graph_revision,
                        approval_activity_id: authorization.activity_id,
                    }),
                },
                "canonical typed receipt fixture",
            )
            .unwrap();
        }
        let (success, _, stderr) = run_atelier(dir.path(), &["rebuild"]);
        assert!(!success);
        assert!(
            stderr.contains("workflow_mission_dependency_bypass"),
            "{stderr}"
        );
        assert!(stderr.contains(&expected_path), "{stderr}");
    }
}
