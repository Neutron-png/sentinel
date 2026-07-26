# Sentinel Architecture

Version: 0.1

Status: Draft

---

# Overview

Sentinel is designed as a modular security assessment framework.

The architecture separates:

- User interaction.
- Assessment workflow.
- Security knowledge.
- Testing execution.
- Evidence management.
- Verification.
- Reporting.
- Data storage.

Each component has a clear responsibility to keep the system maintainable and extensible.

---

# Architectural Principles

## 1. Separation of Responsibilities

Each component should have one primary responsibility.

Examples:

The Workflow Engine manages execution flow.

The Evidence Engine manages evidence.

The Reporting Engine creates reports.

No component should own unrelated responsibilities.

---

## 2. Human Controlled Execution

Sentinel should always keep the researcher in control.

The architecture must support:

- Manual actions.
- User decisions.
- Approval points.
- Review stages.

---

## 3. Evidence Driven Design

Security results should be built around collected evidence.

Evidence should not be an afterthought.

---

## 4. Extensibility

The architecture should allow future support for:

- New testing methodologies.
- New assessment types.
- New integrations.
- Custom workflows.

---

# High-Level Architecture

```
                    User
                     |
                     v
                CLI Interface
                     |
                     v
              Assessment Core
                     |
        +------------+-------------+
        |            |             |
        v            v             v

 Workflow Engine  Knowledge     Evidence
                  Engine        Engine

        |            |             |
        v            v             v

 Testing        Methodology    Evidence
 Actions        References     Storage

                     |
                     v

              Verification Engine

                     |
                     v

             Reporting Engine

                     |
                     v

                Storage Layer
```

---

# Core Components

---

# 1. CLI Interface

## Responsibility

The entry point for users interacting with Sentinel.

## Handles

- Commands.
- Input.
- Output.
- User confirmations.
- Progress display.

## Does Not Handle

- Security logic.
- Workflow decisions.
- Evidence processing.

---

# 2. Assessment Core

## Responsibility

The central coordinator of an assessment.

## Handles

- Project lifecycle.
- Target association.
- Assessment state.
- Communication between components.

## Does Not Handle

- Specific security tests.
- Report formatting.
- Data persistence details.

---

# 3. Workflow Engine

## Responsibility

Controls the execution of security assessment workflows.

## Handles

- Workflow order.
- Progress tracking.
- Task states.
- User interaction points.

## Does Not Handle

- Security knowledge definition.
- Report generation.

---

# 4. Knowledge Engine

## Responsibility

Provides security methodology and testing knowledge.

## Contains

- Testing references.
- Methodologies.
- Testing activities.
- Requirements.

## Does Not Handle

- Executing tests.
- Making final vulnerability decisions.

---

# 5. Testing Layer

## Responsibility

Represents testing activities.

Testing activities can be:

- Automated.
- Interactive.
- Manual.

The testing layer should provide results, not final conclusions.

---

# 6. Evidence Engine

## Responsibility

Manages security assessment evidence.

## Handles

- Evidence collection.
- Evidence organization.
- Evidence relationships.

## Examples

- Requests.
- Responses.
- Screenshots.
- Logs.
- Notes.

---

# 7. Verification Engine

## Responsibility

Determines whether collected information supports a security conclusion.

## Handles

- Verification requirements.
- Result validation.
- Finding status.

Possible outcomes:

- Verified.
- Not Verified.
- Requires Review.

---

# 8. Reporting Engine

## Responsibility

Transforms assessment data into security reports.

## Handles

- Report structure.
- Finding presentation.
- Evidence attachment.
- Export formats.

---

# 9. Storage Layer

## Responsibility

Provides data persistence.

Stores:

- Assessments.
- Targets.
- Workflows.
- Evidence.
- Findings.
- Reports.

The storage implementation should not affect the business logic.

---

# Component Communication

The preferred communication direction:

```
CLI

↓

Assessment Core

↓

Domain Components

↓

Storage
```

Components should communicate through defined interfaces.

---

# Future Architecture Considerations

The architecture should allow future support for:

- Plugin system.
- External tool integrations.
- Multiple interfaces.
- Team collaboration.
- Different security domains.

---

# Architecture Summary

Sentinel is not designed as a collection of security checks.

It is designed as a framework where:

Knowledge defines what should be done.

Workflow defines when it should be done.

Testing performs actions.

Evidence records results.

Verification validates conclusions.

Reporting communicates outcomes.
