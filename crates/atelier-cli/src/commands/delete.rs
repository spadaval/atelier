use anyhow::{bail, Result};
use std::io::{self, Write};

use std::path::Path;

use crate::utils::format_issue_id;
use atelier_sqlite::Database;
pub fn run_lifecycle(state_dir: &Path, db_path: &Path, id: &str, force: bool) -> Result<()> {
    let db = Database::open(db_path)?;
    let issue = match db.get_issue(id)? {
        Some(i) => i,
        None => bail!("Issue {} not found", format_issue_id(id)),
    };
    drop(db);

    if !force {
        print!(
            "Delete issue {} \"{}\"? [y/N] ",
            format_issue_id(id),
            issue.title
        );
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        if !input.trim().eq_ignore_ascii_case("y") {
            println!("Cancelled.");
            return Ok(());
        }
    }

    let deleted = crate::commands::issue::delete_lifecycle(state_dir, db_path, id)?;
    println!("Deleted issue {}", format_issue_id(&deleted));
    Ok(())
}
