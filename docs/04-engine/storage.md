# Sentinel Storage Layer

Version: 0.1

Status: Draft

---

# Overview

The Storage Layer is responsible for preserving Sentinel data.

It provides persistence for:

- Assessments.
- Targets.
- Workflows.
- Activities.
- Evidence.
- Findings.
- Reports.

The Storage Layer should be independent from the rest of the system.

---

# Storage Principles

---

# 1. Data Persistence

Important assessment information should survive:

- Application restarts.
- System updates.
- Workflow interruptions.

---

# 2. Separation From Business Logic

Core Sentinel logic should not depend on how data is stored.

Example:

The Assessment Core should not know whether data is stored in:

- Local files.
- Database.
- Remote storage.

---

# 3. Traceability

Stored data should preserve:

- History.
- Relationships.
- Assessment context.

---

# 4. Portability

A Sentinel assessment should be portable between environments.

---

# Stored Entities

---

# Assessment Data

Contains:

- Assessment information.
- Status.
- Creation time.
- Configuration.

---

# Target Data

Contains:

- Target information.
- Scope information.
- Related assessment.

---

# Workflow Data

Contains:

- Selected workflow.
- Progress.
- Activity states.

---

# Evidence Data

Contains:

- Evidence metadata.
- Content references.
- Relationships.

---

# Finding Data

Contains:

- Finding details.
- Verification status.
- Severity.
- Confidence.
- References.

---

# Report Data

Contains:

- Generated report information.
- Export history.
- Version information.

---

# Storage Options

---

# Initial Approach

The first version should prioritize simplicity.

Possible options:

- Local project storage.
- Structured files.
- Embedded database.

---

# Future Approach

Future versions may support:

- External databases.
- Team storage.
- Cloud synchronization.
- Collaboration features.

---

# Storage Structure Example

A Sentinel project may contain:

```
assessment/

├── config/
│
├── workflows/
│
├── evidence/
│
├── findings/
│
└── reports/
```

---

# Data Integrity

The Storage Layer should support:

- Validation.
- Version tracking.
- Recovery from failures.

---

# Sensitive Data Considerations

Security assessment data may contain sensitive information.

Sentinel should consider:

- Secure local storage.
- Access permissions.
- Safe export handling.

---

# Storage Rules

## Rule 1

Storage should not define security decisions.

---

## Rule 2

Stored data should maintain relationships between:

Activity

↓

Evidence

↓

Finding

---

## Rule 3

Deleting important assessment data should require intentional action.

---

# Future Storage Features

Possible improvements:

- Encryption.
- Team collaboration.
- Assessment synchronization.
- Search indexing.
- Backup support.

---

# Summary

The Storage Layer provides reliable persistence while keeping Sentinel architecture flexible.

It preserves the history and evidence behind security assessments without controlling security decisions.
