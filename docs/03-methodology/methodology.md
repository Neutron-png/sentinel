# Sentinel Security Methodology

Version: 0.1

Status: Draft

---

# Overview

Sentinel is built around a methodology-driven approach to security assessment.

The purpose of methodology is to provide structure, consistency, and repeatability during security testing.

Sentinel does not define security testing as a collection of isolated checks.

Instead, it treats security assessment as a process:

```
Understand

↓

Test

↓

Collect Evidence

↓

Verify

↓

Document
```

---

# Methodology Principles

---

# 1. Structured Testing

Security assessments should follow an organized process.

A researcher should understand:

- What area is being tested.
- Why it matters.
- What information is required.
- What evidence is expected.

Sentinel uses workflows to transform security methodologies into actionable assessment paths.

---

# 2. Evidence-Based Assessment

Security conclusions should be supported by evidence.

Evidence provides:

- Context.
- Reproducibility.
- Review capability.

A statement without evidence should not be treated as a confirmed result.

---

# 3. Verification Before Classification

Sentinel separates:

## Observation

Something unusual was noticed.

↓

## Investigation

Additional information is collected.

↓

## Verification

The behavior is confirmed or rejected.

↓

## Finding

A documented security issue exists.

---

# 4. Human Decision Involvement

Security testing requires judgment.

Sentinel supports researchers by providing:

- Structure.
- Context.
- References.
- Tracking.

However, final security decisions remain with the researcher.

---

# 5. Reference-Driven Testing

Testing activities should be connected to established security knowledge.

Examples:

- Security testing guides.
- Industry standards.
- Vulnerability classifications.
- Official documentation.

References help explain:

- Why a test exists.
- What risk it addresses.
- How results should be interpreted.

---

# Security Testing Model

Sentinel follows this general model:

```
Methodology

        ↓

Workflow

        ↓

Activity

        ↓

Evidence

        ↓

Verification

        ↓

Finding

        ↓

Report
```

---

# Testing Activity Model

Each testing activity should define:

## Objective

What security question is being answered?

Example:

"Is access control enforced correctly?"

---

## Context

Why is this activity important?

---

## Requirements

What information or access is needed?

---

## Expected Evidence

What information supports the result?

---

## Verification Criteria

What determines whether the result is valid?

---

# Methodology Sources

Sentinel may use knowledge from:

- OWASP testing methodologies.
- Security standards.
- Public vulnerability references.
- Official technical documentation.

References should be tracked and versioned.

---

# What Sentinel Avoids

Sentinel avoids:

- Blind testing.
- Unexplained automation.
- Treating every detection as a vulnerability.
- Replacing methodology with random checks.

---

# Initial Methodology Focus

The first Sentinel methodology focus is:

## Web Application Security Testing

Because web applications:

- Are widely used.
- Have mature testing methodologies.
- Provide a strong foundation for the workflow engine.

Future methodology support may include:

- API Security.
- Mobile Security.
- Game Security.

---

# Methodology Summary

Sentinel transforms security knowledge into structured workflows.

It connects:

Security Knowledge

↓

Testing Activities

↓

Evidence

↓

Verification

↓

Security Reports

The goal is not to automate security judgment.

The goal is to make security assessments more organized, repeatable, and reliable.
