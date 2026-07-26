# Sentinel Development Plan

Version: 0.1

Status: Draft

---

# Overview

This document defines the development approach for Sentinel.

The goal is to build a stable foundation before adding advanced capabilities.

Sentinel development should follow dependency order:

```
Foundation

↓

Assessment Workflow

↓

Evidence Management

↓

Verification

↓

Reporting

↓

Extensions
```

---

# Development Principles

---

# 1. Build the Core Before Features

The internal model should be stable before adding integrations or automation.

---

# 2. Prefer Working Software Over Large Scope

Each development phase should produce a usable result.

---

# 3. Keep Security Decisions Transparent

The system should always show:

- What happened.
- Why it happened.
- What evidence exists.

---

# 4. Avoid Premature Complexity

Advanced systems should only be added when the core workflow proves the need.

---

# Development Phases

---

# Phase 0: Project Foundation

## Goal

Create the initial project structure.

---

## Tasks

- Initialize Rust project.
- Setup repository structure.
- Add documentation.
- Setup testing environment.
- Setup CI.

---

## Result

A clean and maintainable project foundation.

---

# Phase 1: Sentinel Core

## Goal

Build the internal domain model.

---

## Tasks

Implement:

- Assessment model.
- Target model.
- Workflow model.
- Activity model.
- State management.

---

## Result

Sentinel can represent a security assessment.

---

# Phase 2: CLI Foundation

## Goal

Create the first user interaction layer.

---

## Tasks

Implement:

- CLI commands.
- Input handling.
- Output formatting.
- Basic navigation.

---

## Result

Users can interact with Sentinel.

---

# Phase 3: Web Security Workflow

## Goal

Support the first security assessment workflow.

---

## Focus

Web application security testing.

---

## Tasks

Implement:

- Workflow loading.
- Testing activities.
- Progress tracking.
- Activity completion.

---

## Result

A researcher can follow a structured web assessment.

---

# Phase 4: Evidence Engine

## Goal

Make evidence a core part of the workflow.

---

## Tasks

Implement:

- Evidence creation.
- Evidence storage.
- Evidence linking.
- Evidence review.

---

## Result

Testing activities produce organized evidence.

---

# Phase 5: Verification System

## Goal

Separate observations from confirmed findings.

---

## Tasks

Implement:

- Finding lifecycle.
- Verification states.
- Confidence tracking.
- Evidence requirements.

---

## Result

Sentinel can manage verified security findings.

---

# Phase 6: Reporting Engine

## Goal

Generate useful security reports.

---

## Tasks

Implement:

- Report templates.
- Finding formatting.
- Evidence references.
- Export support.

---

## Result

Complete assessment reports can be generated.

---

# Phase 7: Quality Improvements

## Goal

Improve reliability and usability.

---

## Tasks

Add:

- Better error handling.
- Documentation improvements.
- Performance improvements.
- User experience improvements.

---

# Phase 8: Future Extensions

Possible future work:

- External tool integrations.
- Custom workflows.
- Plugin system.
- Team collaboration.

---

# Initial MVP Scope

The first usable Sentinel version should contain:

```
CLI

+

Assessment Management

+

Web Security Workflow

+

Evidence Management

+

Finding Verification

+

Basic Reporting
```

---

# Out of MVP Scope

The first version should not include:

- Plugin marketplace.
- Multiple security domains.
- Team collaboration.
- Cloud synchronization.
- Advanced automation.

---

# Success Criteria

The MVP is successful when a researcher can:

1. Create an assessment.
2. Define a target.
3. Follow a web security workflow.
4. Record testing activities.
5. Store evidence.
6. Verify findings.
7. Generate a report.

---

# Summary

Sentinel should be built as a workflow-driven security assessment framework.

The development strategy prioritizes:

- Strong foundations.
- Clear architecture.
- Evidence-driven results.
- Incremental growth.
