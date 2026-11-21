mod cli;
mod commit_type;
mod git;
mod message;
mod prompt;

use anyhow::{Result, bail};
use clap::Parser;
use cli::Cli;
use colored::Colorize;
use message::CommitMessage;
use prompt::Prompter;

const MAX_HEADER_LEN: usize = 72;
const DEFAULT_BREAKING_DESC: &str = "Unspecified breaking change";

fn main() -> Result<()> {
    let cli = Cli::parse();
    let interactive = cli.is_interactive();
    let prompter = Prompter::new();

    if interactive {
        println!(
            "\n{}\n",
            "✨ NEAT — Conventional Commits Helper ✨".bold().magenta()
        );
    }

    if !confirm_staged_files(interactive, &prompter)? {
        return Ok(());
    }

    let message = collect_message(cli, interactive, &prompter)?;

    if message.header().chars().count() > MAX_HEADER_LEN {
        println!(
            "{}",
            format!(
                "⚠️  Warning: First line of commit message exceeds {MAX_HEADER_LEN} characters!"
            )
            .yellow()
        );
    }

    if interactive && !confirm_commit(&message, &prompter)? {
        println!("{}", "Aborted. Commit was not created.".yellow());
        return Ok(());
    }

    git::commit(&message.to_string())?;
    println!("{}", "✅ Success! Code has been committed.".green().bold());

    Ok(())
}

/// Returns `Ok(false)` when the user decided to abort.
fn confirm_staged_files(interactive: bool, prompter: &Prompter) -> Result<bool> {
    if git::has_staged_changes()? {
        return Ok(true);
    }

    if !interactive {
        bail!("No files staged for commit. Run `git add` first.");
    }

    println!(
        "{}",
        "⚠️  Warning: No files staged for commit. Did you forget to run `git add`?".yellow()
    );
    let proceed = prompter.confirm("Do you want to proceed anyway?", false)?;
    if !proceed {
        println!("{}", "Aborted.".bright_black());
    }
    Ok(proceed)
}

fn collect_message(cli: Cli, interactive: bool, prompter: &Prompter) -> Result<CommitMessage> {
    let Cli {
        commit_type,
        scope,
        message,
        breaking,
        breaking_desc,
    } = cli;

    let kind = match commit_type {
        Some(kind) => {
            if !commit_type::is_standard(&kind) {
                println!(
                    "{}",
                    "⚠️  Warning: You are using a non-standard commit type!".yellow()
                );
            }
            kind
        }
        None => prompter.commit_type()?,
    };

    let scope = match scope {
        Some(scope) => Some(scope),
        None if interactive => prompter.scope()?,
        None => None,
    };

    let description = match message {
        Some(message) => message,
        None => prompter.description()?,
    };

    let is_breaking = breaking
        || breaking_desc.is_some()
        || (interactive
            && prompter.confirm("Does this commit introduce a breaking change?", false)?);

    let breaking = if is_breaking {
        Some(match breaking_desc {
            Some(desc) => desc,
            None if interactive => prompter.breaking_description()?,
            None => DEFAULT_BREAKING_DESC.to_owned(),
        })
    } else {
        None
    };

    Ok(CommitMessage::new(kind, scope, description, breaking))
}

fn confirm_commit(message: &CommitMessage, prompter: &Prompter) -> Result<bool> {
    println!(
        "\n{} {}",
        "📝 Prepared commit message:".bright_black(),
        message.to_string().green().bold()
    );
    prompter.confirm("Does this look correct?", true)
}
