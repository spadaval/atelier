use atelier_core::{DomainRecord, Issue, PlanRecordData, PlanRevision};
use atelier_records::{
    canonical_record_kind, canonical_record_path, issue_record_path, list_all_issue_activities,
    list_all_mission_activities, parse_domain_record, parse_issue_record, record_activity_path,
    render_domain_record, render_issue_record, CanonicalDomainRecord, CanonicalIssueRecord,
    IssueSections, RecordKindSpec, RecordStore, RelatesRelationship, Relationships,
    FIRST_CLASS_RECORD_KINDS,
};
use chrono::{TimeZone, Utc};
use std::fs;
use std::path::{Path, PathBuf};
use tempfile::tempdir;

fn fixed_time() -> chrono::DateTime<Utc> {
    Utc.with_ymd_and_hms(2026, 6, 15, 12, 0, 0).unwrap()
}

fn issue_record(id: &str, title: &str, status: &str) -> CanonicalIssueRecord {
    let now = fixed_time();
    CanonicalIssueRecord {
        issue: Issue {
            id: id.to_string(),
            title: title.to_string(),
            description: None,
            status: status.to_string(),
            issue_type: "task".to_string(),
            priority: "high".to_string(),
            parent_id: None,
            created_at: now,
            updated_at: now,
            closed_at: None,
        },
        labels: vec!["tests".to_string(), "rust".to_string()],
        sections: IssueSections {
            description: "Exercise canonical issue rendering.".to_string(),
            outcome: "Issue round-trips through record APIs.".to_string(),
            evidence: "Crate-level integration test.".to_string(),
            notes: Some("Private command state is not required.".to_string()),
        },
        relationships: Relationships::default(),
    }
}

#[test]
fn issue_records_round_trip_through_canonical_store() {
    let dir = tempdir().unwrap();
    let store = RecordStore::new(dir.path());
    let mut record = issue_record("atelier-0abc", "Boundary test", "todo");
    record.relationships.relates.push(RelatesRelationship {
        kind: "issue".to_string(),
        id: "atelier-0def".to_string(),
        relation_type: "related".to_string(),
    });

    store.write_issue_atomic(&record).unwrap();
    let loaded = store.load_issue_by_id("atelier-0abc").unwrap();

    assert_eq!(loaded.issue.id, "atelier-0abc");
    assert_eq!(loaded.issue.title, "Boundary test");
    assert_eq!(
        loaded.sections.searchable_text(),
        "Exercise canonical issue rendering.\nIssue round-trips through record APIs.\nCrate-level integration test.\nPrivate command state is not required."
    );
    assert_eq!(
        store.discover_issue_paths().unwrap(),
        vec![issue_record_path("atelier-0abc")]
    );
}

#[test]
fn domain_records_render_parse_and_mutate_relationships() {
    let dir = tempdir().unwrap();
    let store = RecordStore::new(dir.path());
    let now = fixed_time();
    let data = PlanRecordData {
        revision: 1,
        owner: Some("tests".to_string()),
        revisions: vec![PlanRevision {
            revision: 1,
            reason: "initial".to_string(),
            body: "Plan body".to_string(),
        }],
    };
    let record = CanonicalDomainRecord {
        record: DomainRecord {
            id: "atelier-0fed".to_string(),
            kind: "plan".to_string(),
            title: "Boundary plan".to_string(),
            status: "draft".to_string(),
            body: Some("Plan body".to_string()),
            data_json: serde_json::to_string(&data).unwrap(),
            created_at: now,
            updated_at: now,
        },
        labels: Vec::new(),
        relationships: Relationships::default(),
    };

    store.write_domain_record_atomic(&record).unwrap();
    let spec = canonical_record_kind("plan").unwrap();
    assert_eq!(
        canonical_record_path(spec, "atelier-0fed").unwrap(),
        std::path::PathBuf::from("plans/atelier-0fed.md")
    );

    assert!(store
        .add_relates_relationship("plan", "atelier-0fed", "issue", "atelier-0abc", "related")
        .unwrap());
    assert!(!store
        .add_relates_relationship("plan", "atelier-0fed", "issue", "atelier-0abc", "related")
        .unwrap());

    let loaded = store
        .load_domain_record_by_id("plan", "atelier-0fed")
        .unwrap();
    assert_eq!(loaded.record.kind, "plan");
    assert_eq!(loaded.record.title, "Boundary plan");
    assert_eq!(loaded.relationships.relates.len(), 1);

    assert!(store
        .remove_relates_relationship("plan", "atelier-0fed", "issue", "atelier-0abc", "related")
        .unwrap());
    assert!(store
        .load_domain_record_by_id("plan", "atelier-0fed")
        .unwrap()
        .relationships
        .relates
        .is_empty());
}

