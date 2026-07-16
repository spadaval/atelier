use super::*;
use atelier_records::activity::create_mission_plan_review_activity;
use atelier_records::mission_plan_review::{
    mission_graph_revision, MissionPlanFindingSeverity, MissionPlanReviewEvent,
};

const PLANNER: &str = "actor-v1:tests.atelier.local/planner-1";
const REVIEWER: &str = "actor-v1:tests.atelier.local/reviewer-1";

fn create_mission(dir: &Path, title: &str) -> String {
    let (success, _, stderr) =
        run_atelier(dir, &["issue", "create", title, "--issue-type", "mission"]);
    assert!(success, "mission creation failed: {stderr}");
    issue_id_by_title(dir, title)
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
    append_request(missing.path(), &mission_id);
    request_transition(missing.path(), &mission_id);
    let (success, _, stderr) = run_atelier(
        missing.path(),
        &["issue", "transition", &mission_id, "ready"],
    );
    assert!(!success);
    assert!(stderr.contains("has no independent approval"), "{stderr}");

    let self_approved = tempdir().unwrap();
    init_atelier(self_approved.path());
    let mission_id = create_mission(self_approved.path(), "Self approval rejected");
    let revision = append_request(self_approved.path(), &mission_id);
    request_transition(self_approved.path(), &mission_id);
    append_review_event(
        self_approved.path(),
        &mission_id,
        PLANNER,
        MissionPlanReviewEvent::Approval {
            graph_revision: revision,
        },
    );
    let (success, _, stderr) = run_atelier(
        self_approved.path(),
        &["issue", "transition", &mission_id, "ready"],
    );
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
fn exact_revision_approval_and_closed_dependencies_allow_ready_and_start() {
    let dir = tempdir().unwrap();
    init_git_repo(dir.path());
    init_atelier(dir.path());
    let mission_id = create_mission(dir.path(), "Approved mission");
    let revision = append_request(dir.path(), &mission_id);
    request_transition(dir.path(), &mission_id);
    append_review_event(
        dir.path(),
        &mission_id,
        REVIEWER,
        MissionPlanReviewEvent::Approval {
            graph_revision: revision,
        },
    );
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
    let revision = append_request(dir.path(), &mission_id);
    request_transition(dir.path(), &mission_id);
    append_review_event(
        dir.path(),
        &mission_id,
        REVIEWER,
        MissionPlanReviewEvent::Approval {
            graph_revision: revision,
        },
    );
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
