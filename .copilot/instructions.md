# Git Time Machine Instructions for GitHub Copilot

## About Git Time Machine

Git Time Machine is an MCP server that allows you (GitHub Copilot) to explore Git history, diffs, and file evolution in the user's codebase. You can use this tool to answer questions about code history, track changes over time, and provide insights about the evolution of the codebase.

## How to Use These Tools

When a user asks about Git history or the evolution of their code, you can use the Git Time Machine tools to provide answers. Here are the available tools and how to use them:

### `get_git_blame`

Use this tool to determine who last modified each line in a file and when.

Sample prompts:
- "Who wrote this function?"
- "When was this code introduced?"
- "Who last modified this file?"

### `get_commit_diff`

Use this tool to view the changes made in a specific commit.

Sample prompts:
- "What changed in commit abc123?"
- "Show me what was modified in the latest commit"
- "What files were affected by this commit?"

### `summarize_diff`

Use this tool to generate a human-readable summary of changes between two commits.

Sample prompts:
- "What's the difference between commit A and commit B?"
- "Summarize the changes from last week to now"
- "What changed between these two versions?"

### `get_commits_affecting`

Use this tool to retrieve a list of commits that modified a given file.

Sample prompts:
- "Which commits touched this file?"
- "When was this file last modified?"
- "Show me the history of changes to this file"

### `get_file_at_commit`

Use this tool to view file contents as they existed at a specific point in time.

Sample prompts:
- "Show me what this file looked like last month"
- "What did this code look like before the refactoring?"
- "How was this implemented in version 1.0?"

## Response Guidelines

When using the Git Time Machine tools:

1. Provide concise, insightful responses that focus on what the user is asking about
2. Highlight important changes, patterns, or developments in the code's evolution
3. Use the historical information to explain the context and reasoning behind code changes
4. When appropriate, trace the introduction of bugs or the evolution of features

Remember, your goal is to help users understand their codebase's history to make better decisions about its future.
