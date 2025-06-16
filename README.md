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

### Option 1: Install from Crates.io (Recommended)

```bash
cargo install git-time-machine
git-time-machine
```

### Option 2: Install from Source

#### Prerequisites

- Rust (latest stable version)
- Git

#### Setup

```bash
# Clone the repository
git clone https://github.com/hrirkslab/git-time-machine.git
cd git-time-machine

# Build the project
cargo build --release

# Run the server
cargo run --release
```

### Option 3: Using Docker

```bash
# Build Docker image
docker build -t git-time-machine .

# Run container
docker run -p 3000:3000 -v $(pwd):/workspace git-time-machine
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

## GitHub Copilot Integration

Git Time Machine comes with built-in support for GitHub Copilot through the `.copilot` configuration directory. This allows Copilot to use the Git Time Machine as a tool provider for answering questions about your code's history.

### Setup

1. Make sure the Git Time Machine server is running:
   ```bash
   cargo run --release
   ```

2. The `.copilot/settings.json` file registers the server as a tool provider for GitHub Copilot:
   ```json
   {
     "tool_providers": [
       {
         "name": "git-time-machine",
         "url": "http://localhost:3000",
         "enabled": true
       }
     ]
   }
   ```

3. If you're using GitHub Copilot CLI or Copilot Workspace, it will automatically detect the tool provider configuration.

### Usage with Copilot

Once configured, you can ask Copilot questions about your code's history, such as:

- "Who last modified the authentication function?"
- "When was this bug introduced?"
- "How has this file evolved over the past month?"
- "What were the major changes in commit abc123?"
- "Show me what this file looked like before the refactoring"

Copilot will use the Git Time Machine tools to provide insightful answers based on your repository's history.

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
