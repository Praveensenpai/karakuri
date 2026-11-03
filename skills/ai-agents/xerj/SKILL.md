---
name: xerj
description: >-
  Local-first AI search engine, AST-aware code indexer, and Model Context Protocol (MCP) server.
  Use to auto-index codebases, look up symbol definitions with exact file and line coordinates (xerj def),
  run BM25/vector/hybrid code search (xerj search), manage durable agent memory, and configure xerj MCP.
---

# `xerj` Local-First AI Search & MCP Skill

Integrates AI coding assistants with [`xerj`](https://github.com/xerj-org/xerj), a local-first search engine, AST-aware code indexer, and Model Context Protocol (MCP) server written in Rust.

---

## 1. When to Use

- **Token-Efficient Symbol Lookups**: Finding function, struct, class, or type definitions across large codebases without reading entire source files (`xerj def "<symbol>"`).
- **Targeted Code & Documentation Search**: Executing BM25, vector, or hybrid queries that return ranked passages with exact `file:line` coordinates (`xerj search "<query>"`).
- **Codebase Auto-Indexing**: Ingesting and structuring codebases using Tree-sitter AST parsing without manual schema configuration (`xerj autoindex <folder>`).
- **Persistent Agent Memory (RAG)**: Storing and recalling key architectural decisions, session notes, and design patterns across conversational turns using isolated memory namespaces (`xerj_memory_store`, `xerj_memory_recall`).
- **MCP Client Integration**: Bridging local search, symbol resolution, and agent memory into Antigravity, OpenCode, Claude Code, Cursor, and Codex CLI over `stdio` (`xerj mcp`).

---

## 2. Installation & Quickstart

### Automated Binary Install (Recommended)
Downloads the prebuilt static binary for the detected OS/architecture and installs to `~/.local/bin`:

```bash
# Linux / macOS
curl -fsSL https://xerj.org/get | sh
```

Verify installation:
```bash
xerj --version
```

### Install From Source via Cargo
```bash
cargo install --git https://github.com/xerj-org/xerj.git
```

---

## 3. Core CLI Workflows

### A. Indexing a Project
Run `autoindex` to sniff and index code, markdown docs, logs, JSONL, and PDFs with AST-aware line tracking:

```bash
# Index current workspace
xerj autoindex .

# Inspect the resulting index mappings and schema
xerj autoindex map

# Dry-run extraction check
xerj autoindex . --dry-run
```

### B. AST Symbol Definition (`xerj def`)
Quickly locate where a symbol is defined, its signature, and exact file and line:

```bash
xerj def "RustAnalyzer"
xerj def "AuditReport"
xerj def "handle_install"
```

### C. Hybrid & Semantic Search (`xerj search`)
Execute queries across indexed files with ranked results:

```bash
# Full-text / hybrid search
xerj search "clippy warning audit"

# Search within specific indexed folders or data types
xerj search "token savings" --limit 5
```

### D. Zero-Setup Q&A (`xerj ask`)
Query codebases or external resources with cited answers:

```bash
xerj ask . "How does skill selection work for different stacks?"
```

### E. Measure Token Efficiency Gains
```bash
xerj gain
```

---

## 4. MCP Server Integration

XERJ includes a built-in `stdio` MCP server that exposes search, symbol lookups, and memory tools directly to agent runtimes.

### Automated Setup
```bash
# Wires XERJ into detected agent configurations (Claude Code, Cursor, etc.)
xerj init
```

### Manual Configuration

#### Antigravity & Gemini CLI (`~/.gemini/antigravity.json`)
```json
{
  "mcpServers": {
    "xerj": {
      "command": "xerj",
      "args": ["mcp"]
    }
  }
}
```

#### OpenCode (`~/.opencode/mcp.json` or project `.opencode/mcp.json`)
```json
{
  "mcpServers": {
    "xerj": {
      "command": "xerj",
      "args": ["mcp"]
    }
  }
}
```

#### Claude Desktop & Claude Code (`~/.claude.json`)
```json
{
  "mcpServers": {
    "xerj": {
      "command": "xerj",
      "args": ["mcp"]
    }
  }
}
```

### Available MCP Tools
When `xerj mcp` is active, agents gain access to:
- `xerj_search`: Full-text and query-DSL search over indexed code and data.
- `xerj_semantic_search`: Dense vector kNN search by semantic meaning.
- `xerj_hybrid_search`: Combined lexical BM25 and vector ranking.
- `xerj_memory_store`: Saves context or notes into durable namespace-isolated indices.
- `xerj_memory_recall`: Retrieves stored memories by keyword or semantic similarity.
- `xerj_brain_overview` & `xerj_brain_link`: Knowledge graph relationships and second-brain navigation.

---

## 5. Operational Rules for AI Agents

1. **Avoid Dumping Large Files**: Use `xerj def "<symbol>"` or `xerj search "<query>"` to retrieve specific passages and line coordinates before reading files.
2. **Auto-Index on Project Entry**: Run `xerj autoindex .` when exploring an unfamiliar or large codebase to prepare the AST index.
3. **Use Namespace-Isolated Memory**: When persisting cross-session knowledge with `xerj_memory_store`, always set the namespace to the project name (e.g. `namespace: "karakuri"`) to prevent memory collisions.
4. **Daemon Security**: If starting a local background server (`xerj --insecure`), ensure it only binds to `127.0.0.1`. Never expose `--insecure` to public networks.
