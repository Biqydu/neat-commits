use anyhow::{Context, Result};
use clap::Parser;
use colored::Colorize;
use dialoguer::{Confirm, Input, Select, theme::ColorfulTheme};
use std::process::Command;

#[derive(Parser, Debug)]
#[command(
    name = "neat",
    version,
    about = "A clean and user-friendly Conventional Commits assistant"
)]
struct Args {
    #[arg(short, long)]
    r#type: Option<String>,

    #[arg(short, long)]
    scope: Option<String>,

    #[arg(short, long)]
    message: Option<String>,

    #[arg(short, long)]
    breaking: bool,

    #[arg(long)]
    breaking_desc: Option<String>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let is_interactive = args.r#type.is_none() || args.message.is_none();

    let has_staged_files = Command::new("git")
        .args(["diff", "--cached", "--quiet"])
        .status()
        .map(|s| !s.success())
        .unwrap_or(true);

    if is_interactive {
        println!(
            "\n{}",
            "✨ NEAT — Conventional Commits Helper ✨\n"
                .bold()
                .magenta()
        );
    }

    if !has_staged_files {
        if is_interactive {
            println!(
                "{}",
                "⚠️  Warning: No files staged for commit. Did you forget to run `git add`?"
                    .yellow()
            );
            if !Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt("Do you want to proceed anyway?")
                .default(false)
                .interact()?
            {
                println!("{}", "Aborted.".bright_black());
                return Ok(());
            }
        } else {
            eprintln!(
                "{}",
                "❌ Error: No files staged for commit. Run `git add` first."
                    .red()
                    .bold()
            );
            return Ok(());
        }
    }

    let types = [
        "feat:     New feature",
        "fix:      Bug fix",
        "docs:     Documentation changes",
        "style:    Formatting, punctuation (no code changes)",
        "refactor: Code refactoring (neither bug fix nor new feature)",
        "perf:     Code change that improves performance",
        "test:     Adding or updating tests",
        "build:    Changes affecting build system or external dependencies",
        "ci:       Changes to CI configuration files and scripts",
        "chore:    Other changes that don't modify src or test files",
        "revert:   Reverts a previous commit",
    ];

    let commit_type = match args.r#type {
        Some(t) => t,
        None => {
            let selection = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Select the type of change")
                .default(0)
                .items(types)
                .interact()?;
            types[selection]
                .split(':')
                .next()
                .unwrap()
                .trim()
                .to_string()
        }
    };

    if !types
        .iter()
        .any(|t| t.starts_with(&format!("{}:", commit_type)))
    {
        println!(
            "{}",
            "⚠️  Warning: You are using a non-standard commit type!".yellow()
        );
    }

    let scope = match args.scope {
        Some(s) => s,
        None => Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Scope (optional, press Enter to skip)")
            .allow_empty(true)
            .interact_text()?,
    };

    let description = match args.message {
        Some(m) => m,
        None => Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Enter a short commit message")
            .validate_with(|input: &String| {
                if input.trim().is_empty() {
                    Err("Commit message cannot be empty!")
                } else {
                    Ok(())
                }
            })
            .interact_text()?,
    };

    let is_breaking_change = if args.breaking {
        true
    } else if is_interactive {
        Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Does this commit introduce a breaking change?")
            .default(false)
            .interact()?
    } else {
        false
    };

    let bang = if is_breaking_change { "!" } else { "" };
    let mut final_message = if scope.trim().is_empty() {
        format!("{}{}: {}", commit_type, bang, description)
    } else {
        format!("{}({}){}: {}", commit_type, scope.trim(), bang, description)
    };

    if final_message.lines().next().unwrap().len() > 72 {
        println!(
            "{}",
            "⚠️  Warning: First line of commit message exceeds 72 characters!".yellow()
        );
    }

    if is_breaking_change {
        let breaking_description = match args.breaking_desc {
            Some(desc) => desc,
            None => {
                if is_interactive {
                    Input::with_theme(&ColorfulTheme::default())
                        .with_prompt("Describe the breaking change (required)")
                        .validate_with(|input: &String| {
                            if input.trim().is_empty() {
                                Err("Breaking change description cannot be empty!")
                            } else {
                                Ok(())
                            }
                        })
                        .interact_text()?
                } else {
                    "Unspecified breaking change".to_string()
                }
            }
        };
        final_message = format!(
            "{}\n\nBREAKING CHANGE: {}",
            final_message,
            breaking_description.trim()
        );
    }

    if is_interactive {
        println!(
            "\n{} {}",
            "📝 Prepared commit message:".bright_black(),
            final_message.green().bold()
        );

        if !Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Does this look correct?")
            .default(true)
            .interact()?
        {
            println!("{}", "Aborted. Commit was not created.".yellow());
            return Ok(());
        }
    }

    let status = Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg(&final_message)
        .status()
        .context("Failed to run git command. Is git in your PATH?")?;

    if status.success() {
        println!("{}", "✅ Success! Code has been committed.".green().bold());
    } else {
        eprintln!("\n{}", "❌ Git returned an error!".red().bold());
        eprintln!(
            "{}",
            "Make sure you staged your files (git add) before running neat.".bright_black()
        );
    }

    Ok(())
}
