use crate::spec::Spec;
use anyhow::Result;
use chrono::Utc;
use serde_json::json;
use std::fs;
use std::path::PathBuf;

pub struct FileGenerator;

impl FileGenerator {
    pub fn generate_all(spec: &Spec, output_dir: &PathBuf) -> Result<Vec<String>> {
        fs::create_dir_all(output_dir)?;
        fs::create_dir_all(output_dir.join(".agent"))?;

        let mut generated_files = Vec::new();

        // 1. AGENTS.md
        let agents_md = Self::generate_agents_md(spec);
        fs::write(output_dir.join("AGENTS.md"), &agents_md)?;
        generated_files.push("AGENTS.md".to_string());

        // 2. .agent/PLANS.md
        let plans_md = Self::generate_plans_md();
        fs::write(output_dir.join(".agent/PLANS.md"), &plans_md)?;
        generated_files.push(".agent/PLANS.md".to_string());

        // 3. .agent/EXECPLAN.md
        let execplan_md = Self::generate_execplan_md(spec);
        fs::write(output_dir.join(".agent/EXECPLAN.md"), &execplan_md)?;
        generated_files.push(".agent/EXECPLAN.md".to_string());

        // 4. ONE_SHOT_PROMPT.md
        let one_shot = Self::generate_one_shot_prompt(spec);
        fs::write(output_dir.join("ONE_SHOT_PROMPT.md"), &one_shot)?;
        generated_files.push("ONE_SHOT_PROMPT.md".to_string());

        // 5. SPEC.json
        let spec_json = serde_json::to_string_pretty(spec)?;
        fs::write(output_dir.join("SPEC.json"), &spec_json)?;
        generated_files.push("SPEC.json".to_string());

        // 6. MANIFEST.json
        let manifest = Self::generate_manifest(spec, &generated_files);
        fs::write(output_dir.join("MANIFEST.json"), &manifest)?;
        generated_files.push("MANIFEST.json".to_string());

        // 7. README.md
        let readme = Self::generate_readme(spec);
        fs::write(output_dir.join("README.md"), &readme)?;
        generated_files.push("README.md".to_string());

        Ok(generated_files)
    }

    fn generate_agents_md(spec: &Spec) -> String {
        format!(
            r#"# Agent Guidelines for: {}

## Overview
This document provides guidelines for AI agents (like Codex) working on this project.

## Project Context
**Initial Prompt**: {}

**Intent**: {}

**Deliverables**:
{}

## ExecPlans

When implementing complex features or refactors for this project, agents should use an **ExecPlan** approach as described in `.agent/PLANS.md`.

### Key Principles
- Break down complex work into milestones
- Make progress visible through tracking
- Validate at each gate before proceeding
- Document decisions and trade-offs

## Current ExecPlan
See `.agent/EXECPLAN.md` for the current execution plan for this project.

## Validation Requirements
{}

## Constraints

### Hard Constraints (Must-Haves)
{}

### Soft Constraints (Nice-to-Haves)
{}

## Non-Goals
The following are explicitly out of scope:
{}

## Acceptance Criteria
This project is considered complete when:
{}

---
Generated: {}
"#,
            spec.title,
            spec.initial_prompt,
            spec.intent.as_ref().unwrap_or(&"Not specified".to_string()),
            spec.deliverables
                .iter()
                .map(|d| format!("- {}", d))
                .collect::<Vec<_>>()
                .join("\n"),
            spec.validation_plan
                .as_ref()
                .unwrap_or(&"Not specified".to_string()),
            if spec.constraints.hard.is_empty() {
                "- None specified".to_string()
            } else {
                spec.constraints
                    .hard
                    .iter()
                    .map(|c| format!("- {}", c))
                    .collect::<Vec<_>>()
                    .join("\n")
            },
            if spec.constraints.soft.is_empty() {
                "- None specified".to_string()
            } else {
                spec.constraints
                    .soft
                    .iter()
                    .map(|c| format!("- {}", c))
                    .collect::<Vec<_>>()
                    .join("\n")
            },
            spec.non_goals
                .iter()
                .map(|ng| format!("- {}", ng))
                .collect::<Vec<_>>()
                .join("\n"),
            spec.acceptance_criteria
                .iter()
                .map(|ac| format!("- {}", ac))
                .collect::<Vec<_>>()
                .join("\n"),
            Utc::now().format("%Y-%m-%d %H:%M:%S UTC")
        )
    }

