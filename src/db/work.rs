use anyhow::{bail, Result};
use chrono::Utc;
use rusqlite::params;

use super::{parse_datetime, Database};
use crate::models::WorkAssociation;

impl Database {
    pub fn start_work_association(
        &self,
        issue_id: &str,
        branch: Option<&str>,
        worktree_path: Option<&str>,
    ) -> Result<()> {
        self.require_issue(issue_id)?;
        if let Some(active) = self.active_work_association_for_worktree_path(worktree_path)? {
            if active.issue_id != issue_id {
                bail!(
                    "Worktree already has associated issue {}. Use `atelier worktree status`, `atelier worktree remove {}`, or `atelier worktree repair {}` before associating {}.",
                    active.issue_id,
                    active.issue_id,
                    active.issue_id,
                    issue_id
                );
            }
        }
        let now = Utc::now().to_rfc3339();
        self.conn.execute(
            "INSERT INTO work_associations (issue_id, status, branch, worktree_path, started_at)
             VALUES (?1, 'active', ?2, ?3, ?4)
             ON CONFLICT(issue_id) DO UPDATE SET
                status = 'active',
                branch = excluded.branch,
                worktree_path = excluded.worktree_path,
                started_at = excluded.started_at,
                finished_at = NULL",
            params![issue_id, branch, worktree_path, now],
        )?;
        Ok(())
    }

    pub fn finish_work_association(&self, issue_id: &str) -> Result<bool> {
        self.complete_work_association(issue_id, "finished")
    }

    fn complete_work_association(&self, issue_id: &str, status: &str) -> Result<bool> {
        let now = Utc::now().to_rfc3339();
        let rows = self.conn.execute(
            "UPDATE work_associations
             SET status = ?1, finished_at = ?2
             WHERE issue_id = ?3 AND status = 'active'",
            params![status, now, issue_id],
        )?;
        Ok(rows > 0)
    }

    pub fn active_work_association_for_worktree_path(
        &self,
        worktree_path: Option<&str>,
    ) -> Result<Option<WorkAssociation>> {
        let mut stmt = self.conn.prepare(
            "SELECT issue_id, status, branch, worktree_path, started_at, finished_at
             FROM work_associations
             WHERE status = 'active'
               AND (
                    (?1 IS NULL AND worktree_path IS NULL)
                    OR worktree_path = ?1
               )
             ORDER BY started_at DESC LIMIT 1",
        )?;
        Ok(stmt.query_row(params![worktree_path], work_from_row).ok())
    }

    pub fn get_work_association(&self, issue_id: &str) -> Result<Option<WorkAssociation>> {
        let mut stmt = self.conn.prepare(
            "SELECT issue_id, status, branch, worktree_path, started_at, finished_at
             FROM work_associations WHERE issue_id = ?1",
        )?;
        Ok(stmt.query_row(params![issue_id], work_from_row).ok())
    }

    pub fn list_work_associations(&self) -> Result<Vec<WorkAssociation>> {
        let mut stmt = self.conn.prepare(
            "SELECT issue_id, status, branch, worktree_path, started_at, finished_at
             FROM work_associations ORDER BY started_at DESC",
        )?;
        let rows = stmt.query_map([], work_from_row)?;
        rows.collect::<std::result::Result<Vec<_>, _>>()
            .map_err(Into::into)
    }

    pub fn remove_work_association(&self, issue_id: &str) -> Result<bool> {
        let rows = self.conn.execute(
            "DELETE FROM work_associations WHERE issue_id = ?1",
            params![issue_id],
        )?;
        Ok(rows > 0)
    }
}

fn work_from_row(row: &rusqlite::Row) -> rusqlite::Result<WorkAssociation> {
    Ok(WorkAssociation {
        issue_id: row.get(0)?,
        status: row.get(1)?,
        branch: row.get(2)?,
        worktree_path: row.get(3)?,
        started_at: parse_datetime(row.get::<_, String>(4)?),
        finished_at: row.get::<_, Option<String>>(5)?.map(parse_datetime),
    })
}
