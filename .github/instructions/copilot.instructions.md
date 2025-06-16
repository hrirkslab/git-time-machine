# Git Time Machine – Copilot Project Instruction

## Goal
Build an **MCP (Model Context Protocol) server** in **Rust** that allows GitHub Copilot to explore Git history, diffs, and file evolution. The server should expose tools and context providers that allow an LLM to answer questions like:

- “Who last modified this function?”
- “What changed between these two commits?”
- “How did this file evolve over time?”

## Key Features

### 🧰 Tools

Implement the following HTTP endpoints under `/tools` to expose these tools:

1. **get_git_blame(file: string)**  
   → Return structured blame metadata for each line (commit hash, author, date).

2. **get_commit_diff(sha: string)**  
   → Return the full diff and metadata for a specific commit.

3. **summarize_diff(base: string, head: string)**  
   → Compare two commits and summarize the diff in plain text.

4. **get_commits_affecting(file: string)**  
   → Return a list of commits that modified the given file.

5. **get_file_at_commit(file: string, sha: string)**  
   → Return the file contents as it existed at the given commit.

### 📚 Metadata

- Provide `/metadata` endpoint that returns MCP-compatible metadata.
- Follow MCP spec: https://github.com/openai/model-context-protocol

## Technical Constraints

- Use **Rust** with the `axum` web framework.
- Use `tokio` as the async runtime.
- Use `serde` and `serde_json` for serialization.
- Use `git2` crate to interact with Git repositories.
- Return tool definitions in **OpenAPI schema** format.
- Optional: Use `utoipa` to generate OpenAPI tool specs automatically.

## Integration

- Make the server MCP-compliant.
- Serve a `.well-known/ai-plugin.json` if needed.
- Expose all tools through `/tools` endpoint in OpenAPI schema.
- Add basic context provider support later (optional).

## Directory Structure (Recommended)

```txt
src/
  main.rs
  handlers/
    mod.rs
    blame.rs
    diff.rs
    commits.rs
  models/
    tool_schema.rs
  utils/
    git_helpers.rs

.copilot/
  instructions.md
  context.json (optional)