    fn generate_plans_md() -> String {
        r#"# ExecPlan Requirements

## What is an ExecPlan?

An ExecPlan is a structured, milestone-driven approach to implementing complex features or refactors. It ensures that work is:
- **Self-contained**: Complete enough for any developer to understand and execute
- **Plain-language**: Written for clarity, not showing off expertise
- **Demonstrable**: Focuses on working behavior, not just code changes
- **Trackable**: Includes clear milestones and progress indicators

## Required Elements

Every ExecPlan MUST include:

1. **Objective** (1-2 sentences)
   - What are we building/changing and why?

2. **Context** (brief)
   - Current state
   - Key dependencies or constraints
   - What needs to change

3. **Milestones** (numbered list)
   - Break work into 3-7 major steps
   - Each milestone should be verifiable
   - Order matters: dependencies should be clear

4. **Success Criteria**
   - How do we know each milestone is done?
   - What tests/commands verify correctness?

5. **Progress Tracking**
   - Use checkboxes: `- [x]` for done, `- [ ]` for pending
   - Update as work progresses

## Writing Style

### ✅ Do
- Define technical terms when first used
- Use concrete examples
- Write for a junior developer audience
- Focus on "what" and "why" before "how"
- Keep language direct and unambiguous

### ❌ Don't
- Assume deep domain knowledge
- Use jargon without explanation
- Skip validation steps
- Make milestones too large or vague
- Nest code blocks (use single-level fences only)

## Template

```md
# ExecPlan: [Feature/Change Name]

## Objective
[1-2 sentence description]

## Context
- Current state: [brief]
- Why this change: [brief]
- Key constraints: [if any]

## Milestones

### Milestone 1: [Name]
- [ ] Task 1
- [ ] Task 2
- **Validation**: [how to verify]

### Milestone 2: [Name]
- [ ] Task 1
- [ ] Task 2
- **Validation**: [how to verify]

[... more milestones ...]

## Success Criteria
- [ ] Criterion 1
- [ ] Criterion 2

## Notes
[Any additional context, trade-offs, or decisions]
```

## Formatting Note

When embedding an ExecPlan in other documentation or prompts, the entire plan should be wrapped in a single markdown code fence (triple backticks). Do not use nested code fences within the plan content itself.
"#.to_string()
    }

    fn generate_execplan_md(spec: &Spec) -> String {
        let env_desc = if let Some(lang) = &spec.target_environment.language {
            format!("Language: {}", lang)
        } else if let Some(toolchain) = &spec.target_environment.toolchain {
            format!("Toolchain: {}", toolchain)
        } else {
            "Environment: Not specified".to_string()
        };

        format!(
            r#"# ExecPlan: {}

## Objective
{}

## Context
- **Initial Request**: {}
- **{}**
- **Deliverables**: {}

## Milestones

{}

## Success Criteria
{}

## Validation Plan
{}

## Notes
- Non-goals: {}
- Hard constraints: {}
- Soft constraints: {}

---
This plan was generated from a structured specification interview.
Update progress as milestones are completed.
"#,
            spec.title,
            spec.intent
                .as_ref()
                .unwrap_or(&"Not specified".to_string()),
            spec.initial_prompt,
            env_desc,
            spec.deliverables.join(", "),
            Self::generate_milestones_from_deliverables(&spec.deliverables),
            spec.acceptance_criteria
                .iter()
                .map(|ac| format!("- [ ] {}", ac))
                .collect::<Vec<_>>()
                .join("\n"),
            spec.validation_plan
                .as_ref()
                .unwrap_or(&"Run tests and manual verification".to_string()),
            spec.non_goals.join(", "),
            spec.constraints.hard.join(", "),
            spec.constraints.soft.join(", ")
        )
    }

