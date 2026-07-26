# Sentinel Use Cases

Version: 0.1

Status: Draft

---

# Overview

This document describes the main scenarios where Sentinel is expected to provide value.

A use case describes:

- Who is using Sentinel.
- What they want to achieve.
- How Sentinel supports the process.
- What the expected outcome is.

Use cases are focused on user goals, not implementation details.

---

# UC-001: Create Security Assessment

## Actor

Security Researcher

---

## Goal

Create a structured workspace for a new penetration testing assessment.

---

## Preconditions

- Sentinel is installed.
- User has authorization to test the target.
- No existing assessment is required.

---

## Main Flow

1. User creates a new assessment.
2. User provides assessment information.
3. User defines the target scope.
4. Sentinel creates an assessment workspace.
5. Sentinel prepares available workflows.

---

## Expected Result

A new assessment exists and is ready for testing.

---

# UC-002: Add Target

## Actor

Security Researcher

---

## Goal

Add an application target to an assessment.

---

## Preconditions

- An assessment exists.
- User has permission to test the target.

---

## Main Flow

1. User adds a target.
2. User provides target information.
3. Sentinel stores target information.
4. Target becomes available for workflows.

---

## Expected Result

The target is connected to the assessment.

---

# UC-003: Start Testing Workflow

## Actor

Security Researcher

---

## Goal

Begin a structured security assessment.

---

## Preconditions

- Assessment exists.
- Target exists.
- A suitable workflow is available.

---

## Main Flow

1. User selects a workflow.
2. Sentinel displays the assessment structure.
3. Sentinel starts the first testing activity.
4. User follows the workflow.
5. Progress is tracked.

---

## Expected Result

The assessment has an active workflow.

---

# UC-004: Execute Testing Activity

## Actor

Security Researcher

---

## Goal

Perform a security testing activity.

---

## Preconditions

- A workflow is running.
- Required information is available.

---

## Main Flow

1. Sentinel presents the activity.
2. Sentinel explains the purpose.
3. User performs the required action.
4. Results are recorded.
5. Evidence can be attached.

---

## Expected Result

The activity receives a final state.

Possible outcomes:

- Completed.
- Failed.
- Skipped.
- Requires verification.
- Requires manual review.

---

# UC-005: Collect Evidence

## Actor

Security Researcher

---

## Goal

Store information that supports testing results.

---

## Preconditions

- An assessment exists.
- A testing activity exists.

---

## Main Flow

1. User attaches evidence.
2. Sentinel associates evidence with the activity.
3. Evidence becomes available for review.

---

## Expected Result

Evidence is stored with proper context.

---

# UC-006: Verify Security Finding

## Actor

Security Researcher

---

## Goal

Determine whether a potential issue is a confirmed security finding.

---

## Preconditions

- A potential issue exists.
- Required evidence is available.

---

## Main Flow

1. Sentinel presents verification requirements.
2. User performs verification.
3. Evidence is reviewed.
4. Finding receives a verification status.

---

## Expected Result

The result becomes one of:

- Verified.
- Not Verified.
- Requires More Investigation.

---

# UC-007: Generate Security Report

## Actor

Security Researcher

---

## Goal

Create a professional security assessment report.

---

## Preconditions

- Assessment contains testing information.
- Findings and evidence exist.

---

## Main Flow

1. User requests a report.
2. Sentinel collects assessment information.
3. Sentinel organizes findings.
4. Sentinel generates a report.

---

## Expected Result

A structured report is created.

---

# UC-008: Resume Previous Assessment

## Actor

Security Researcher

---

## Goal

Continue an assessment without losing previous progress.

---

## Preconditions

- Previous assessment exists.

---

## Main Flow

1. User opens an existing assessment.
2. Sentinel restores the previous state.
3. User continues from the last point.

---

## Expected Result

The assessment continues with previous context.

---

# UC-009: Review Assessment History

## Actor

Security Team Reviewer

---

## Goal

Understand what was tested and how results were reached.

---

## Preconditions

- Assessment data exists.

---

## Main Flow

1. Reviewer opens assessment history.
2. Reviewer reviews completed activities.
3. Reviewer reviews evidence and findings.

---

## Expected Result

The reviewer understands assessment coverage and results.

---

# Future Use Cases

The following are possible future scenarios:

- API security assessments.
- Mobile application assessments.
- Team collaboration.
- Custom workflows.
- Integration with external security tools.

These are not part of the initial release scope.

---

# Summary

The initial Sentinel experience focuses on:

Create Assessment

↓

Define Target

↓

Run Workflow

↓

Perform Testing

↓

Collect Evidence

↓

Verify Results

↓

Generate Report
