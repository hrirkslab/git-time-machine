## 🕰️ Git Time Machine — Unlock Your Code’s Past with AI

**Git Time Machine** is an **MCP (Model Context Protocol) server** that empowers GitHub Copilot and LLMs to reason about the *evolution of your codebase*. By exposing rich Git history through structured tools and context, it transforms AI into a powerful assistant for code archaeology, change tracking, and debugging.

> *“What did this function look like two months ago?”*
> *“Why was this file refactored last week?”*
> *“Which commits introduced this bug?”*
> Copilot can now answer.

---

### 🚀 Features

✅ **Blame-aware Code Analysis**

* Expose `git blame` for any file
* Let Copilot find when a line was introduced and by whom

✅ **Commit Timeline Exploration**

* Query commits that touched a file
* Summarize commit diffs using LLMs
* Compare two commits and generate human-friendly summaries

✅ **Diff Context Provider**

* Expose contextual diffs for the current working branch
* Let AI explain complex refactors or regressions

✅ **Interactive Git Metadata**

* Author info, timestamps, commit messages
* Structured access to all Git metadata

✅ **Powered by Rust + MCP**

* Built in **Rust** using `axum`, `tokio`, and `serde`
* Conforms to **OpenAI’s Model Context Protocol**, ready for Copilot Workspace and ChatGPT plugins

---

### 🛠️ Example Tools

| Tool Name                                       | Description                            |
| ----------------------------------------------- | -------------------------------------- |
| `get_git_blame(file: string)`                   | Returns line-by-line blame metadata    |
| `get_commit_diff(sha: string)`                  | Shows full diff with metadata          |
| `summarize_diff(base: string, head: string)`    | Uses LLM to describe changes           |
| `get_commits_affecting(file: string)`           | Lists commits that touched a file      |
| `get_file_at_commit(file: string, sha: string)` | Returns file contents at a past commit |

---

### 🧠 Use Cases

* **Debugging**: Ask “When was this bug introduced?”
* **Code Review**: Auto-generate human-readable summaries of commits
* **Learning**: See how a file evolved over time and why
* **Audit**: Trace authorship and reasoning behind changes

---

### 📦 Integration

This server is compatible with:

* 🧠 **GitHub Copilot Workspace**
* 💬 **ChatGPT via Plugins**
* 🖥️ **VS Code Extensions**
* 🧪 Any LLMs supporting tool-calling or function schemas

---

### 📍 Status

✅ MVP ready
🚧 Live context provider support in progress
🔜 Full support for GitHub Copilot CLI

