# Sentinel State Machine

Version: 0.1

Status: Draft

---

# Overview

The State Machine defines how Sentinel tracks the lifecycle of different entities during a security assessment.

A state represents the current condition of an entity.

A transition represents a valid movement from one state to another.

The State Machine helps Sentinel maintain:

- Assessment progress.
- Workflow execution.
- Activity status.
- Finding verification status.
- Evidence lifecycle.

---

# General State Principles

## 1. Every State Has Meaning

A state should represent a real condition, not a temporary UI status.

---

## 2. Invalid Transitions Are Prevented

The system should prevent impossible states.

Example:

A finding should not become "Verified" without evidence.

---

## 3. Human Decisions Are Supported

Some transitions require researcher approval.

Example:

Potential Issue

↓

Verified Finding

---

# Assessment State Machine

## States

```
Draft

↓

Active

↓

Paused

↓

Completed

↓

Archived
```

---

## State Definitions

### Draft

The assessment exists but testing has not started.

---

### Active

The assessment is currently being executed.

---

### Paused

The assessment is temporarily stopped.

Reasons:

- Waiting for information.
- Scope changes.
- Researcher decision.

---

### Completed

All required assessment activities are finished.

---

### Archived

The assessment is stored for future reference.

---

# Assessment Transitions

```
Draft

start

↓

Active


Active

pause

↓

Paused


Paused

resume

↓

Active


Active

complete

↓

Completed


Completed

archive

↓

Archived
```

---

# Workflow State Machine

## States

```
Not Started

↓

Running

↓

Waiting

↓

Completed
```

---

## State Definitions

### Not Started

Workflow has been assigned but execution has not begun.

---

### Running

Activities are currently being executed.

---

### Waiting

Workflow requires:

- User input.
- Manual verification.
- External information.

---

### Completed

All required workflow activities are finished.

---

# Activity State Machine

## States

```
Pending

↓

In Progress

↓

Review Required

↓

Completed
```

Possible alternative:

```
Pending

↓

Skipped
```

---

## State Definitions

### Pending

Activity is available but not started.

---

### In Progress

Researcher is currently performing the activity.

---

### Review Required

Activity produced information that needs analysis.

---

### Completed

Activity requirements have been fulfilled.

---

### Skipped

Activity was intentionally not performed.

A reason should be recorded.

---

# Finding State Machine

## States

```
Potential

↓

Under Verification

↓

Verified

or

Rejected
```

---

## State Definitions

### Potential

An interesting behavior or possible issue exists.

Not a confirmed vulnerability.

---

### Under Verification

The researcher is collecting information to confirm impact.

---

### Verified

The finding has enough evidence and validation.

---

### Rejected

The behavior was reviewed and determined not to be a security issue.

---

# Evidence State Machine

## States

```
Collected

↓

Reviewed

↓

Linked
```

---

## State Definitions

### Collected

Evidence was captured.

---

### Reviewed

Evidence quality and context were checked.

---

### Linked

Evidence is associated with an activity or finding.

---

# Transition Rules

## Rule 1

A Finding cannot become Verified without evidence.

---

## Rule 2

An Activity cannot become Completed without meeting its requirements.

---

## Rule 3

Skipped activities require a reason.

---

## Rule 4

State history should be preserved.

Example:

```
Pending

↓

In Progress

↓

Review Required

↓

Completed
```

The previous states should remain traceable.

---

# Future Considerations

Future versions may add:

- Custom states.
- Workflow branching.
- Team approval states.
- Review pipelines.
- Automated state transitions.

---

# Summary

The Sentinel State Machine provides a controlled way to track:

Assessment progress.

Workflow execution.

Activity completion.

Finding verification.

Evidence lifecycle.

It ensures that security results move through a transparent and reviewable process.
