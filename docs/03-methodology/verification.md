# Sentinel Verification Model

Version: 0.1

Status: Draft

---

# Overview

Verification is the process of determining whether an observed behavior represents a valid security issue.

Sentinel separates:

- Observation.
- Investigation.
- Verification.
- Reporting.

This separation helps reduce false positives and improves assessment quality.

---

# Verification Philosophy

A security result should not be considered confirmed only because:

- A tool detected something.
- A pattern matched.
- A response looked unusual.
- A rule was triggered.

A confirmed result requires:

- Context.
- Evidence.
- Understanding of impact.
- Researcher validation.

---

# Verification Lifecycle

```
Observation

    ↓

Investigation

    ↓

Evidence Collection

    ↓

Verification

    ↓

Finding

    ↓

Report
```

---

# Verification States

---

# 1. Observation

## Definition

An event or behavior that requires attention.

---

## Examples

- Unexpected response behavior.
- Unusual application logic.
- Suspicious configuration.

---

## Requirements

No security conclusion is made.

---

# 2. Investigation

## Definition

The researcher analyzes the behavior.

---

## Activities

- Collect additional information.
- Understand application behavior.
- Compare expected vs actual behavior.

---

# 3. Verification

## Definition

The researcher determines whether the behavior represents a security issue.

---

## Verification Questions

Examples:

- Does the behavior violate a security expectation?
- Is there a realistic impact?
- Can the behavior be reproduced?
- Is the evidence sufficient?

---

# 4. Verified Finding

## Definition

A confirmed security issue supported by evidence.

---

## Required Information

A verified finding should contain:

- Description.
- Impact.
- Evidence.
- Reproduction context.
- References.
- Recommendation.

---

# 5. Rejected Result

## Definition

A behavior was investigated but does not represent a security issue.

---

## Reasons

Examples:

- Expected behavior.
- Insufficient impact.
- Missing evidence.
- Incorrect assumption.

---

# Verification Requirements

A verification process should evaluate:

---

## Evidence Quality

Questions:

- Is the evidence relevant?
- Does it support the conclusion?
- Can another person understand it?

---

## Reproducibility

Questions:

- Can the behavior happen again?
- Are the required conditions documented?

---

## Impact

Questions:

- What security property is affected?
- Is there a meaningful consequence?

---

## Context

Questions:

- Is the behavior expected?
- Is it within the testing scope?

---

# Finding Confidence Levels

Sentinel may represent confidence separately from severity.

Example:

```
Confidence:

Low
Medium
High
Confirmed
```

---

# Severity vs Confidence

These concepts are different.

Severity:

"How bad is the impact?"

Confidence:

"How sure are we that this is real?"

Example:

A researcher may have:

High Severity

+

Low Confidence

This should not be reported as a confirmed critical issue.

---

# Verification Rules

## Rule 1

No finding without evidence.

---

## Rule 2

No automatic confirmation based only on detection.

---

## Rule 3

The researcher can override automated suggestions.

---

## Rule 4

All verification decisions should be traceable.

---

# Verification and Automation

Automation can assist with:

- Collecting information.
- Comparing results.
- Organizing evidence.
- Running predefined checks.

Automation should not independently decide:

- Final vulnerability status.
- Business impact.
- Risk acceptance.

---

# Future Verification Features

Possible future capabilities:

- Verification templates.
- Evidence requirements.
- Review workflows.
- Team approval processes.

---

# Summary

Verification is the bridge between finding interesting behavior and creating a reliable security finding.

Sentinel treats verification as a separate, explicit process to improve accuracy, transparency, and trust.
