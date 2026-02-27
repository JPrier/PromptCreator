# Prompt Pack: Task Manager API

This directory contains a complete "prompt pack" - a set of files designed to guide an AI coding agent (like Codex) through implementing your project.

## What's Included

- **ONE_SHOT_PROMPT.md**: The main prompt to paste into Codex
- **AGENTS.md**: Project context and guidelines for the agent
- **.agent/PLANS.md**: Methodology for structured execution
- **.agent/EXECPLAN.md**: The actual execution plan with milestones
- **SPEC.json**: Machine-readable specification
- **MANIFEST.json**: Index of all files
- **README.md**: This file

## Quick Start

### Option 1: One-Shot Execution (Recommended)

1. Open your target repository in Codex (cloud or CLI)
2. Copy the entire contents of `ONE_SHOT_PROMPT.md`
3. Paste it into Codex
4. Codex will automatically:
   - Read the context files
   - Follow the execution plan
   - Update progress as it works
   - Validate at each milestone

### Option 2: Manual Step-by-Step

1. Share the `.agent/` directory and `AGENTS.md` with Codex
2. Ask Codex to read and understand the context
3. Request that Codex execute the plan in `.agent/EXECPLAN.md`
4. Monitor progress and provide guidance as needed

## Project Summary

**Intent**: Create a production-ready REST API that allows users to manage their tasks with full CRUD capabilities

**Deliverables**:
- REST API server with CRUD endpoints
- Database schema and migrations
- API documentation (OpenAPI/Swagger)
- Integration tests
- Docker deployment config

**Validation Plan**: npm test && npm run lint && docker-compose up -d && curl http://localhost:3000/health

**Acceptance Criteria**:
- All CRUD endpoints respond correctly
- Integration tests have >80% coverage
- API documentation is complete and accurate
- Docker container starts and serves requests
- Authentication middleware protects routes

## How It Works

The prompt pack uses an "ExecPlan" methodology:
- Work is broken into clear milestones
- Each milestone has validation gates
- Progress is tracked with checkboxes
- The agent updates the plan as it works

This ensures:
- ✅ Systematic progress
- ✅ Verifiable results at each step
- ✅ Clear communication of status
- ✅ Easy recovery if interrupted

## Monitoring Progress

As Codex works, it will:
- Update checkboxes in `.agent/EXECPLAN.md`
- Run validation commands
- Document decisions and issues
- Commit changes incrementally

You can check progress by viewing `.agent/EXECPLAN.md` in your repository.

## Validation

To verify the implementation:

```bash
npm test && npm run lint && docker-compose up -d && curl http://localhost:3000/health
```

## Need Help?

- Review `AGENTS.md` for project context
- Check `.agent/PLANS.md` for methodology details
- Examine `SPEC.json` for the complete specification
- Ask Codex to explain its progress on the current milestone

---
Generated: 2026-02-27 04:19:08 UTC
