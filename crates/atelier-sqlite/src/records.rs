use anyhow::Result;

use super::{validate_record_kind, Database};
use crate::record_id;
use atelier_core::RecordLink;

#[derive(Debug, Clone, PartialEq)]
pub struct RecordSummary {
    pub kind: String,
    pub id: String,
    pub title: String,
    pub status: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub source_path: String,
}

impl Database {
    pub fn records_linking_to(&self, kind: &str, id: &str) -> Result<Vec<RecordLink>> {
        validate_record_kind(kind)?;
        record_id::validate_record_id(id)?;
        Ok(self
            .list_all_record_links()?
            .into_iter()
            .filter(|link| link.target_kind == kind && link.target_id == id)
            .collect())
    }

    pub fn get_record(&self, kind: &str, id: &str) -> Result<Option<RecordSummary>> {
        validate_record_kind(kind)?;
        Ok(self
            .list_records(kind, None)?
            .into_iter()
            .find(|record| record.id == id))
    }
    pub fn record_exists(&self, id: &str) -> Result<bool> {
        if self.get_issue(id)?.is_some() {
            return Ok(true);
        }
        Ok(self.evidence_cache_row(id)?.is_some() || self.review_room_cache_row(id)?.is_some())
    }

    pub fn record_kind_for_id(&self, id: &str) -> Result<Option<String>> {
        if record_id::validate_record_id(id).is_err() {
            return Ok(None);
        }
        if self.get_issue(id)?.is_some() {
            return Ok(Some("issue".to_string()));
        }

        if self.evidence_cache_row(id)?.is_some() {
            Ok(Some("evidence".to_string()))
        } else if self.review_room_cache_row(id)?.is_some() {
            Ok(Some("review".to_string()))
        } else {
            Ok(None)
        }
    }

    pub fn require_record(&self, kind: &str, id: &str) -> Result<RecordSummary> {
        self.get_record(kind, id)?
            .ok_or_else(|| anyhow::anyhow!("{} record {} not found", kind, id))
    }

    pub fn list_records(&self, kind: &str, status: Option<&str>) -> Result<Vec<RecordSummary>> {
        validate_record_kind(kind)?;
        let sources = self.record_source_cache_rows()?;
        let source_path = |record_kind: &str, id: &str| {
            sources
                .iter()
                .find(|source| source.record_kind == record_kind && source.record_id == id)
                .map(|source| source.path.clone())
                .unwrap_or_default()
        };
        let mut records = match kind {
            "evidence" => self
                .query_evidence_cache(&super::EvidenceCacheQuery {
                    status,
                    evidence_type: None,
                })?
                .into_iter()
                .map(|row| RecordSummary {
                    kind: "evidence".to_string(),
                    source_path: source_path("evidence", &row.id),
                    id: row.id,
                    title: row.title,
                    status: row.status,
                    created_at: row.created_at,
                    updated_at: row.updated_at,
                })
                .collect(),
            "review" => self
                .review_room_cache_rows()?
                .into_iter()
                .filter(|row| status.is_none_or(|status| row.status == status))
                .map(|row| RecordSummary {
                    kind: "review".to_string(),
                    source_path: source_path("review", &row.id),
                    id: row.id,
                    title: row.title,
                    status: row.status,
                    created_at: row.created_at,
                    updated_at: row.updated_at,
                })
                .collect(),
            _ => Vec::new(),
        };
        records.sort_by(|left, right| {
            right
                .updated_at
                .cmp(&left.updated_at)
                .then_with(|| right.id.cmp(&left.id))
        });
        Ok(records)
    }

    pub fn list_record_links(&self, kind: &str, id: &str) -> Result<Vec<RecordLink>> {
        validate_record_ref(self, kind, id)?;
        Ok(self
            .list_all_record_links()?
            .into_iter()
            .filter(|link| {
                (link.source_kind == kind && link.source_id == id)
                    || (link.target_kind == kind && link.target_id == id)
            })
            .collect())
    }

    pub fn list_all_record_links(&self) -> Result<Vec<RecordLink>> {
        let mut links = Vec::new();
        for evidence in self.query_evidence_cache(&super::EvidenceCacheQuery::default())? {
            for target in self.evidence_cache_targets(&evidence.id)? {
                links.push(RecordLink {
                    source_kind: "evidence".to_string(),
                    source_id: evidence.id.clone(),
                    target_kind: target.target_kind,
                    target_id: target.target_id,
                    relation_type: target.role,
                    created_at: evidence.created_at,
                });
            }
        }
        for issue in self.query_issue_cache(&super::IssueCacheQuery::default())? {
            for relation in self.issue_cache_relations(&issue.id)? {
                links.push(RecordLink {
                    source_kind: "issue".to_string(),
                    source_id: relation.source_issue_id,
                    target_kind: "issue".to_string(),
                    target_id: relation.target_issue_id,
                    relation_type: relation.relation_type,
                    created_at: relation.created_at,
                });
            }
        }
        links.sort_by(|left, right| {
            (
                &left.source_kind,
                &left.source_id,
                &left.target_kind,
                &left.target_id,
                &left.relation_type,
            )
                .cmp(&(
                    &right.source_kind,
                    &right.source_id,
                    &right.target_kind,
                    &right.target_id,
                    &right.relation_type,
                ))
        });
        Ok(links)
    }
}

fn validate_record_ref(db: &Database, kind: &str, id: &str) -> Result<()> {
    validate_record_kind(kind)?;
    record_id::validate_record_id(id)?;
    match kind {
        "issue" => {
            db.require_issue(id)?;
        }
        _ => {
            db.require_record(kind, id)?;
        }
    }
    Ok(())
}
