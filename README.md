# PromptCreator

A programmatic tool that turns one-shot ideas into **Codex long-horizon prompt packs**.

## What It Does

PromptCreator guides you through an interactive interview to clarify your project requirements, then generates a complete "prompt pack" - a set of structured files that guide AI coding agents (like GitHub Codex) through implementing your project systematically.

### The Problem It Solves

When you have a rough idea and want an AI agent to implement it, simply pasting your one-shot prompt often leads to:
- Unclear requirements
- Missing edge cases
- No systematic approach
- Difficulty tracking progress
- No validation plan

### The Solution

PromptCreator enforces a structured workflow:
1. **Mandatory clarification interview** - collects all required fields before proceeding
2. **Generates execution plans** - breaks work into verifiable milestones
3. **Creates pasteable prompts** - one-shot prompts that guide Codex systematically
4. **Enables validation** - defines how to verify success at each step

## Installation

### Prerequisites
- Rust 1.70+ and Cargo

### Build from Source

```bash
git clone https://github.com/JPrier/PromptCreator.git
cd PromptCreator
cargo build --release
```

The binary will be at `target/release/promptpack`.

### Install Locally

```bash
cargo install --path .
```

## Quick Start

### Interactive Wizard (Recommended)

```bash
promptpack wizard \
  --title "Task Manager API" \
  --prompt "Build a REST API for managing tasks with CRUD operations" \
  --out outputs
```

The wizard will ask clarifying questions about:
- Intent and objectives
- Concrete deliverables
- Constraints (hard and soft)
- Non-goals
- Acceptance criteria
- Validation plan
- Target environment

After the interview, it generates a complete prompt pack in `outputs/task-manager-api/`.

### Generate from Existing Spec

If you already have a `SPEC.json`:

```bash
promptpack generate \
  --spec examples/task-manager-api-spec.json \
  --out outputs/task-manager-api
```

### Resume a Session

If you need to pause and resume:

```bash
promptpack resume --state outputs/task-manager-api/STATE.json
```

### Validate a Spec

Check if a spec has all required fields:

```bash
promptpack validate --spec outputs/task-manager-api/SPEC.json
```

## Generated Files

Each prompt pack contains:

| File | Purpose |
|------|---------|
| `ONE_SHOT_PROMPT.md` | Main prompt to paste into Codex |
| `AGENTS.md` | Project guidelines for the AI agent |
| `.agent/PLANS.md` | ExecPlan methodology documentation |
| `.agent/EXECPLAN.md` | Actual execution plan with milestones |
| `SPEC.json` | Machine-readable frozen specification |
| `MANIFEST.json` | Index of all files with usage info |
| `README.md` | Guide for using the prompt pack |
| `STATE.json` | Session state (for resuming) |

## Using the Generated Prompt Pack

### With GitHub Codex

1. Open your target repository in GitHub Codex (cloud or CLI)
2. Copy the entire contents of `ONE_SHOT_PROMPT.md`
3. Paste it into Codex
4. Codex will:
   - Read the context files
   - Execute the plan milestone-by-milestone
   - Update progress as it works
   - Validate at each gate

### Monitoring Progress

As Codex works, check `.agent/EXECPLAN.md` in your repo to see:
- Which milestones are complete (✅ checkboxes)
- Current status and any issues
- Next steps

## Example Session

```bash
$ promptpack wizard --title "Weather CLI" --prompt "Build a CLI that shows weather forecasts"

🎯 Welcome to PromptPack Wizard!
I'll ask you some questions to clarify your requirements.

--- Round 1 ---

Question 1 of 6
📋 What is the primary intent or objective of this project? (1-2 sentences)
   💡 Hint: Describe what you want to accomplish and why
Your answer: Create a command-line tool that fetches and displays weather forecasts for any city

Question 2 of 6
📋 What are the concrete deliverables? (comma-separated list)
   💡 Hint: Example: CLI tool, REST API, documentation
Your answer: CLI binary, README with usage examples, Unit tests

[... more questions ...]

✅ All required fields collected!

📝 Generating prompt pack files...

✅ Successfully generated 7 files in: outputs/weather-cli

🚀 Next steps:
   1. Read outputs/weather-cli/README.md
   2. Copy outputs/weather-cli/ONE_SHOT_PROMPT.md into Codex
   3. Watch Codex execute the plan!
```

## Architecture

```
promptpack (CLI)
├── wizard: Interactive interview
│   ├── Enforces required fields
│   ├── Generates clarifying questions
│   └── Collects structured answers
├── generator: File generation
│   ├── AGENTS.md (project guidelines)
│   ├── .agent/PLANS.md (methodology)
│   ├── .agent/EXECPLAN.md (actual plan)
│   ├── ONE_SHOT_PROMPT.md (pasteable)
│   ├── SPEC.json (machine-readable)
│   ├── MANIFEST.json (file index)
│   └── README.md (usage guide)
└── state: Session persistence
    ├── Save/load STATE.json
    └── Resume interrupted sessions
```

## Required Fields

The wizard enforces collection of these fields:

- **Intent/Objective**: What and why (1-2 sentences)
- **Concrete Deliverables**: Explicit list of what to build
- **Constraints**: Hard (must-have) and soft (nice-to-have)
- **Non-goals**: Explicitly out of scope
- **Acceptance Criteria**: "Done when..." statements
- **Validation Plan**: How to verify success (commands or rubric)
- **Target Environment**: Language, toolchain, OS, runtime

## Testing

Run the test suite:

```bash
cargo test
```

Run with verbose output:

```bash
cargo test -- --nocapture
```

## Examples

See `examples/` directory for sample specs:
- `examples/task-manager-api-spec.json` - REST API project spec

Generate a prompt pack from an example:

```bash
promptpack generate \
  --spec examples/task-manager-api-spec.json \
  --out /tmp/test-output
```

## Configuration

### Default Values

- **Output directory**: `outputs/<slug>/`
- **Questions per round**: 6
- **Max rounds**: 10
- **LLM mode**: `off` (deterministic templates only)
- **Spec version**: `1.0`

### Force Mode

Use `--force` to generate files even if spec is incomplete:

```bash
promptpack wizard --title "My Project" --prompt "Build something" --force
```

This will document assumptions in the output.

## Non-Interactive Mode

For CI/CD or testing, you can skip the interactive wizard:

```bash
# Generate directly from spec
promptpack generate --spec SPEC.json --out outputs/my-project
```

## Future Features

- [ ] LLM-assisted question generation (OpenAI API integration)
- [ ] LLM-assisted plan drafting
- [ ] Custom question banks
- [ ] Multiple output formats (Markdown, HTML, PDF)
- [ ] Template customization
- [ ] Multi-language support

## Contributing

Contributions welcome! Please:
1. Fork the repository
2. Create a feature branch
3. Add tests for new functionality
4. Ensure `cargo test` passes
5. Submit a pull request

## License

MIT License - see LICENSE file for details

## Credits

Built with:
- Rust + Cargo
- clap (CLI parsing)
- serde (JSON serialization)
- dialoguer (interactive prompts)
- anyhow (error handling)
