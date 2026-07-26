use crate::reports::report::{ReportData, ReportExporter};

pub struct MarkdownExporter;
pub struct HtmlExporter;
pub struct JsonExporter;

impl ReportExporter for MarkdownExporter {
    fn name(&self) -> &'static str {
        "Markdown"
    }
    fn extension(&self) -> &'static str {
        "md"
    }
    fn export(&self, data: &ReportData) -> anyhow::Result<String> {
        Ok(render_markdown(data))
    }
}

impl ReportExporter for HtmlExporter {
    fn name(&self) -> &'static str {
        "HTML"
    }
    fn extension(&self) -> &'static str {
        "html"
    }
    fn export(&self, data: &ReportData) -> anyhow::Result<String> {
        Ok(render_html(data))
    }
}

impl ReportExporter for JsonExporter {
    fn name(&self) -> &'static str {
        "JSON"
    }
    fn extension(&self) -> &'static str {
        "json"
    }
    fn export(&self, data: &ReportData) -> anyhow::Result<String> {
        serde_json::to_string_pretty(data).map_err(Into::into)
    }
}

fn render_markdown(d: &ReportData) -> String {
    let mut s = String::new();
    s.push_str(&format!(
        "# Assessment Report: {}\n\n---\n\n",
        d.assessment.name
    ));
    s.push_str("## 1. Assessment Information\n\n| Field | Value |\n|-------|-------|\n");
    s.push_str(&format!("| Name | {} |\n", d.assessment.name));
    s.push_str(&format!("| Target | {} |\n", d.assessment.target));
    s.push_str(&format!("| Environment | {} |\n", d.assessment.environment));
    s.push_str(&format!("| Scope | {} |\n", d.assessment.scope));
    s.push_str(&format!("| Methodology | {} |\n", d.assessment.methodology));
    s.push_str(&format!("| Status | {} |\n", d.assessment.status));
    s.push_str(&format!("| Created | {} |\n", d.assessment.created_at));
    s.push_str(&format!(
        "| Completed | {} |\n\n",
        d.assessment.completed_at
    ));

    s.push_str("## 2. Executive Summary\n\n");
    s.push_str(&format!(
        "- **Total Sections**: {}\n",
        d.executive_summary.total_sections
    ));
    s.push_str(&format!(
        "- **Total Activities**: {}\n",
        d.executive_summary.total_activities
    ));
    s.push_str(&format!(
        "- **Total Tasks**: {}\n",
        d.executive_summary.total_tasks
    ));
    s.push_str(&format!(
        "- **Completed Tasks**: {}\n",
        d.executive_summary.completed_tasks
    ));
    s.push_str(&format!(
        "- **Skipped Tasks**: {}\n",
        d.executive_summary.skipped_tasks
    ));
    s.push_str(&format!(
        "- **In Progress Tasks**: {}\n",
        d.executive_summary.in_progress_tasks
    ));
    s.push_str(&format!(
        "- **Total Evidence**: {}\n",
        d.executive_summary.total_evidence
    ));
    s.push_str(&format!(
        "- **Total Findings**: {}\n\n",
        d.executive_summary.total_findings
    ));

    s.push_str("## 3. Workflow Summary\n\n");
    for sec in &d.workflow {
        s.push_str(&format!(
            "### {}\n\nProgress: {}/{} tasks completed\n\n",
            sec.title, sec.completed, sec.total
        ));
        for act in &sec.activities {
            s.push_str(&format!(
                "- **{}** ({}/{})\n",
                act.title, act.completed, act.total
            ));
            for t in &act.tasks {
                let icon = match t.status.as_str() {
                    "Completed" => "\u{2713} ",
                    "In Progress" => "\u{25CF} ",
                    "Skipped" => "\u{2298} ",
                    _ => "\u{25CB} ",
                };
                s.push_str(&format!("  - {}{} ({})\n", icon, t.title, t.status));
            }
        }
        s.push('\n');
    }

    s.push_str("## 4. Findings\n\n");
    if d.findings.is_empty() {
        s.push_str("*No findings recorded.*\n\n");
    } else {
        for f in &d.findings {
            s.push_str(&format!(
                "### {} - {} - {}\n\n",
                f.title, f.severity, f.confidence
            ));
            s.push_str(&format!(
                "**Severity**: {} | **Confidence**: {} | **Status**: {}\n\n",
                f.severity, f.confidence, f.status
            ));
            if !f.description.is_empty() {
                s.push_str(&format!("**Description**: {}\n\n", f.description));
            }
            if !f.impact.is_empty() {
                s.push_str(&format!("**Impact**: {}\n\n", f.impact));
            }
            if !f.recommendation.is_empty() {
                s.push_str(&format!("**Recommendation**: {}\n\n", f.recommendation));
            }
            if !f.references.is_empty() {
                s.push_str(&format!("**References**: {}\n\n", f.references));
            }
            if !f.linked_evidence.is_empty() {
                s.push_str("**Linked Evidence**:\n\n");
                for ev in &f.linked_evidence {
                    s.push_str(&format!("- *{}*: {}\n", ev.evidence_type, ev.title));
                }
                s.push('\n');
            }
            s.push_str("---\n\n");
        }
    }

    s.push_str("## 5. Evidence Summary\n\n");
    s.push_str(&format!(
        "**Total Evidence Items**: {}\n\n",
        d.evidence.total
    ));
    s.push_str("### By Type\n\n");
    for et in &d.evidence.by_type {
        s.push_str(&format!("- {}: {}\n", et.evidence_type, et.count));
    }
    s.push_str("\n### By Task\n\n");
    for tg in &d.evidence.by_task {
        s.push_str(&format!("- {}: {}\n", tg.task_title, tg.count));
    }
    s.push('\n');
    s.push_str("## 6. Metadata\n\n");
    s.push_str(&format!("- **Generated**: {}\n", d.metadata.generated_at));
    s.push_str(&format!("- **Sentinel Version**: {}\n", d.metadata.version));
    s
}

