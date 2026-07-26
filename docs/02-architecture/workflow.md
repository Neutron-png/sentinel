# Sentinel Workflow

Version: 0.1

Status: Draft

---

# Overview

A workflow in Sentinel represents a structured sequence of security assessment activities.

Workflows define:

- What should be tested.
- The order of activities.
- Required evidence.
- Verification points.
- Completion criteria.

A workflow does not replace researcher judgment.

It provides structure and guidance.

---

# Workflow Lifecycle

The general Sentinel workflow:

```
Create Assessment

        ↓

Define Scope

        ↓

Select Workflow

        ↓

Execute Activities

        ↓

Collect Evidence

        ↓

Review Results

        ↓

Verify Findings

        ↓

Generate Report
```

---

# 1. Create Assessment

## Purpose

Create a workspace for a security assessment.

---

## Inputs

- Assessment name.
- Description.
- Authorization information.
- Testing objectives.

---

## Output

A new Assessment entity.

---

# 2. Define Scope

## Purpose

Establish the boundaries of testing.

---

## Scope Includes

- Allowed targets.
- Excluded targets.
- Testing limitations.
- Assessment rules.

---

## Expected Result

The assessment has a clear testing boundary.

---

# 3. Select Workflow

## Purpose

Choose the methodology that will guide the assessment.

---

## Examples

- Web Application Security Workflow.
- API Security Workflow.
- Mobile Application Workflow.

---

## Output

A workflow instance attached to the assessment.

---

# 4. Execute Workflow Activities

## Purpose

Perform security testing activities.

---

Each activity contains:

- Objective.
- Description.
- Required knowledge.
- Expected evidence.
- Verification requirements.

---

## Activity Types

### Manual Activity

Requires researcher action.

Example:

```
Review authorization behavior.
```

---

### Interactive Activity

Requires user input with Sentinel assistance.

Example:

```
Capture request.

Modify parameter.

Compare response.
```

---

### Automated Activity

Can be executed automatically.

Example:

```
Check security header configuration.
```

---

# 5. Collect Evidence

## Purpose

Record information produced during testing.

---

Evidence may include:

- Requests.
- Responses.
- Screenshots.
- Logs.
- Notes.
- Test observations.

---

## Evidence Relationship

Evidence should always have context.

Example:

```
Evidence

belongs to

Activity

or

Finding
```

---

# 6. Review Results

## Purpose

Analyze the outcome of activities.

---

Possible activity outcomes:

```
Completed

Failed

Skipped

Requires Review

Potential Issue
```

---

# 7. Verification Process

## Purpose

Determine whether a potential issue is a confirmed security finding.

---

Verification flow:

```
Potential Issue

        ↓

Review Requirements

        ↓

Collect Additional Evidence

        ↓

Researcher Decision

        ↓

Verified Finding
```

---

# 8. Generate Report

## Purpose

Create a structured security assessment report.

---

Report contains:

- Assessment information.
- Scope.
- Methodology.
- Findings.
- Evidence.
- Recommendations.

---

# Workflow Rules

---

## Rule 1: No Direct Finding Creation

Activities produce results.

Results require verification before becoming findings.

---

## Rule 2: Every Activity Has a Purpose

An activity should explain:

- Why it exists.
- What it tests.
- What evidence is expected.

---

## Rule 3: Workflow Progress Must Be Resumable

A researcher should be able to stop and continue later without losing context.

---

## Rule 4: Manual Decision Points Are Supported

The workflow must allow:

- User approval.
- User decisions.
- Manual verification.

---

# Example Workflow

```
Web Application Assessment

        |

Information Gathering

        |

Authentication Testing

        |

Authorization Testing

        |

Session Testing

        |

Input Validation Testing

        |

Business Logic Testing

        |

Verification

        |

Reporting
```

---

# Future Workflow Capabilities

Possible future improvements:

- Custom workflows.
- Team workflows.
- Workflow sharing.
- Methodology packages.
- External tool integrations.

---

# Workflow Summary

Sentinel workflows transform security methodologies into structured assessment journeys.

They connect:

Knowledge

↓

Activities

↓

Evidence

↓

Verification

↓

Reports

while keeping the researcher responsible for final decisions.