#[test]
fn committed_canonical_records_round_trip_through_records_apis() {
    let state_dir = repo_state_dir();
    assert!(
        state_dir.exists(),
        "expected repository canonical tracker state at {}",
        state_dir.display()
    );

    let store = RecordStore::new(&state_dir);
    let issue_paths = store.discover_issue_paths().unwrap();
    assert!(!issue_paths.is_empty(), "expected committed issue records");

    for relative in issue_paths {
        let record = store.load_issue(&relative).unwrap();
        let rendered = render_issue_record(&record).unwrap();
        assert_stable_issue_round_trip(&relative, &record, &rendered);
        assert_no_legacy_or_duplicate_relationship_surface(&relative, &rendered);
    }

    let mut covered_domain_kinds = Vec::new();
    for spec in FIRST_CLASS_RECORD_KINDS {
        let paths = collect_domain_record_paths(&state_dir, spec).unwrap();
        if paths.is_empty() {
            continue;
        }
        covered_domain_kinds.push(spec.kind);
        for relative in paths {
            let record = store.load_domain_record(&relative, spec).unwrap();
            let rendered = render_domain_record(&record).unwrap();
            assert_stable_domain_round_trip(&relative, spec, &record, &rendered);
            assert_no_legacy_or_duplicate_relationship_surface(&relative, &rendered);
        }
    }

    assert!(
        covered_domain_kinds.contains(&"mission"),
        "expected committed mission records"
    );
    assert!(
        covered_domain_kinds.contains(&"evidence"),
        "expected committed evidence records"
    );

    let issue_activities = list_all_issue_activities(&state_dir).unwrap();
    assert!(
        !issue_activities.is_empty(),
        "expected committed issue activity sidecars"
    );
    for activity in issue_activities {
        let relative =
            record_activity_path(&activity.subject_kind, &activity.subject_id, &activity.id);
        let rendered = activity.to_markdown().unwrap();
        let reparsed = atelier_records::IssueActivity::from_markdown(&rendered, &relative).unwrap();
        assert_eq!(
            reparsed, activity,
            "activity round-trip changed {relative:?}"
        );
        assert_eq!(
            reparsed.to_markdown().unwrap(),
            rendered,
            "activity render was not deterministic for {relative:?}"
        );
        assert!(
            !rendered.contains("\ndata: "),
            "activity {relative:?} rendered data payload"
        );
    }

    for activity in list_all_mission_activities(&state_dir).unwrap() {
        let relative =
            record_activity_path(&activity.subject_kind, &activity.subject_id, &activity.id);
        let rendered = activity.to_markdown().unwrap();
        let reparsed = atelier_records::IssueActivity::from_markdown(&rendered, &relative).unwrap();
        assert_eq!(
            reparsed, activity,
            "mission activity round-trip changed {relative:?}"
        );
        assert_eq!(
            reparsed.to_markdown().unwrap(),
            rendered,
            "mission activity render was not deterministic for {relative:?}"
        );
    }
}

#[test]
fn committed_plan_and_milestone_fixtures_round_trip_through_records_apis() {
    let state_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join("canonical_state");
    let store = RecordStore::new(&state_dir);

    for kind in ["plan", "milestone"] {
        let spec = canonical_record_kind(kind).unwrap();
        let paths = collect_domain_record_paths(&state_dir, spec).unwrap();
        assert!(!paths.is_empty(), "expected {kind} fixture records");
        for relative in paths {
            let record = store.load_domain_record(&relative, spec).unwrap();
            let rendered = render_domain_record(&record).unwrap();
            assert_stable_domain_round_trip(&relative, spec, &record, &rendered);
            assert_no_legacy_or_duplicate_relationship_surface(&relative, &rendered);
        }
    }
}

fn repo_state_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("crate lives below repository root")
        .join(".atelier")
}

fn collect_domain_record_paths(
    state_dir: &Path,
    spec: &RecordKindSpec,
) -> std::io::Result<Vec<PathBuf>> {
    let dir = state_dir.join(spec.canonical_dir.expect("canonical record kind has a dir"));
    if !dir.exists() {
        return Ok(Vec::new());
    }
    let mut paths = Vec::new();
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|extension| extension.to_str()) == Some("md") {
            paths.push(path.strip_prefix(state_dir).unwrap().to_path_buf());
        }
    }
    paths.sort();
    Ok(paths)
}

fn assert_stable_issue_round_trip(relative: &Path, record: &CanonicalIssueRecord, rendered: &str) {
    let reparsed = parse_issue_record(rendered, relative).unwrap();
    assert_eq!(
        &reparsed,
        record,
        "issue parse/render round-trip changed {}",
        relative.display()
    );
    assert_eq!(
        render_issue_record(&reparsed).unwrap(),
        rendered,
        "issue render was not deterministic for {}",
        relative.display()
    );
}

fn assert_stable_domain_round_trip(
    relative: &Path,
    spec: &RecordKindSpec,
    record: &CanonicalDomainRecord,
    rendered: &str,
) {
    let reparsed = parse_domain_record(rendered, relative, spec).unwrap();
    assert_eq!(
        &reparsed,
        record,
        "{} parse/render round-trip changed {}",
        spec.kind,
        relative.display()
    );
    assert_eq!(
        render_domain_record(&reparsed).unwrap(),
        rendered,
        "{} render was not deterministic for {}",
        spec.kind,
        relative.display()
    );
}

fn assert_no_legacy_or_duplicate_relationship_surface(relative: &Path, rendered: &str) {
    let front_matter = rendered
        .strip_prefix("---\n")
        .and_then(|rest| {
            rest.split_once("\n---\n")
                .map(|(front_matter, _)| front_matter)
        })
        .expect("rendered canonical record includes YAML front matter");
    assert!(
        !front_matter.contains("\ndata: "),
        "{} rendered a generic escaped data payload",
        relative.display()
    );
    assert_eq!(
        front_matter.matches("\nrelationships:\n").count(),
        1,
        "{} rendered duplicate relationship surfaces",
        relative.display()
    );
}
