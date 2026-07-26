# Sentinel Architecture Decision Records

Version: 0.1

Status: Draft

---

# Overview

This document records important architectural decisions made during Sentinel development.

Each decision contains:

- Context.
- Decision.
- Reasoning.
- Consequences.

The purpose is to preserve technical reasoning and avoid repeated discussions.

---

# ADR-001: Build Sentinel as a CLI Application

## Status

Accepted

---

## Context

Security researchers often use command-line tools during technical workflows.

Sentinel requires:

- Fast interaction.
- Automation capability.
- Scriptability.
- Low resource usage.

---

## Decision

Sentinel will initially be built as a CLI-first application.

---

## Reasoning

CLI provides:

- Simple distribution.
- Developer-friendly workflow.
- Easy automation.
- Clear interaction model.

A graphical interface can be considered later.

---

## Consequences

Positive:

- Faster initial development.
- Easier automation.
- Suitable for technical users.

Negative:

- Less accessible for non-technical users.
- Requires good documentation.

---

# ADR-002: Use Rust as the Primary Language

## Status

Accepted

---

## Context

Sentinel requires:

- Reliability.
- Performance.
- Strong tooling.
- Safe system-level development.

---

## Decision

Rust will be used for the core application.

---

## Reasoning

Rust provides:

- Memory safety.
- Good performance.
- Strong type system.
- Modern development ecosystem.

---

## Consequences

Positive:

- Reliable core.
- Good long-term foundation.

Negative:

- Higher learning curve.
- Slower initial development for beginners.

---

# ADR-003: Focus on Web Applications First

## Status

Accepted

---

## Context

The original vision included multiple security domains.

Supporting everything immediately would increase complexity.

---

## Decision

The first version will focus only on web application security assessments.

---

## Reasoning

Web security provides:

- Mature methodologies.
- Clear testing standards.
- Large user base.
- Strong foundation for workflow design.

---

## Consequences

Positive:

- Smaller scope.
- Faster iteration.
- Easier validation.

Negative:

- Other domains are postponed.

---

# ADR-004: Sentinel Is Not an Automated Scanner

## Status

Accepted

---

## Context

Many security tools focus on automated detection.

Sentinel's goal is workflow organization and verification.

---

## Decision

Sentinel will not be designed as a traditional vulnerability scanner.

---

## Reasoning

The project prioritizes:

- Methodology.
- Evidence.
- Verification.
- Researcher control.

---

## Consequences

Positive:

- Higher quality results.
- Lower false confidence.

Negative:

- Requires more user involvement.

---

# ADR-005: Evidence First Architecture

## Status

Accepted

---

## Context

Security conclusions require supporting information.

---

## Decision

Evidence will be a core domain component.

---

## Reasoning

Every important result should be traceable.

---

## Consequences

Positive:

- Better reports.
- Easier reviews.
- Better reproducibility.

Negative:

- More data management complexity.

---

# ADR-006: No AI Dependency in Core Security Logic

## Status

Accepted

---

## Context

AI systems can introduce uncertainty.

Security decisions require transparency.

---

## Decision

Core security decisions will not depend on AI.

---

## Reasoning

Sentinel should rely on:

- Security references.
- Defined workflows.
- Evidence.
- Researcher decisions.

---

## Consequences

Positive:

- More predictable behavior.
- Easier auditing.

Negative:

- Less automation.

---

# ADR-007: Human Verification Before Findings

## Status

Accepted

---

## Context

Detection does not always represent a real vulnerability.

---

## Decision

Findings require verification before confirmation.

---

## Reasoning

This reduces:

- False positives.
- Unsupported conclusions.

---

## Consequences

Positive:

- Higher confidence results.

Negative:

- More researcher involvement.

---

# Future Decisions

Future ADRs may cover:

- Database selection.
- Plugin architecture implementation.
- External integrations.
- Distribution model.
- Security model.

---

# Summary

Sentinel decisions prioritize:

- Transparency.
- Reliability.
- Maintainability.
- Researcher control.

Architectural decisions should always support the original goal:

Creating a structured and evidence-driven web security assessment workflow.
