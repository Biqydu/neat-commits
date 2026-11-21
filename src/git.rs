use std::process::Command;

use anyhow::{Context, Result, bail, ensure};

const GIT_MISSING: &str = "Failed to run git command. Is git in your PATH?";

/// Whether the index contains staged changes.
pub fn has_staged_changes() -> Result<bool> {
    let status = Command::new("git")
        .args(["diff", "--cached", "--quiet"])
        .status()
        .context(GIT_MISSING)?;

    // `--quiet` implies `--exit-code`: 0 = no changes, 1 = changes, other = error.
    match status.code() {
        Some(0) => Ok(false),
        Some(1) => Ok(true),
        _ => bail!("`git diff --cached` failed. Are you inside a git repository?"),
    }
}

pub fn commit(message: &str) -> Result<()> {
    let status = Command::new("git")
        .args(["commit", "-m", message])
        .status()
        .context(GIT_MISSING)?;

    ensure!(
        status.success(),
        "git commit failed. Make sure you staged your files (git add) before running neat."
    );
    Ok(())
}
