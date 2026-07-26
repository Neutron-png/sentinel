# Sentinel CLI Design

Version: 0.1

Status: Draft

---

# Overview

The Sentinel CLI is the primary interface for interacting with the security assessment framework.

The CLI should provide a clear, structured, and transparent experience for researchers.

The CLI is responsible for:

- User interaction.
- Command execution.
- Input handling.
- Displaying assessment progress.

The CLI is not responsible for:

- Security logic.
- Workflow decisions.
- Verification decisions.
- Data processing.

---

# CLI Design Principles

---

# 1. Clarity Over Complexity

Commands should be easy to understand.

Users should not need to memorize complex commands.

---

# 2. Explain Actions

Sentinel should explain:

- What is happening.
- Why it is happening.
- What the next step is.

---

# 3. Human Control

Actions that affect assessment results should require appropriate user control.

---

# 4. Consistent Output

CLI output should follow a consistent format.

Examples:

- Status messages.
- Progress information.
- Warnings.
- Errors.

---

# Command Structure

General format:

```
sentinel <command> <subcommand> [options]
```

---

# Core Commands

---

# 1. init

## Purpose

Initialize a new Sentinel workspace.

Example:

```
sentinel init
```

Creates:

- Project configuration.
- Workspace structure.
- Local metadata.

---

# 2. assessment

## Purpose

Manage security assessments.

Examples:

```
sentinel assessment create

sentinel assessment list

sentinel assessment open
```

---

## Responsibilities

- Create assessments.
- View assessments.
- Resume previous work.

---

# 3. target

## Purpose

Manage assessment targets.

Examples:

```
sentinel target add

sentinel target list
```

---

## Responsibilities

- Add target information.
- View target scope.
- Associate targets with assessments.

---

# 4. workflow

## Purpose

Manage assessment workflows.

Examples:

```
sentinel workflow list

sentinel workflow start
```

---

## Responsibilities

- Select workflows.
- Start workflow execution.
- Display progress.

---

# 5. activity

## Purpose

Interact with testing activities.

Examples:

```
sentinel activity show

sentinel activity complete
```

---

## Responsibilities

- View current activity.
- Record activity results.
- Attach evidence.

---

# 6. evidence

## Purpose

Manage collected evidence.

Examples:

```
sentinel evidence add

sentinel evidence list
```

---

## Responsibilities

- Store evidence.
- Link evidence.
- Review evidence.

---

# 7. finding

## Purpose

Manage security findings.

Examples:

```
sentinel finding list

sentinel finding verify
```

---

## Responsibilities

- Review findings.
- Update verification state.
- Prepare reporting information.

---

# 8. report

## Purpose

Generate assessment reports.

Examples:

```
sentinel report generate
```

---

## Responsibilities

- Collect assessment data.
- Format results.
- Export reports.

---

# Example User Journey

```
Create Assessment

↓

sentinel assessment create


Add Target

↓

sentinel target add


Start Workflow

↓

sentinel workflow start


Perform Activities

↓

sentinel activity show


Collect Evidence

↓

sentinel evidence add


Generate Report

↓

sentinel report generate
```

---

# CLI Output Guidelines

---

# Success Messages

Should explain:

- What completed.
- What changed.

Example:

```
Assessment created successfully.
ID: assessment-001
```

---

# Warning Messages

Should explain:

- Why attention is required.
- Possible next actions.

---

# Error Messages

Should explain:

- What failed.
- Possible solutions.

Avoid:

```
Error: failed
```

Prefer:

```
Unable to load assessment.
The assessment file may be missing or corrupted.
```

---

# Future CLI Capabilities

Possible future improvements:

- Interactive mode.
- Command autocomplete.
- Multiple output formats.
- JSON output.
- External integrations.

---

# CLI Summary

The Sentinel CLI is the user's gateway into the assessment workflow.

It should provide:

- Control.
- Visibility.
- Context.
- Clear progress.

The CLI should make complex security workflows easier to follow without hiding the underlying process.
