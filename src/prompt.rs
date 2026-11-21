use crate::commit_type::COMMIT_TYPES;
use anyhow::Result;
use dialoguer::{Confirm, Input, Select, theme::ColorfulTheme};

/// Thin wrapper around `dialoguer` sharing a single theme.
pub struct Prompter {
    theme: ColorfulTheme,
}

impl Default for Prompter {
    fn default() -> Self {
        Self::new()
    }
}

impl Prompter {
    pub fn new() -> Self {
        Self {
            theme: ColorfulTheme::default(),
        }
    }

    pub fn confirm(&self, prompt: &str, default: bool) -> Result<bool> {
        Ok(Confirm::with_theme(&self.theme)
            .with_prompt(prompt)
            .default(default)
            .interact()?)
    }

    pub fn commit_type(&self) -> Result<String> {
        let items: Vec<String> = COMMIT_TYPES.iter().map(|t| t.label()).collect();

        let selection = Select::with_theme(&self.theme)
            .with_prompt("Select the type of change")
            .default(0)
            .items(&items)
            .interact()?;

        Ok(COMMIT_TYPES[selection].name.to_owned())
    }

    pub fn scope(&self) -> Result<Option<String>> {
        let scope: String = Input::with_theme(&self.theme)
            .with_prompt("Scope (optional, press Enter to skip)")
            .allow_empty(true)
            .interact_text()?;

        Ok(Some(scope).filter(|s| !s.trim().is_empty()))
    }

    pub fn description(&self) -> Result<String> {
        self.required_text(
            "Enter a short commit message",
            "Commit message cannot be empty!",
        )
    }

    pub fn breaking_description(&self) -> Result<String> {
        self.required_text(
            "Describe the breaking change (required)",
            "Breaking change description cannot be empty!",
        )
    }

    fn required_text(&self, prompt: &str, error: &'static str) -> Result<String> {
        Ok(Input::with_theme(&self.theme)
            .with_prompt(prompt)
            .validate_with(move |input: &String| {
                if input.trim().is_empty() {
                    Err(error)
                } else {
                    Ok(())
                }
            })
            .interact_text()?)
    }
}
