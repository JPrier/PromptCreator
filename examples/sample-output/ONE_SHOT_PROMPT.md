# One-Shot Prompt for Codex

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
**Title**: Task Manager API
**Intent**: Create a production-ready REST API that allows users to manage their tasks with full CRUD capabilities
**Deliverables**: REST API server with CRUD endpoints, Database schema and migrations, API documentation (OpenAPI/Swagger), Integration tests, Docker deployment config

## Validation Plan
npm test && npm run lint && docker-compose up -d && curl http://localhost:3000/health

## Constraints
**Hard**: Must use Node.js and Express, Must use PostgreSQL for persistence, Must include authentication via JWT
**Soft**: Prefer TypeScript over JavaScript, Consider rate limiting for production

## Non-Goals
Frontend UI, Real-time websocket updates, Mobile app support

## Acceptance Criteria
All CRUD endpoints respond correctly, Integration tests have >80% coverage, API documentation is complete and accurate, Docker container starts and serves requests, Authentication middleware protects routes

## How to Begin
1. Read the three context files mentioned in Step 1
2. Start with Milestone 1 in `.agent/EXECPLAN.md`
3. Update the plan's checkboxes as you complete tasks
4. Run validations at each gate
5. Continue until all acceptance criteria are met

---
**Note**: This is a single, self-contained prompt. Paste it into Codex to start execution.
