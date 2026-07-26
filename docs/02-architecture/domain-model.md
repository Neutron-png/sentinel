# Sentinel Domain Model

Version: 0.1

Status: Draft

---

# Overview

The Domain Model defines the main concepts that exist inside Sentinel and how they relate to each other.

This document focuses on business concepts and responsibilities, not implementation details.

The domain model should remain independent from:

- Programming language.
- Database technology.
- User interface.
- External tools.

---

# Core Domain Concepts

The main entities in Sentinel are:

- Assessment
- Target
- Scope
- Workflow
- Stage
- Activity
- Evidence
- Result
- Finding
- Reference
- Report

---

# Entity Relationships

High-level relationship:

```
Assessment

    |
    |
    +── Target

    |
    |
    +── Workflow

            |
            |
            +── Stage

                    |
                    |
                    +── Activity

                            |
                            |
                            +── Evidence

                            |
                            |
                            +── Result


Assessment

    |
    |
    +── Findings

            |
            |
            +── Evidence

            |
            |
            +── References


Assessment

    |
    |
    +── Report
```

---

# Assessment

## Purpose

Represents a security testing project.

An assessment contains everything related to one security engagement.

---

## Responsibilities

Assessment manages:

- Assessment identity.
- Assessment status.
- Related targets.
- Applied workflows.
- Collected results.
- Generated reports.

---

## Example

```
Web Application Security Assessment

Target:
https://example.com

Methodology:
OWASP WSTG
```

---

# Target

## Purpose

Represents the application or system being tested.

---

## Responsibilities

Target manages:

- Target information.
- Technology information.
- Related scope.

---

## Examples

- Web Application.
- API.
- Mobile Application.
- Game Server.

---

# Scope

## Purpose

Defines what is allowed and expected to be tested.

---

## Responsibilities

Scope manages:

- Included assets.
- Excluded assets.
- Testing boundaries.

---

# Workflow

## Purpose

Represents a structured testing methodology.

A workflow defines the path of an assessment.

---

## Examples

- OWASP Web Testing Workflow.
- API Security Workflow.
- Mobile Security Workflow.

---

## Responsibilities

Workflow manages:

- Stages.
- Activities.
- Progress.

---

# Stage

## Purpose

Represents a major testing area.

---

## Examples

- Information Gathering.
- Authentication.
- Authorization.
- Session Management.

---

## Responsibilities

Stage manages:

- Related activities.
- Completion state.
- Dependencies.

---

# Activity

## Purpose

Represents a single testing action or learning unit.

An activity is not necessarily an automated test.

It can be:

- Automated.
- Interactive.
- Manual.

---

## Examples

- Review security headers.
- Verify authorization behavior.
- Test session handling.

---

## Responsibilities

Activity manages:

- Instructions.
- Requirements.
- Expected evidence.
- Completion state.

---

# Evidence

## Purpose

Represents information collected during testing.

Evidence supports conclusions.

---

## Examples

- HTTP Request.
- HTTP Response.
- Screenshot.
- Log.
- Note.

---

## Responsibilities

Evidence manages:

- Source.
- Context.
- Relationship to activity or finding.

---

# Result

## Purpose

Represents the outcome of an activity.

---

## Possible States

- Completed.
- Failed.
- Skipped.
- Requires Review.
- Verified.

---

# Finding

## Purpose

Represents a security issue after verification.

A finding should only exist when enough evidence supports it.

---

## Responsibilities

Finding manages:

- Title.
- Description.
- Impact.
- Severity.
- Evidence.
- References.
- Recommendations.

---

# Reference

## Purpose

Represents the source of security knowledge.

---

## Examples

- OWASP WSTG.
- CWE.
- RFC.
- Official Documentation.

---

# Report

## Purpose

Represents the final assessment output.

---

## Responsibilities

Report contains:

- Assessment information.
- Scope.
- Methodology.
- Findings.
- Evidence.
- Recommendations.

---

# Important Domain Rules

## Rule 1

A Finding should not exist without supporting evidence.

---

## Rule 2

An Activity result is not automatically a Finding.

Testing result:

↓

Verification

↓

Finding

---

## Rule 3

Knowledge does not execute itself.

A reference or methodology only provides guidance.

---

## Rule 4

The user remains responsible for security decisions.

---

# Future Domain Extensions

Possible future entities:

- Team.
- User.
- Integration.
- Plugin.
- Asset Inventory.
- Environment.

These are intentionally excluded from the first version.

---

# Domain Summary

The Sentinel domain can be summarized as:

Assessment

contains

Targets and Workflows.

Workflows contain Stages.

Stages contain Activities.

Activities produce Results and Evidence.

Verified Results become Findings.

Findings become Reports.
