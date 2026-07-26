# Sentinel Coding Standards

Version: 0.1

Status: Draft

---

# Overview

This document defines coding standards and engineering practices for Sentinel.

The goal is to maintain:

- Readable code.
- Predictable architecture.
- Easy maintenance.
- Consistent development practices.

---

# General Principles

---

# 1. Prefer Simplicity

Code should solve the current problem without unnecessary complexity.

Avoid:

- Premature abstractions.
- Unused frameworks.
- Over-engineering.

---

# 2. Clear Responsibilities

Each module should have a clear purpose.

Example:

Good:

```
evidence/
    manages evidence
```

Bad:

```
utils/
    contains everything
```

---

# 3. Explicit Over Implicit

Code behavior should be easy to understand.

Avoid hidden logic.

Prefer:

- Clear naming.
- Small functions.
- Defined interfaces.

---

# 4. Maintain Domain Language

Code should use Sentinel terminology.

Examples:

Prefer:

```
Assessment
Finding
Evidence
Workflow
Activity
```

Avoid vague names:

```
Data
Object
Manager
Handler
Thing
```

---

# Rust Guidelines

---

# Naming

Use Rust conventions:

## Types

PascalCase:

```
Assessment
SecurityFinding
EvidenceItem
```

---

## Functions and Variables

snake_case:

```
create_assessment()

load_workflow()

verify_finding()
```

---

# Error Handling

Errors should provide context.

Avoid:

```
failed
```

Prefer:

```
failed to load assessment configuration
```

---

# No Silent Failures

The system should never ignore important errors.

---

# Module Structure

Modules should follow responsibility boundaries.

Example:

```
src/

├── assessment/
├── workflow/
├── evidence/
├── verification/
├── reporting/
└── storage/
```

---

# Testing Standards

---

# Unit Tests

Used for:

- Domain logic.
- State transitions.
- Data validation.

---

# Integration Tests

Used for:

- CLI workflows.
- Component communication.

---

# Test Requirements

Important behavior should have tests.

Especially:

- State transitions.
- Verification rules.
- Evidence relationships.

---

# Documentation Standards

---

# Code Comments

Comments should explain:

- Why something exists.
- Important decisions.

Avoid comments that only repeat the code.

---

Bad:

```
// create assessment
create_assessment();
```

Good:

```
// Assessment creation validates scope before activation
```

---

# Commit Standards

Commits should describe changes clearly.

Examples:

Good:

```
add assessment state transitions
```

Bad:

```
update stuff
```

---

# Pull Request Standards

A pull request should include:

- What changed.
- Why it changed.
- Testing performed.
- Possible risks.

---

# Security Development Rules

---

# 1. Never Hide Security Decisions

Important security behavior should be visible and traceable.

---

# 2. Avoid Unsafe Operations

Unsafe code should only be used when necessary and documented.

---

# 3. Validate External Input

All external input should be treated as untrusted.

---

# 4. Preserve Evidence Integrity

Assessment data should not be modified unexpectedly.

---

# Dependency Rules

Dependencies should be added only when:

- They solve a real problem.
- They are maintained.
- They do not unnecessarily increase complexity.

---

# Code Review Checklist

Before merging:

```
Is the code readable?

Does it follow architecture boundaries?

Are errors handled?

Are tests included?

Does it introduce unnecessary complexity?
```

---

# Summary

Sentinel code should prioritize:

- Clarity.
- Maintainability.
- Security.
- Domain consistency.

The best code is not the most complex code.

It is the code that keeps the system understandable as it grows.
