use std::fmt;

/// A Conventional Commits message.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommitMessage {
    kind: String,
    scope: Option<String>,
    description: String,
    breaking: Option<String>,
}

impl CommitMessage {
    pub fn new(
        kind: String,
        scope: Option<String>,
        description: String,
        breaking: Option<String>,
    ) -> Self {
        Self {
            kind,
            scope: scope.map(|s| s.trim().to_owned()).filter(|s| !s.is_empty()),
            description,
            breaking: breaking.map(|b| b.trim().to_owned()),
        }
    }

    /// The first line: `type(scope)!: description`.
    pub fn header(&self) -> String {
        let scope = self
            .scope
            .as_deref()
            .map(|s| format!("({s})"))
            .unwrap_or_default();
        let bang = if self.breaking.is_some() { "!" } else { "" };
        format!("{}{scope}{bang}: {}", self.kind, self.description)
    }
}

impl fmt::Display for CommitMessage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.header())?;
        if let Some(breaking) = &self.breaking {
            write!(f, "\n\nBREAKING CHANGE: {breaking}")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn msg(scope: Option<&str>, breaking: Option<&str>) -> String {
        CommitMessage::new(
            "feat".into(),
            scope.map(Into::into),
            "add thing".into(),
            breaking.map(Into::into),
        )
        .to_string()
    }

    #[test]
    fn simple() {
        assert_eq!(msg(None, None), "feat: add thing");
    }

    #[test]
    fn blank_scope_is_ignored() {
        assert_eq!(msg(Some("  "), None), "feat: add thing");
    }

    #[test]
    fn with_scope() {
        assert_eq!(msg(Some(" api "), None), "feat(api): add thing");
    }

    #[test]
    fn breaking() {
        assert_eq!(
            msg(Some("api"), Some("removed v1")),
            "feat(api)!: add thing\n\nBREAKING CHANGE: removed v1"
        );
    }
}
