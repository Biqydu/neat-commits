pub struct CommitType {
    pub name: &'static str,
    pub description: &'static str,
}

impl CommitType {
    /// Text shown in the selection menu, e.g. `feat:     New feature`.
    pub fn label(&self) -> String {
        format!("{:<10} {}", format!("{}:", self.name), self.description)
    }
}

pub const COMMIT_TYPES: &[CommitType] = &[
    CommitType {
        name: "feat",
        description: "New feature",
    },
    CommitType {
        name: "fix",
        description: "Bug fix",
    },
    CommitType {
        name: "docs",
        description: "Documentation changes",
    },
    CommitType {
        name: "style",
        description: "Formatting, punctuation (no code changes)",
    },
    CommitType {
        name: "refactor",
        description: "Code refactoring (neither bug fix nor new feature)",
    },
    CommitType {
        name: "perf",
        description: "Code change that improves performance",
    },
    CommitType {
        name: "test",
        description: "Adding or updating tests",
    },
    CommitType {
        name: "build",
        description: "Changes affecting build system or external dependencies",
    },
    CommitType {
        name: "ci",
        description: "Changes to CI configuration files and scripts",
    },
    CommitType {
        name: "chore",
        description: "Other changes that don't modify src or test files",
    },
    CommitType {
        name: "revert",
        description: "Reverts a previous commit",
    },
];

pub fn is_standard(name: &str) -> bool {
    COMMIT_TYPES.iter().any(|t| t.name == name)
}
