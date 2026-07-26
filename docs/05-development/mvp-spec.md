# Sentinel MVP Specification

Version: 0.1

Status: Draft

---

# Overview

This document defines the minimum viable version of Sentinel.

The MVP focuses on validating the core idea:

> A structured, evidence-driven workflow for web application security assessments.

The goal is not to build a complete security platform.

The goal is to prove that Sentinel can help researchers organize testing, verification, and reporting.

---

# MVP Goal

The first Sentinel version should allow a researcher to:

1. Create a security assessment.
2. Define a web application target.
3. Follow a structured testing workflow.
4. Record testing activities.
5. Store evidence.
6. Track potential issues.
7. Verify findings.
8. Generate a basic security report.

---

# MVP Scope

---

# 1. Assessment Management

## Included

Users can:

- Create assessments.
- Open existing assessments.
- View assessment status.
- Track progress.

---

## Not Included

- Team collaboration.
- Cloud synchronization.
- User accounts.

---

# 2. Target Management

## Included

Users can:

- Add web application targets.
- Define basic scope information.
- Associate targets with assessments.

---

## Not Included

- Automatic asset discovery.
- Large asset inventory systems.

---

# 3. Web Security Workflow

## Included

Support a structured workflow for:

- Information gathering.
- Authentication testing.
- Authorization testing.
- Session testing.
- Input validation testing.
- Business logic review.

---

## Not Included

- Complete automated vulnerability scanning.
- Automatic exploitation.
- Autonomous testing.

---

# 4. Activity Management

## Included

Users can:

- View activities.
- Mark activities as completed.
- Add notes.
- Track progress.

---

## Not Included

- Complex workflow customization.

---

# 5. Evidence Management

## Included

Users can:

- Add evidence.
- Organize evidence.
- Link evidence to activities.

Supported initial types:

- Notes.
- Requests.
- Responses.
- Screenshots references.

---

## Not Included

- Advanced evidence analytics.
- Team evidence review.

---

# 6. Finding Verification

## Included

Users can:

- Create potential findings.
- Add verification information.
- Mark findings as verified or rejected.

---

## Not Included

- Automatic vulnerability confirmation.

---

# 7. Reporting

## Included

Generate basic reports containing:

- Assessment information.
- Scope.
- Methodology.
- Findings.
- Evidence references.

---

## Not Included

- Advanced templates.
- Client branding.
- PDF generation initially.

---

# MVP Technical Requirements

---

# Interface

Initial interface:

CLI application.

---

# Storage

Initial storage:

Local project-based storage.

---

# Architecture

The MVP should maintain separation between:

- CLI.
- Core logic.
- Workflow.
- Evidence.
- Verification.
- Reporting.
- Storage.

---

# Security Requirements

Sentinel should:

- Preserve assessment history.
- Validate user input.
- Avoid silent failures.
- Keep security decisions traceable.

---

# MVP Success Criteria

The MVP is successful if a researcher can complete:

```
Create Assessment

↓

Add Target

↓

Start Web Workflow

↓

Complete Activities

↓

Attach Evidence

↓

Verify Finding

↓

Generate Report
```

without manually managing the entire process outside Sentinel.

---

# MVP Exclusions

The following are intentionally postponed:

- AI features.
- Mobile security.
- Game security.
- Cloud platform.
- Plugin marketplace.
- Automated exploitation.
- Large-scale scanning.

---

# Future Direction

After validating the MVP, Sentinel may expand into:

- Additional workflows.
- External integrations.
- Advanced automation.
- Collaboration features.

---

# Final MVP Statement

Sentinel MVP is a web application security assessment workflow tool focused on organization, evidence, verification, and reporting.

It is not a scanner.

It is not an autonomous pentester.

It is a framework that helps researchers perform better assessments.
