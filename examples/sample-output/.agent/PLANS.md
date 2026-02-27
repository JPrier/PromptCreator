# ExecPlan Requirements

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
