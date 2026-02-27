# Agent Guidelines for: Task Manager API

## Overview
This document provides guidelines for AI agents (like Codex) working on this project.

## Project Context
**Initial Prompt**: Build a REST API for managing tasks with CRUD operations

**Intent**: Create a production-ready REST API that allows users to manage their tasks with full CRUD capabilities

**Deliverables**:
- REST API server with CRUD endpoints
- Database schema and migrations
- API documentation (OpenAPI/Swagger)
- Integration tests
- Docker deployment config

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
npm test && npm run lint && docker-compose up -d && curl http://localhost:3000/health

## Constraints

### Hard Constraints (Must-Haves)
- Must use Node.js and Express
- Must use PostgreSQL for persistence
- Must include authentication via JWT

### Soft Constraints (Nice-to-Haves)
- Prefer TypeScript over JavaScript
- Consider rate limiting for production

## Non-Goals
The following are explicitly out of scope:
- Frontend UI
- Real-time websocket updates
- Mobile app support

## Acceptance Criteria
This project is considered complete when:
- All CRUD endpoints respond correctly
- Integration tests have >80% coverage
- API documentation is complete and accurate
- Docker container starts and serves requests
- Authentication middleware protects routes

---
Generated: 2026-02-27 04:19:08 UTC
