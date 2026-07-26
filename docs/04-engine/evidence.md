# Sentinel Evidence Engine

Version: 0.1

Status: Draft

---

# Overview

The Evidence Engine is responsible for collecting, organizing, and managing information produced during security assessments.

Evidence provides the connection between:

Testing Activity

↓

Observed Behavior

↓

Verification

↓

Security Finding

---

# Purpose

The Evidence Engine helps researchers:

- Preserve assessment context.
- Organize collected information.
- Support verification.
- Improve report quality.
- Make assessments reviewable.

---

# Evidence Principles

---

# 1. Evidence Must Have Context

Evidence should never exist as an isolated file or value.

Every piece of evidence should explain:

- Where it came from.
- When it was collected.
- Why it matters.
- Which activity it supports.

---

# 2. Evidence Does Not Equal Finding

Collected evidence is not automatically a vulnerability.

The relationship is:

```
Evidence

↓

Analysis

↓

Verification

↓

Finding
```

---

# 3. Evidence Should Be Traceable

A reviewer should be able to understand:

- Which activity created the evidence.
- Who collected it.
- Which finding uses it.

---

# Evidence Types

---

# HTTP Evidence

## Purpose

Store web communication information.

Examples:

- Request.
- Response.
- Headers.
- Parameters.
- Status information.

---

# Screenshot Evidence

## Purpose

Store visual confirmation.

Examples:

- Application behavior.
- Interface state.
- Error messages.

---

# Log Evidence

## Purpose

Store technical records.

Examples:

- Application logs.
- Server logs.
- Tool output.

---

# Note Evidence

## Purpose

Store researcher observations.

Examples:

- Testing notes.
- Assumptions.
- Investigation details.

---

# File Evidence

## Purpose

Store related files.

Examples:

- Configuration files.
- Exported data.
- Assessment artifacts.

---

# Evidence Lifecycle

```
Created

↓

Collected

↓

Reviewed

↓

Linked

↓

Archived
```

---

# Evidence States

---

# Created

A placeholder exists for expected evidence.

Example:

A testing activity requires a response capture.

---

# Collected

Evidence has been captured.

---

# Reviewed

Evidence quality and relevance were checked.

---

# Linked

Evidence is connected to:

- Activity.
- Result.
- Finding.

---

# Archived

Evidence remains available for historical purposes.

---

# Evidence Metadata

Every evidence item should contain:

```
Evidence ID

Type

Created Date

Source

Related Activity

Related Assessment

Description

Content Reference

Notes
```

---

# Evidence Relationships

Evidence can be connected to:

```
Assessment

    |

Activity

    |

Result

    |

Finding
```

---

# Evidence Quality

Evidence quality can be evaluated by:

---

## Relevance

Does the evidence support the conclusion?

---

## Completeness

Does it contain enough information?

---

## Clarity

Can another person understand it?

---

## Reproducibility

Can the situation be understood and repeated?

---

# Evidence Rules

---

## Rule 1

Evidence should not be modified silently.

Changes should preserve history.

---

## Rule 2

Evidence should maintain original context.

---

## Rule 3

Sensitive information should be handled carefully.

---

## Rule 4

Evidence collection should respect authorization boundaries.

---

# Future Evidence Features

Possible future improvements:

- Evidence tagging.
- Evidence search.
- Evidence comparison.
- Evidence integrity verification.
- Evidence export packages.

---

# Summary

The Evidence Engine makes security assessment results transparent and reviewable.

It ensures that Sentinel does not only record conclusions, but preserves the reasoning and information behind those conclusions.
