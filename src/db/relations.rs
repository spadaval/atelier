use anyhow::Result;
use chrono::Utc;
use rusqlite::params;

use super::{issue_from_row, Database};
use crate::models::{Issue, Relation};

const TRANSITIVE_IMPACT_RELATIONS: &[&str] = &["derived", "caused-by", "falsifies"];

impl Database {
    /// Add a typed relation between two issues.
    /// Defaults to "related" if no type is specified.
    pub fn add_typed_relation(
        &self,
        issue_id_1: impl ToString,
        issue_id_2: impl ToString,
        relation_type: &str,
    ) -> Result<bool> {
        let issue_id_1 = issue_id_1.to_string();
        let issue_id_2 = issue_id_2.to_string();
        if issue_id_1 == issue_id_2 {
            anyhow::bail!("Cannot relate an issue to itself");
        }
        let (a, b) = if issue_id_1 < issue_id_2 {
            (issue_id_1, issue_id_2)
        } else {
            (issue_id_2, issue_id_1)
        };
        let now = Utc::now().to_rfc3339();
        let result = self.conn.execute(
            "INSERT OR IGNORE INTO relations (issue_id_1, issue_id_2, relation_type, created_at) VALUES (?1, ?2, ?3, ?4)",
            params![a, b, relation_type, now],
        )?;
        Ok(result > 0)
    }

    /// Backward-compatible: add a "related" relation.
    #[cfg(test)]
    pub fn add_relation(
        &self,
        issue_id_1: impl ToString,
        issue_id_2: impl ToString,
    ) -> Result<bool> {
        self.add_typed_relation(issue_id_1, issue_id_2, "related")
    }

    /// Remove a typed relation between two issues.
    #[cfg(test)]
    pub fn remove_typed_relation(
        &self,
        issue_id_1: impl ToString,
        issue_id_2: impl ToString,
        relation_type: &str,
    ) -> Result<bool> {
        let issue_id_1 = issue_id_1.to_string();
        let issue_id_2 = issue_id_2.to_string();
        let (a, b) = if issue_id_1 < issue_id_2 {
            (issue_id_1, issue_id_2)
        } else {
            (issue_id_2, issue_id_1)
        };
        let rows = self.conn.execute(
            "DELETE FROM relations WHERE issue_id_1 = ?1 AND issue_id_2 = ?2 AND relation_type = ?3",
            params![a, b, relation_type],
        )?;
        Ok(rows > 0)
    }

    /// Get all related issues (any relation type).
    #[cfg(test)]
    pub fn get_related_issues(&self, issue_id: impl ToString) -> Result<Vec<Issue>> {
        let issue_id = issue_id.to_string();
        let mut stmt = self.conn.prepare(
            r#"
            SELECT i.id, i.title, i.description, i.status, i.issue_type, i.priority, i.parent_id, i.created_at, i.updated_at, i.closed_at
            FROM issues i
            WHERE i.id IN (
                SELECT issue_id_2 FROM relations WHERE issue_id_1 = ?1
                UNION
                SELECT issue_id_1 FROM relations WHERE issue_id_2 = ?1
            )
            ORDER BY i.id
            "#,
        )?;

        let issues = stmt
            .query_map([issue_id], issue_from_row)?
            .collect::<std::result::Result<Vec<_>, _>>()?;

        Ok(issues)
    }

    /// Get all relations for an issue with their types.
    pub fn get_typed_relations(&self, issue_id: impl ToString) -> Result<Vec<Relation>> {
        let issue_id = issue_id.to_string();
        let mut stmt = self.conn.prepare(
            r#"
            SELECT issue_id_1, issue_id_2, relation_type, created_at
            FROM relations
            WHERE issue_id_1 = ?1 OR issue_id_2 = ?1
            ORDER BY created_at
            "#,
        )?;

        let relations = stmt
            .query_map([issue_id], |row| {
                Ok(Relation {
                    issue_id_1: row.get(0)?,
                    issue_id_2: row.get(1)?,
                    relation_type: row.get(2)?,
                    created_at: super::parse_datetime(row.get::<_, String>(3)?),
                })
            })?
            .collect::<std::result::Result<Vec<_>, _>>()?;

        Ok(relations)
    }

    /// Get issues related by a specific relation type.
    pub fn get_issues_by_relation_type(
        &self,
        issue_id: impl ToString,
        relation_type: &str,
    ) -> Result<Vec<Issue>> {
        let issue_id = issue_id.to_string();
        let mut stmt = self.conn.prepare(
            r#"
            SELECT i.id, i.title, i.description, i.status, i.issue_type, i.priority, i.parent_id, i.created_at, i.updated_at, i.closed_at
            FROM issues i
            WHERE i.id IN (
                SELECT issue_id_2 FROM relations WHERE issue_id_1 = ?1 AND relation_type = ?2
                UNION
                SELECT issue_id_1 FROM relations WHERE issue_id_2 = ?1 AND relation_type = ?2
            )
            ORDER BY i.id
            "#,
        )?;

        let issues = stmt
            .query_map(params![issue_id, relation_type], issue_from_row)?
            .collect::<std::result::Result<Vec<_>, _>>()?;

        Ok(issues)
    }

    /// Downstream impact: find issues that depend on a source through hierarchy
    /// or impact-bearing relations.
    ///
    /// Child, "derived", "caused-by", and "falsifies" relations are followed
    /// transitively. "assumption" links are included one hop from the source
    /// because shared assumptions should be reviewed without treating the whole
    /// cluster as invalidated.
    pub fn downstream_impact(&self, source_id: impl ToString) -> Result<Vec<Issue>> {
        let source_id = source_id.to_string();
        let mut visited = std::collections::HashSet::new();
        let mut queue = std::collections::VecDeque::new();
        let mut affected = Vec::new();

        queue.push_back(source_id.clone());
        visited.insert(source_id.clone());

        while let Some(current_id) = queue.pop_front() {
            // 1. Children (parent_id = current) are downstream work.
            let children = self.get_subissues(&current_id)?;
            for child in children {
                if visited.insert(child.id.clone()) {
                    affected.push(child.clone());
                    queue.push_back(child.id);
                }
            }

            // 2. Impact-bearing typed relations describe downstream dependency.
            for relation_type in TRANSITIVE_IMPACT_RELATIONS {
                let linked = self.get_issues_by_relation_type(&current_id, relation_type)?;
                for issue in linked {
                    if visited.insert(issue.id.clone()) {
                        affected.push(issue.clone());
                        queue.push_back(issue.id);
                    }
                }
            }

            // 3. "assumption" links are one hop only from the source.
            if current_id == source_id {
                let shared = self.get_issues_by_relation_type(&current_id, "assumption")?;
                for issue in shared {
                    if visited.insert(issue.id.clone()) {
                        affected.push(issue.clone());
                    }
                }
            }
        }

        Ok(affected)
    }
}
