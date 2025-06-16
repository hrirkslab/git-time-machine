# 🕰️ Git Time Machine

A Rust-based MCP (Model Context Protocol) server that provides git history insights to AI assistants like GitHub Copilot. This server exposes tools and context providers that allow LLMs to reason about the evolution of your codebase.

## Features

- **Blame-aware Code Analysis**: Expose `git blame` for any file
- **Commit Timeline Exploration**: Query commits that touched a file
- **Diff Context Provider**: Expose contextual diffs for any commit
- **Interactive Git Metadata**: Access author info, timestamps, commit messages
- **Powered by Rust + MCP**: Built using `axum`, `tokio`, and `serde`

## API Endpoints

| Endpoint | Description |
|---|---|
| `POST /tools/get_git_blame` | Returns line-by-line blame metadata |
| `POST /tools/get_commit_diff` | Shows full diff with metadata |
| `POST /tools/summarize_diff` | Describes changes between commits |
| `POST /tools/get_commits_affecting` | Lists commits that modified a file |
| `POST /tools/get_file_at_commit` | Returns file contents at a past commit |
| `GET /metadata` | Returns MCP metadata |
| `GET /.well-known/ai-plugin.json` | Returns plugin manifest for AI integration |
| `GET /openapi.json` | Returns OpenAPI schema |

## Installation

### Prerequisites

- Rust (latest stable version)
- Git

### Setup

```bash
# Clone the repository
git clone https://github.com/yourusername/git-time-machine.git
cd git-time-machine

# Build the project
cargo build --release

# Run the server
cargo run --release
```

The server will start on `http://localhost:3000`.

## Usage

### With GitHub Copilot

Git Time Machine is compatible with GitHub Copilot through the MCP protocol. When using Copilot, you can:

1. Ask questions about code history
2. Get insights into when bugs were introduced
3. Understand the evolution of code over time

Example prompts:
- "Who last modified this function?"
- "What changed in this file last week?"
- "Show me the history of this file."

### Direct API Access

You can also directly access the API endpoints:

```bash
# Example: Get blame for a file
curl -X POST -H "Content-Type: application/json" \
  -d '{"file": "src/main.rs"}' \
  http://localhost:3000/tools/get_git_blame
```

## Development

### Project Structure

```
src/
  main.rs              # Server setup and routing
  handlers/            # HTTP endpoint handlers
    mod.rs
    blame.rs           # Git blame handlers
    commits.rs         # Commit history handlers
    diff.rs            # Diff generation handlers
    metadata.rs        # MCP metadata handlers
  models/              # Data models
    mod.rs
    tool_schema.rs     # API request/response schemas
    git.rs             # Git-related models
  utils/               # Utility functions
    mod.rs
    git_helpers.rs     # Git interaction utilities
```

### Contributing

1. Fork the repository
2. Create a feature branch: `git checkout -b feature-name`
3. Commit your changes: `git commit -am 'Add some feature'`
4. Push to the branch: `git push origin feature-name`
5. Submit a pull request

## License

MIT
