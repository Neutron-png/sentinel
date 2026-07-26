# Sentinel Security References

Version: 0.1

Status: Draft

---

# Overview

This document defines the security references used by Sentinel.

References provide the foundation for:

- Testing methodologies.
- Security terminology.
- Vulnerability classification.
- Verification criteria.
- Reporting standards.

Sentinel should prefer authoritative and publicly available security resources.

---

# Reference Principles

## 1. Source Reliability

References should prioritize:

- Official organizations.
- Industry standards.
- Maintained security projects.
- Public security research.

---

## 2. Version Tracking

Security references change over time.

Each reference should track:

- Name.
- Version.
- Release date.
- Usage inside Sentinel.

---

## 3. Contextual Usage

References should not be copied blindly.

Each reference should be evaluated for:

- What problem it solves.
- How it fits Sentinel workflows.
- What limitations exist.

---

# Core References

---

# OWASP Web Security Testing Guide (WSTG)

## Purpose

Provides a structured methodology for testing web applications.

---

## Used For

- Web security workflows.
- Testing activities.
- Assessment structure.

---

## Sentinel Usage

WSTG can provide:

- Testing categories.
- Activity definitions.
- Security testing guidance.

---

# OWASP Application Security Verification Standard (ASVS)

## Purpose

Defines security verification requirements for applications.

---

## Used For

- Security requirements.
- Verification criteria.
- Secure development checks.

---

## Sentinel Usage

ASVS can support:

- Finding validation.
- Security requirement mapping.
- Report references.

---

# Common Weakness Enumeration (CWE)

## Purpose

Provides a classification system for software weaknesses.

---

## Used For

- Vulnerability categorization.
- Finding classification.
- Reporting consistency.

---

## Sentinel Usage

Findings may reference related CWE categories.

---

# Common Vulnerability Scoring System (CVSS)

## Purpose

Provides a standardized way to describe vulnerability severity.

---

## Used For

- Severity explanation.
- Risk communication.

---

## Sentinel Usage

CVSS may support reporting decisions.

It should not replace researcher judgment.

---

# CAPEC

## Purpose

Provides a catalog of attack patterns.

---

## Used For

- Understanding attacker behavior.
- Connecting weaknesses with attack approaches.

---

# MITRE ATT&CK

## Purpose

Provides a knowledge base of adversary tactics and techniques.

---

## Used For

Future integrations and broader security context.

---

# Reference Categories

Sentinel references can be grouped into:

```
Methodology

    ↓

Testing Standards

    ↓

Weakness Classification

    ↓

Risk Scoring

    ↓

Attack Knowledge
```

---

# Reference Management Rules

## Rule 1

Every security activity should have supporting references when possible.

---

## Rule 2

References should explain why a test exists, not only provide a name.

---

## Rule 3

References should not automatically determine vulnerability status.

---

## Rule 4

Deprecated references should be archived, not silently removed.

---

# Future References

Possible future additions:

- Mobile security standards.
- API security standards.
- Cloud security references.
- Game security research.
- Industry-specific standards.

---

# Summary

References provide the knowledge foundation of Sentinel.

They allow the system to transform trusted security knowledge into structured workflows while keeping verification and decisions transparent.
