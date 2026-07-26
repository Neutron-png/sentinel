# Sentinel Repository Structure

Version: 0.1

Status: Draft

---

# Overview

This document describes the organization of the Sentinel repository.

The goal of this structure is to maintain clear boundaries between:

- Source code.
- Documentation.
- Testing.
- Automation.
- Project configuration.

The repository should remain easy to navigate as Sentinel grows.

---

# Repository Layout

```
sentinel/

├── .github/
├── docs/
├── src/
├── tests/
├── scripts/
├── assets/
│
├── README.md
├── CONTRIBUTING.md
├── CHANGELOG.md
├── LICENSE
├── .gitignore
└── Cargo.toml
```

---

# Root Files

---

# README.md

## Purpose

Provides a quick introduction to Sentinel.

Contains:

- Project overview.
- Current status.
- Documentation links.
- Basic project information.

---

# CONTRIBUTING.md

## Purpose

Defines contribution guidelines.

Contains:

- Contribution rules.
- Development workflow.
- Code standards.
- Pull request process.

---

# CHANGELOG.md

## Purpose

Tracks project changes between releases.

Contains:

- New features.
- Fixes.
- Breaking changes.

---

# LICENSE

## Purpose

Defines how the project can be used and distributed.

---

# Cargo.toml

## Purpose

Rust project configuration.

Contains:

- Package information.
- Dependencies.
- Build configuration.

---

# .gitignore

## Purpose

Defines files that should not be tracked by Git.

Examples:

- Build artifacts.
- Local configuration.
- Temporary files.

---

# .github/

## Purpose

Contains GitHub-specific project configuration.

Structure:

```
.github/

├── workflows/

├── ISSUE_TEMPLATE/

└── pull_request_template.md
```

---

## workflows/

Contains:

- CI pipelines.
- Automated checks.
- Build validation.

---

## ISSUE_TEMPLATE/

Contains templates for:

- Bug reports.
- Feature requests.
- Research discussions.

---

# docs/

## Purpose

Contains project documentation.

Structure:

```
docs/

├── 00-research/
├── 01-product/
├── 02-architecture/
├── 03-methodology/
├── 04-engine/
└── 05-development/
```

---

# docs/00-research/

## Purpose

Contains research about:

- Existing tools.
- Security methodologies.
- Industry references.

This folder supports architectural decisions.

---

# docs/01-product/

## Purpose

Defines:

- Product vision.
- Problems.
- Goals.
- Users.
- Use cases.

---

# docs/02-architecture/

## Purpose

Defines:

- System design.
- Components.
- Domain concepts.
- Workflows.

---

# docs/03-methodology/

## Purpose

Contains security methodology information.

Examples:

- Testing references.
- Verification rules.
- Security standards.

---

# docs/04-engine/

## Purpose

Contains design documents related to Sentinel engines.

Examples:

- CLI.
- Evidence system.
- Reporting.
- Plugin architecture.

---

# docs/05-development/

## Purpose

Contains engineering process documentation.

Examples:

- Roadmap.
- Coding standards.
- Architecture decisions.

---

# src/

## Purpose

Contains application source code.

The internal structure will evolve based on architecture decisions.

Possible future structure:

```
src/

├── cli/
├── core/
├── workflow/
├── knowledge/
├── evidence/
├── verification/
├── reporting/
└── storage/
```

This structure is a direction, not a final implementation.

---

# tests/

## Purpose

Contains automated tests.

Possible categories:

```
tests/

├── unit/
├── integration/
└── fixtures/
```

---

# scripts/

## Purpose

Contains development and automation scripts.

Examples:

- Build helpers.
- Development utilities.
- Release automation.

---

# assets/

## Purpose

Contains project assets.

Examples:

- Diagrams.
- Images.
- Documentation resources.

---

# Repository Rules

## Rule 1

Documentation should explain decisions, not duplicate code.

---

## Rule 2

Source code organization should follow responsibilities.

---

## Rule 3

Temporary files should not enter the repository.

---

## Rule 4

Major architectural changes should be documented through ADRs.

---

# Summary

The repository structure is designed around clear separation:

```
Product Knowledge

        ↓

Architecture

        ↓

Implementation

        ↓

Testing

        ↓

Documentation
```

The structure should evolve with the project while preserving clear responsibilities.
