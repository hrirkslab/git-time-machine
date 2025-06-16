# Git Time Machine - Copilot Integration

This directory contains configuration files for integrating the Git Time Machine MCP server with GitHub Copilot.

## Configuration Files

- **settings.json**: Registers the Git Time Machine MCP server as a tool provider for GitHub Copilot CLI and Workspace

## How It Works

1. The `settings.json` file tells Copilot to connect to the Git Time Machine MCP server running at http://localhost:3000
2. Copilot will automatically discover the tools from the server's `/metadata` and `/tools` endpoints
3. When you ask Copilot questions about Git history, it can use these tools to provide answers

## Available Tools

- `get_git_blame`: Returns line-by-line blame metadata for a file
- `get_commit_diff`: Shows full diff with metadata for a specific commit
- `summarize_diff`: Compares two commits and provides a summary of changes
- `get_commits_affecting`: Lists commits that modified a specific file
- `get_file_at_commit`: Retrieves file contents as they existed at a specific commit

## Usage

Ensure the Git Time Machine server is running at http://localhost:3000 before using Copilot to access Git history information.

Example questions you can ask Copilot:
- "Who last modified function X in file Y?"
- "What changed in commit abc123?"
- "How did this file evolve over the last month?"
- "What commits affected this file?"
- "Show me what this file looked like before the recent refactoring"

For more information on using GitHub Copilot with custom MCP servers, refer to the GitHub Copilot documentation.
