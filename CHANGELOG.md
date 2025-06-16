# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0] - 2025-06-16

### Added
- Initial release of Git Time Machine MCP server
- Git blame analysis with line-by-line metadata
- Commit diff viewing with full metadata
- File history exploration showing commits that affected a file
- Diff summarization between two commits
- File content retrieval at specific commits
- MCP (Model Context Protocol) compliance
- GitHub Copilot integration support
- OpenAPI documentation
- RESTful API endpoints
- Docker support
- Comprehensive documentation and examples

### Features
- `/tools/get_git_blame` - Returns line-by-line blame metadata
- `/tools/get_commit_diff` - Shows full diff with metadata
- `/tools/summarize_diff` - Describes changes between commits
- `/tools/get_commits_affecting` - Lists commits that modified a file
- `/tools/get_file_at_commit` - Returns file contents at a past commit
- `/metadata` - Returns MCP metadata
- `/.well-known/ai-plugin.json` - Returns plugin manifest for AI integration
- `/openapi.json` - Returns OpenAPI schema

### Technical
- Built with Rust using axum web framework
- Uses git2 library for Git operations
- Structured logging with tracing
- CORS support for web integration
- Error handling with thiserror
- Async/await support with tokio