fn render_html(d: &ReportData) -> String {
    let mut s = String::new();
    s.push_str("<html><head><meta charset=\"utf-8\"><title>Sentinel Report</title>");
    s.push_str("<style>body{font-family:Arial,sans-serif;max-width:900px;margin:0 auto;padding:20px;color:#333}");
    s.push_str(
        "h1{color:#1a1a2e}h2{border-bottom:2px solid #2563eb;padding-bottom:4px;margin-top:30px}",
    );
    s.push_str("h3{color:#2563eb}table{border-collapse:collapse;width:100%}td,th{border:1px solid #ddd;padding:8px}th{background:#f0f0f0}");
    s.push_str(
        ".s-Critical{background:#fee;border-left:4px solid #dc2626;padding:10px;margin:10px 0}",
    );
    s.push_str(
        ".s-High{background:#fff3e0;border-left:4px solid #ea580c;padding:10px;margin:10px 0}",
    );
    s.push_str(
        ".s-Medium{background:#ffc;border-left:4px solid #ca8a04;padding:10px;margin:10px 0}",
    );
    s.push_str(
        ".s-Low{background:#e8f5e9;border-left:4px solid #16a34a;padding:10px;margin:10px 0}",
    );
    s.push_str(".s-Informational{background:#e3f2fd;border-left:4px solid #2563eb;padding:10px;margin:10px 0}");
    s.push_str("</style></head><body>");
    s.push_str(&format!(
        "<h1>Assessment Report: {}</h1>",
        d.assessment.name
    ));
    s.push_str("<h2>1. Assessment Information</h2><table>");
    s.push_str(&format!(
        "<tr><td>Name</td><td>{}</td></tr>",
        d.assessment.name
    ));
    s.push_str(&format!(
        "<tr><td>Target</td><td>{}</td></tr>",
        d.assessment.target
    ));
    s.push_str(&format!(
        "<tr><td>Environment</td><td>{}</td></tr>",
        d.assessment.environment
    ));
    s.push_str(&format!(
        "<tr><td>Scope</td><td>{}</td></tr>",
        d.assessment.scope
    ));
    s.push_str(&format!(
        "<tr><td>Methodology</td><td>{}</td></tr>",
        d.assessment.methodology
    ));
    s.push_str(&format!(
        "<tr><td>Status</td><td>{}</td></tr>",
        d.assessment.status
    ));
    s.push_str(&format!(
        "<tr><td>Created</td><td>{}</td></tr>",
        d.assessment.created_at
    ));
    s.push_str(&format!(
        "<tr><td>Completed</td><td>{}</td></tr>",
        d.assessment.completed_at
    ));
    s.push_str("</table>");
    s.push_str("<h2>2. Executive Summary</h2><ul>");
    s.push_str(&format!(
        "<li>Total Sections: {}</li>",
        d.executive_summary.total_sections
    ));
    s.push_str(&format!(
        "<li>Total Activities: {}</li>",
        d.executive_summary.total_activities
    ));
    s.push_str(&format!(
        "<li>Total Tasks: {}</li>",
        d.executive_summary.total_tasks
    ));
    s.push_str(&format!(
        "<li>Completed: {}</li>",
        d.executive_summary.completed_tasks
    ));
    s.push_str(&format!(
        "<li>Skipped: {}</li>",
        d.executive_summary.skipped_tasks
    ));
    s.push_str(&format!(
        "<li>In Progress: {}</li>",
        d.executive_summary.in_progress_tasks
    ));
    s.push_str(&format!(
        "<li>Total Evidence: {}</li>",
        d.executive_summary.total_evidence
    ));
    s.push_str(&format!(
        "<li>Total Findings: {}</li>",
        d.executive_summary.total_findings
    ));
    s.push_str("</ul>");
    s.push_str("<h2>3. Workflow Summary</h2>");
    for sec in &d.workflow {
        s.push_str(&format!(
            "<h3>{}</h3><p>Progress: {}/{}</p><ul>",
            sec.title, sec.completed, sec.total
        ));
        for act in &sec.activities {
            s.push_str(&format!(
                "<li><strong>{}</strong> ({}/{})<ul>",
                act.title, act.completed, act.total
            ));
            for t in &act.tasks {
                s.push_str(&format!("<li>{} ({})</li>", t.title, t.status));
            }
            s.push_str("</ul></li>");
        }
        s.push_str("</ul>");
    }
    s.push_str("<h2>4. Findings</h2>");
    if d.findings.is_empty() {
        s.push_str("<p>No findings recorded.</p>");
    }
    for f in &d.findings {
        let cls = format!("s-{}", f.severity);
        s.push_str(&format!(
            "<div class=\"{}\"><h3>{} - {} - {}</h3>",
            cls, f.title, f.severity, f.confidence
        ));
        s.push_str(&format!("<p><strong>Severity:</strong> {} | <strong>Confidence:</strong> {} | <strong>Status:</strong> {}</p>", f.severity, f.confidence, f.status));
        if !f.description.is_empty() {
            s.push_str(&format!(
                "<p><strong>Description:</strong> {}</p>",
                f.description
            ));
        }
        if !f.impact.is_empty() {
            s.push_str(&format!("<p><strong>Impact:</strong> {}</p>", f.impact));
        }
        if !f.recommendation.is_empty() {
            s.push_str(&format!(
                "<p><strong>Recommendation:</strong> {}</p>",
                f.recommendation
            ));
        }
        if !f.linked_evidence.is_empty() {
            s.push_str("<p><strong>Linked Evidence:</strong></p><ul>");
            for ev in &f.linked_evidence {
                s.push_str(&format!("<li>{}</li>", ev.title));
            }
            s.push_str("</ul>");
        }
        s.push_str("</div>");
    }
    s.push_str("<h2>5. Evidence Summary</h2>");
    s.push_str(&format!(
        "<p>Total: {}</p><h3>By Type</h3><ul>",
        d.evidence.total
    ));
    for et in &d.evidence.by_type {
        s.push_str(&format!("<li>{}: {}</li>", et.evidence_type, et.count));
    }
    s.push_str("</ul><h3>By Task</h3><ul>");
    for tg in &d.evidence.by_task {
        s.push_str(&format!("<li>{}: {}</li>", tg.task_title, tg.count));
    }
    s.push_str("</ul>");
    s.push_str("<h2>6. Metadata</h2><ul>");
    s.push_str(&format!("<li>Generated: {}</li>", d.metadata.generated_at));
    s.push_str(&format!("<li>Sentinel v{}</li>", d.metadata.version));
    s.push_str("</ul></body></html>");
    s
}
