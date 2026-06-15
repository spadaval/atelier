use atelier_core::{DomainRecord, Issue, PlanRecordData, PlanRevision};
use atelier_records::{
    canonical_record_kind, canonical_record_path, issue_record_path, CanonicalDomainRecord,
    CanonicalIssueRecord, IssueSections, RecordStore, RelatesRelationship, Relationships,
};
use chrono::{TimeZone, Utc};
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
