# ✨ neat-commits (neat)

A lightweight, blazing fast, and user-friendly CLI helper for writing **Conventional Commits**.

Built with Rust, it weighs **under 700 KB**, boots up instantly, and works both as an interactive wizard and a quick scriptable CLI tool.

---

## 🚀 Features

- **Interactive Wizard:** Don't remember all the conventional types? Just use your arrow keys!
- **Hybrid CLI Mode:** Pass arguments via flags to skip the wizard (perfect for power users and automation).
- **Breaking Change Support:** Automatically appends the `!` bang and injects `BREAKING CHANGE: ...` into the commit footer.
- **Header Length Warning:** Warns you if your commit title exceeds the standard 72-character limit.
- **Safety First:** Prompts for confirmation before executing the actual `git commit`.

---

## 📦 Installation

Since the project is open-source, you can install it directly from GitHub using Cargo. Make sure you have [Rust installed](https://www.rust-lang.org/tools/install).

```bash
cargo install --git [https://github.com/biqydu/neat-commits.git](https://github.com/biqydu/neat-commits.git)
```
