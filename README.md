# ✨ neat-commits (neat)

A lightweight, blazing fast, and user-friendly CLI helper for writing **Conventional Commits**.

Built with Rust, it weighs **under 700 KB**, boots up instantly, and works both as an interactive wizard and a quick scriptable CLI tool.

---

## Features

- **Interactive Wizard:** Don't remember all the conventional types? Just use your arrow keys!
- **Hybrid CLI Mode:** Pass arguments via flags to skip the wizard (perfect for power users and automation).
- **Breaking Change Support:** Automatically appends the `!` bang and injects `BREAKING CHANGE: ...` into the commit footer.
- **Header Length Warning:** Warns you if your commit title exceeds the standard 72-character limit.
- **Safety First:** Prompts for confirmation before executing the actual `git commit`.

---

## 📦 Installation

Since the project is open-source, you can install it directly from GitHub using Cargo. Make sure you have [Rust installed](https://www.rust-lang.org/tools/install).

```bash
cargo install --git https://github.com/Biqydu/neat-commits.git

```

_Note: Even though the package is named `neat-commits`, it installs a binary called **`neat`** for quick and easy typing._

---

## 💡 Usage

### 1. Interactive Mode (The Wizard)

Simply type `neat` in any Git repository. The wizard will guide you through the process:

```bash
neat

```

**How it looks:**

1. Select the change type (e.g., `feat`, `fix`, `docs`, `chore`...) using arrow keys.
2. Enter an optional scope (e.g., `api`, `ui`).
3. Write a short description.
4. Specify if it's a breaking change.
5. Confirm the generated message, and `neat` will talk to Git for you!

### 2. Fast Mode (For Power Users)

If you already know what to write, you can bypass the questions entirely by using flags:

```bash
neat -t feat -s api -m "add user authentication"

```

For **Breaking Changes** via CLI:

```bash
neat -t fix -s core -m "remove deprecated endpoints" -b --breaking-desc "Drop support for v1 API"

```

---

## 🛠️ CLI Arguments & Flags

You can always check the built-in help by running `neat --help`.

| Flag / Option     | Short | Description                                       |
| ----------------- | ----- | ------------------------------------------------- |
| `--type`          | `-t`  | Commit type (e.g., `feat`, `fix`, `refactor`)     |
| `--scope`         | `-s`  | Optional scope of the change (e.g., `db`, `deps`) |
| `--message`       | `-m`  | The commit description/message                    |
| `--breaking`      | `-b`  | Marks the commit as a breaking change             |
| `--breaking-desc` |       | Description for the breaking change footer        |
| `--help`          | `-h`  | Prints help information                           |
| `--version`       | `-V`  | Prints version information                        |

---

## 📄 License

This project is dual-licensed under either:

- **MIT License** ([LICENSE-MIT](https://www.google.com/search?q=LICENSE-MIT))
- **Apache License, Version 2.0** ([LICENSE-APACHE](https://www.google.com/search?q=LICENSE-APACHE))

at your option.

---

## 👤 Author

Created with 🦀. Feel free to open issues or submit pull requests!
