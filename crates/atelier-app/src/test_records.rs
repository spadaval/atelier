//! Record fixtures for app-layer unit tests.

use atelier_core::{EvidenceRecord, EvidenceRecordData, Issue, IssueSections, RecordHeader};
use atelier_records::{
    attachment_relationship, issue_relates_relationship, issue_relationship_target,
    CanonicalIssueRecord, RelationshipTarget, Relationships,
};
use chrono::Utc;

use crate::record_index::RecordIndex;

pub fn issue_record(id: &str, issue_type: &str, status: &str) -> CanonicalIssueRecord {
    let now = Utc::now();
    CanonicalIssueRecord {
        issue: Issue {
            id: id.to_string(),
            title: id.to_string(),
            description: None,
            status: status.to_string(),
            issue_type: issue_type.to_string(),
            priority: "medium".to_string(),
            strategy: None,
            adr: None,
            workflow_branch: None,
            parent_id: None,
            created_at: now,
            updated_at: now,
            closed_at: None,
        },
        labels: Vec::new(),
        sections: IssueSections::for_type(issue_type, Some("Fixture issue.")),
        relationships: Relationships::default(),
    }
}

/// Evidence that attaches to `issue_id` with `role`.
pub fn evidence_record(id: &str, issue_id: &str, role: &str) -> EvidenceRecord {
    let now = Utc::now();
    EvidenceRecord {
        header: RecordHeader {
            kind: "evidence".to_string(),
            id: id.to_string(),
            title: id.to_string(),
            status: "pass".to_string(),
            labels: Vec::new(),
            relationships: Relationships {
                attachments: vec![attachment_relationship("issue", issue_id, role)],
                ..Relationships::default()
            },
            created_at: now,
            updated_at: now,
        },
        data: EvidenceRecordData {
            commit: "a".repeat(40),
            result: atelier_core::EvidenceResult::Pass,
            evidence_type: "validation".to_string(),
            captured_at: now,
            command: None,
            path: None,
            uri: None,
            producer: None,
            proof_scope: None,
            agent_identity: None,
            independence_level: None,
            residual_risks: Vec::new(),
            exit_code: None,
            exit_status: None,
            success: Some(true),
            spawn_error: None,
            output: None,
            target: None,
            criteria: Vec::new(),
            criteria_mission: None,
        },
        summary: "Fixture evidence.".to_string(),
    }
}

pub fn link(id: &str) -> RelationshipTarget {
    issue_relationship_target(id)
}

/// Mutable record set that builds a [`RecordIndex`].
#[derive(Default)]
pub struct Records {
    issues: Vec<CanonicalIssueRecord>,
    evidence: Vec<EvidenceRecord>,
}

impl Records {
    pub fn issue(&mut self, id: &str, issue_type: &str, status: &str) -> &mut CanonicalIssueRecord {
        self.issues.retain(|record| record.issue.id != id);
        self.issues.push(issue_record(id, issue_type, status));
        self.issues.last_mut().unwrap()
    }

    pub fn get(&mut self, id: &str) -> &mut CanonicalIssueRecord {
        self.issues
            .iter_mut()
            .find(|record| record.issue.id == id)
            .unwrap_or_else(|| panic!("fixture issue {id} not found"))
    }

    pub fn parent(&mut self, child: &str, parent: &str) {
        self.get(parent).relationships.children.push(link(child));
    }

    pub fn block(&mut self, blocked: &str, blocker: &str) {
        self.get(blocker).relationships.blocks.push(link(blocked));
    }

    pub fn relate(&mut self, source: &str, target: &str, relation_type: &str) {
        self.get(source)
            .relationships
            .relates
            .push(issue_relates_relationship(target, relation_type));
    }

    pub fn evidence(&mut self, record: EvidenceRecord) {
        self.evidence.push(record);
    }

    pub fn index(&self) -> RecordIndex {
        RecordIndex::from_records(self.issues.clone(), self.evidence.clone(), Vec::new()).unwrap()
    }
}