    fn generate_milestones_from_deliverables(deliverables: &[String]) -> String {
        if deliverables.is_empty() {
            return "### Milestone 1: Initial Implementation\n- [ ] Complete implementation\n- **Validation**: Verify basic functionality\n".to_string();
        }

        deliverables
            .iter()
            .enumerate()
            .map(|(i, deliverable)| {
                format!(
                    "### Milestone {}: {}\n- [ ] Implement {}\n- [ ] Test {}\n- **Validation**: Verify {} works as expected\n",
                    i + 1,
                    deliverable,
                    deliverable,
                    deliverable,
                    deliverable
                )
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn generate_one_shot_prompt(spec: &Spec) -> String {
        format!(
            r#"# One-Shot Prompt for Codex

You are Codex with full repo read/write capabilities and command execution.

## Mission
Implement the following project according to the ExecPlan methodology.

## Step 1: Read Context Documents
Before starting, read these files in order:
1. `AGENTS.md` - Understand project guidelines and constraints
2. `.agent/PLANS.md` - Learn the ExecPlan requirements
3. `.agent/EXECPLAN.md` - This is your SOURCE OF TRUTH

## Step 2: Execute the Plan
Follow `.agent/EXECPLAN.md` milestone by milestone:
- Complete each milestone fully before moving to the next
- Run validation steps after each milestone
- Update progress tracking in `.agent/EXECPLAN.md` as you work
- If you discover issues, document them and adjust the plan

## Step 3: Validate Continuously
After each milestone:
- Run the validation commands specified in the plan
- Verify acceptance criteria
- Ensure no regressions
- Update the progress log

## Project Summary
**Title**: {}
**Intent**: {}
**Deliverables**: {}

## Validation Plan
{}

## Constraints
**Hard**: {}
**Soft**: {}

## Non-Goals
{}

## Acceptance Criteria
{}

## How to Begin
1. Read the three context files mentioned in Step 1
2. Start with Milestone 1 in `.agent/EXECPLAN.md`
3. Update the plan's checkboxes as you complete tasks
4. Run validations at each gate
5. Continue until all acceptance criteria are met

---
**Note**: This is a single, self-contained prompt. Paste it into Codex to start execution.
"#,
            spec.title,
            spec.intent
                .as_ref()
                .unwrap_or(&"Not specified".to_string()),
            spec.deliverables.join(", "),
            spec.validation_plan
                .as_ref()
                .unwrap_or(&"Run tests".to_string()),
            spec.constraints.hard.join(", "),
            spec.constraints.soft.join(", "),
            spec.non_goals.join(", "),
            spec.acceptance_criteria.join(", ")
        )
    }

    fn generate_manifest(spec: &Spec, _generated_files: &[String]) -> String {
        let manifest = json!({
            "project": spec.title,
            "slug": spec.slug,
            "version": spec.version,
            "generated_at": Utc::now().to_rfc3339(),
            "files": [
                {
                    "path": "AGENTS.md",
                    "purpose": "Project guidelines and context for AI agents",
                    "usage": "Read this file to understand project constraints and requirements"
                },
                {
                    "path": ".agent/PLANS.md",
                    "purpose": "ExecPlan methodology documentation",
                    "usage": "Reference this to understand how to structure and follow execution plans"
                },
                {
                    "path": ".agent/EXECPLAN.md",
                    "purpose": "The actual execution plan for this project",
                    "usage": "Follow this milestone-by-milestone to implement the project"
                },
                {
                    "path": "ONE_SHOT_PROMPT.md",
                    "purpose": "Single prompt to paste into Codex for execution",
                    "usage": "Copy and paste this entire file into Codex to start implementation"
                },
                {
                    "path": "SPEC.json",
                    "purpose": "Machine-readable frozen specification",
                    "usage": "Reference this for programmatic access to project requirements"
                },
                {
                    "path": "README.md",
                    "purpose": "Human-readable guide for using this prompt pack",
                    "usage": "Start here to understand how to use these generated files"
                },
                {
                    "path": "MANIFEST.json",
                    "purpose": "Index of all generated files with purposes",
                    "usage": "This file - provides overview of the prompt pack contents"
                }
            ],
            "how_to_use": [
                "1. Read README.md in this directory for an overview",
                "2. Copy the contents of ONE_SHOT_PROMPT.md",
                "3. Paste it into Codex (cloud or CLI) in your target repository",
                "4. Codex will read the .agent files and execute the plan",
                "5. Monitor progress by checking updates to .agent/EXECPLAN.md"
            ]
        });

        serde_json::to_string_pretty(&manifest).unwrap()
    }

    fn generate_readme(spec: &Spec) -> String {
        format!(
            r#"# Prompt Pack: {}

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

**Intent**: {}

**Deliverables**:
{}

**Validation Plan**: {}

**Acceptance Criteria**:
{}

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
{}
```

## Need Help?

- Review `AGENTS.md` for project context
- Check `.agent/PLANS.md` for methodology details
- Examine `SPEC.json` for the complete specification
- Ask Codex to explain its progress on the current milestone

---
Generated: {}
"#,
            spec.title,
            spec.intent
                .as_ref()
                .unwrap_or(&"Not specified".to_string()),
            spec.deliverables
                .iter()
                .map(|d| format!("- {}", d))
                .collect::<Vec<_>>()
                .join("\n"),
            spec.validation_plan
                .as_ref()
                .unwrap_or(&"Run tests".to_string()),
            spec.acceptance_criteria
                .iter()
                .map(|ac| format!("- {}", ac))
                .collect::<Vec<_>>()
                .join("\n"),
            spec.validation_plan
                .as_ref()
                .unwrap_or(&"# Run your validation commands here".to_string()),
            Utc::now().format("%Y-%m-%d %H:%M:%S UTC")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_generate_all_files() {
        let mut spec = Spec::new("Test Project".to_string(), "Build a tool".to_string());
        spec.intent = Some("Test intent".to_string());
        spec.deliverables.push("Feature A".to_string());
        spec.constraints.hard.push("Must be fast".to_string());
        spec.non_goals.push("Not mobile".to_string());
        spec.acceptance_criteria
            .push("All tests pass".to_string());
        spec.validation_plan = Some("cargo test".to_string());
        spec.target_environment.language = Some("Rust".to_string());

        let temp_dir = TempDir::new().unwrap();
        let output_path = temp_dir.path().to_path_buf();

        let files = FileGenerator::generate_all(&spec, &output_path).unwrap();

        assert!(files.contains(&"AGENTS.md".to_string()));
        assert!(files.contains(&".agent/PLANS.md".to_string()));
        assert!(files.contains(&".agent/EXECPLAN.md".to_string()));
        assert!(files.contains(&"ONE_SHOT_PROMPT.md".to_string()));
        assert!(files.contains(&"SPEC.json".to_string()));
        assert!(files.contains(&"MANIFEST.json".to_string()));
        assert!(files.contains(&"README.md".to_string()));

        // Verify files exist
        assert!(output_path.join("AGENTS.md").exists());
        assert!(output_path.join(".agent/PLANS.md").exists());
        assert!(output_path.join(".agent/EXECPLAN.md").exists());
        assert!(output_path.join("ONE_SHOT_PROMPT.md").exists());
    }
}
