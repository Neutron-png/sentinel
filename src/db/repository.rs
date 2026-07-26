use std::str::FromStr;

use rusqlite::{params, Connection};
use uuid::Uuid;

use crate::history::models::HistoryEntry;
use crate::models::assessment::{Assessment, AssessmentStatus, Environment, Methodology, Scope};
use crate::models::evidence::{Evidence, EvidenceType};
use crate::models::finding::{Confidence, Finding, FindingStatus, Severity};
use crate::models::http::{HttpRequest, HttpResponse, HttpTransaction};
use crate::models::session::Session;
use crate::models::workflow::{Activity, Section, Task, TaskStatus};

pub struct Repository {
    conn: Connection,
}

#[allow(dead_code)]
impl Repository {
    pub fn open(path: &std::path::Path) -> anyhow::Result<Self> {
        let conn = crate::db::initialize(path)?;
        Ok(Self { conn })
    }

    // ── assessments ──

    pub fn create_assessment(&self, assessment: &Assessment) -> anyhow::Result<()> {
        self.conn.execute(
            "INSERT INTO assessments (id, name, target, environment, scope, methodology, status, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            params![
                assessment.id.to_string(),
                assessment.name,
                assessment.target,
                assessment.environment.to_string(),
                assessment.scope.to_string(),
                assessment.methodology.to_string(),
                assessment.status.to_string(),
                assessment.created_at.to_rfc3339(),
                assessment.updated_at.to_rfc3339(),
            ],
        )?;
        Ok(())
    }

    pub fn get_assessment(&self, id: &Uuid) -> anyhow::Result<Option<Assessment>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, name, target, environment, scope, methodology, status, created_at, updated_at FROM assessments WHERE id = ?1",
        )?;
        let mut rows = stmt.query(params![id.to_string()])?;
        match rows.next()? {
            Some(row) => Ok(Some(row_to_assessment(row)?)),
            None => Ok(None),
        }
    }

    pub fn list_assessments(&self) -> anyhow::Result<Vec<Assessment>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, name, target, environment, scope, methodology, status, created_at, updated_at FROM assessments ORDER BY created_at DESC",
        )?;
        let rows = stmt.query_map([], row_to_assessment)?;
        let mut assessments = Vec::new();
        for row in rows {
            assessments.push(row?);
        }
        Ok(assessments)
    }

    pub fn update_assessment(&self, assessment: &Assessment) -> anyhow::Result<()> {
        self.conn.execute(
            "UPDATE assessments SET name = ?2, target = ?3, environment = ?4, scope = ?5, methodology = ?6, status = ?7, updated_at = ?8 WHERE id = ?1",
            params![
                assessment.id.to_string(),
                assessment.name,
                assessment.target,
                assessment.environment.to_string(),
                assessment.scope.to_string(),
                assessment.methodology.to_string(),
                assessment.status.to_string(),
                assessment.updated_at.to_rfc3339(),
            ],
        )?;
        Ok(())
    }

    pub fn delete_assessment(&self, id: &Uuid) -> anyhow::Result<()> {
        self.conn.execute(
            "DELETE FROM assessments WHERE id = ?1",
            params![id.to_string()],
        )?;
        Ok(())
    }

    // ── sections ──

    pub fn load_sections(&self, assessment_id: &Uuid) -> anyhow::Result<Vec<Section>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, assessment_id, title, display_order FROM sections WHERE assessment_id = ?1 ORDER BY display_order",
        )?;
        let rows = stmt.query_map(params![assessment_id.to_string()], |row| {
            Ok(Section {
                id: Uuid::parse_str(&row.get::<_, String>("id")?).unwrap(),
                assessment_id: Uuid::parse_str(&row.get::<_, String>("assessment_id")?).unwrap(),
                title: row.get("title")?,
                display_order: row.get("display_order")?,
            })
        })?;
        let mut sections = Vec::new();
        for row in rows {
            sections.push(row?);
        }
        Ok(sections)
    }

    pub fn insert_section(&self, section: &Section) -> anyhow::Result<()> {
        self.conn.execute(
            "INSERT INTO sections (id, assessment_id, title, display_order) VALUES (?1, ?2, ?3, ?4)",
            params![
                section.id.to_string(),
                section.assessment_id.to_string(),
                section.title,
                section.display_order,
            ],
        )?;
        Ok(())
    }

    // ── activities ──

    pub fn load_activities(&self, section_id: &Uuid) -> anyhow::Result<Vec<Activity>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, section_id, title, description, display_order FROM activities WHERE section_id = ?1 ORDER BY display_order",
        )?;
        let rows = stmt.query_map(params![section_id.to_string()], |row| {
            Ok(Activity {
                id: Uuid::parse_str(&row.get::<_, String>("id")?).unwrap(),
                section_id: Uuid::parse_str(&row.get::<_, String>("section_id")?).unwrap(),
                title: row.get("title")?,
                description: row.get("description")?,
                display_order: row.get("display_order")?,
            })
        })?;
        let mut activities = Vec::new();
        for row in rows {
            activities.push(row?);
        }
        Ok(activities)
    }

    pub fn insert_activity(&self, activity: &Activity) -> anyhow::Result<()> {
        self.conn.execute(
            "INSERT INTO activities (id, section_id, title, description, display_order) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                activity.id.to_string(),
                activity.section_id.to_string(),
                activity.title,
                activity.description,
                activity.display_order,
            ],
        )?;
        Ok(())
    }

    // ── tasks ──

    pub fn load_tasks(&self, activity_id: &Uuid) -> anyhow::Result<Vec<Task>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, activity_id, title, description, status, notes, reference, display_order FROM tasks WHERE activity_id = ?1 ORDER BY display_order",
        )?;
        let rows = stmt.query_map(params![activity_id.to_string()], |row| {
            let st: String = row.get("status")?;
            Ok(Task {
                id: Uuid::parse_str(&row.get::<_, String>("id")?).unwrap(),
                activity_id: Uuid::parse_str(&row.get::<_, String>("activity_id")?).unwrap(),
                title: row.get("title")?,
                description: row.get("description")?,
                status: TaskStatus::from_str(&st).unwrap_or(TaskStatus::NotStarted),
                notes: row.get("notes")?,
                reference: row.get("reference")?,
                display_order: row.get("display_order")?,
            })
        })?;
        let mut tasks = Vec::new();
        for row in rows {
            tasks.push(row?);
        }
        Ok(tasks)
    }

    pub fn insert_task(&self, task: &Task) -> anyhow::Result<()> {
        self.conn.execute(
            "INSERT INTO tasks (id, activity_id, title, description, status, notes, reference, display_order) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            params![
                task.id.to_string(),
                task.activity_id.to_string(),
                task.title,
                task.description,
                task.status.to_string(),
                task.notes,
                task.reference,
                task.display_order,
            ],
        )?;
        Ok(())
    }

    pub fn update_task_status(&self, task_id: &Uuid, status: &TaskStatus) -> anyhow::Result<()> {
        self.conn.execute(
            "UPDATE tasks SET status = ?2 WHERE id = ?1",
            params![task_id.to_string(), status.to_string()],
        )?;
        Ok(())
    }

    pub fn has_workflow(&self, assessment_id: &Uuid) -> anyhow::Result<bool> {
        let count: i32 = self.conn.query_row(
            "SELECT COUNT(*) FROM sections WHERE assessment_id = ?1",
            params![assessment_id.to_string()],
            |row| row.get(0),
        )?;
        Ok(count > 0)
    }

    // ── evidence ──

    pub fn load_evidence(&self, task_id: &Uuid) -> anyhow::Result<Vec<Evidence>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, assessment_id, task_id, evidence_type, title, content, created_at, updated_at
             FROM evidence WHERE task_id = ?1 ORDER BY created_at DESC",
        )?;
        let rows = stmt.query_map(params![task_id.to_string()], row_to_evidence)?;
        let mut items = Vec::new();
        for row in rows {
            items.push(row?);
        }
        Ok(items)
    }

    pub fn load_evidence_counts(
        &self,
        assessment_id: &Uuid,
    ) -> anyhow::Result<std::collections::HashMap<Uuid, usize>> {
        let mut stmt = self.conn.prepare(
            "SELECT task_id, COUNT(*) FROM evidence WHERE assessment_id = ?1 GROUP BY task_id",
        )?;
        let rows = stmt.query_map(params![assessment_id.to_string()], |row| {
            let tid: String = row.get(0)?;
            let count: i64 = row.get(1)?;
            Ok((Uuid::parse_str(&tid).unwrap(), count as usize))
        })?;
        let mut map = std::collections::HashMap::new();
        for row in rows {
            let (tid, count) = row?;
            map.insert(tid, count);
        }
        Ok(map)
    }

    pub fn insert_evidence(&self, evidence: &Evidence) -> anyhow::Result<()> {
        self.conn.execute(
            "INSERT INTO evidence (id, assessment_id, task_id, evidence_type, title, content, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            params![
                evidence.id.to_string(),
                evidence.assessment_id.to_string(),
                evidence.task_id.to_string(),
                evidence.evidence_type.to_string(),
                evidence.title,
                evidence.content,
                evidence.created_at.to_rfc3339(),
                evidence.updated_at.to_rfc3339(),
            ],
        )?;
        Ok(())
    }

    pub fn update_evidence(&self, evidence: &Evidence) -> anyhow::Result<()> {
        self.conn.execute(
            "UPDATE evidence SET evidence_type = ?2, title = ?3, content = ?4, updated_at = ?5 WHERE id = ?1",
            params![
                evidence.id.to_string(),
                evidence.evidence_type.to_string(),
                evidence.title,
                evidence.content,
                evidence.updated_at.to_rfc3339(),
            ],
        )?;
        Ok(())
    }

    pub fn delete_evidence(&self, id: &Uuid) -> anyhow::Result<()> {
        self.conn.execute(
            "DELETE FROM evidence WHERE id = ?1",
            params![id.to_string()],
        )?;
        Ok(())
    }

    // ── findings ──

    pub fn load_findings(&self, task_id: &Uuid) -> anyhow::Result<Vec<Finding>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, assessment_id, task_id, title, description, severity, confidence, status, impact, recommendation, `references`, created_at, updated_at
             FROM findings WHERE task_id = ?1 ORDER BY created_at DESC",
        )?;
        let rows = stmt.query_map(params![task_id.to_string()], row_to_finding)?;
        let mut items = Vec::new();
        for row in rows {
            items.push(row?);
        }
        Ok(items)
    }

    pub fn insert_finding(&self, finding: &Finding) -> anyhow::Result<()> {
        self.conn.execute(
            "INSERT INTO findings (id, assessment_id, task_id, title, description, severity, confidence, status, impact, recommendation, `references`, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)",
            params![
                finding.id.to_string(),
                finding.assessment_id.to_string(),
                finding.task_id.to_string(),
                finding.title,
                finding.description,
                finding.severity.to_string(),
                finding.confidence.to_string(),
                finding.status.to_string(),
                finding.impact,
                finding.recommendation,
                finding.references,
                finding.created_at.to_rfc3339(),
                finding.updated_at.to_rfc3339(),
            ],
        )?;
        Ok(())
    }

    pub fn update_finding(&self, finding: &Finding) -> anyhow::Result<()> {
        self.conn.execute(
            "UPDATE findings SET title = ?2, description = ?3, severity = ?4, confidence = ?5, status = ?6, impact = ?7, recommendation = ?8, `references` = ?9, updated_at = ?10 WHERE id = ?1",
            params![
                finding.id.to_string(),
                finding.title,
                finding.description,
                finding.severity.to_string(),
                finding.confidence.to_string(),
                finding.status.to_string(),
                finding.impact,
                finding.recommendation,
                finding.references,
                finding.updated_at.to_rfc3339(),
            ],
        )?;
        Ok(())
    }

    pub fn delete_finding(&self, id: &Uuid) -> anyhow::Result<()> {
        self.conn.execute(
            "DELETE FROM finding_evidence WHERE finding_id = ?1",
            params![id.to_string()],
        )?;
        self.conn.execute(
            "DELETE FROM findings WHERE id = ?1",
            params![id.to_string()],
        )?;
        Ok(())
    }

    // ── finding-evidence links ──

    pub fn link_evidence(&self, finding_id: &Uuid, evidence_id: &Uuid) -> anyhow::Result<()> {
        self.conn.execute(
            "INSERT OR IGNORE INTO finding_evidence (finding_id, evidence_id) VALUES (?1, ?2)",
            params![finding_id.to_string(), evidence_id.to_string()],
        )?;
        Ok(())
    }

    pub fn unlink_evidence(&self, finding_id: &Uuid, evidence_id: &Uuid) -> anyhow::Result<()> {
        self.conn.execute(
            "DELETE FROM finding_evidence WHERE finding_id = ?1 AND evidence_id = ?2",
            params![finding_id.to_string(), evidence_id.to_string()],
        )?;
        Ok(())
    }

    pub fn load_linked_evidence_ids(&self, finding_id: &Uuid) -> anyhow::Result<Vec<Uuid>> {
        let mut stmt = self
            .conn
            .prepare("SELECT evidence_id FROM finding_evidence WHERE finding_id = ?1")?;
        let rows = stmt.query_map(params![finding_id.to_string()], |row| {
            let s: String = row.get(0)?;
            Ok(Uuid::parse_str(&s).unwrap())
        })?;
        let mut ids = Vec::new();
        for row in rows {
            ids.push(row?);
        }
        Ok(ids)
    }

    // ── sessions ──

    pub fn load_session(&self) -> anyhow::Result<Option<Session>> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, assessment_id, current_screen, last_opened_at FROM sessions ORDER BY last_opened_at DESC LIMIT 1")?;
        let mut rows = stmt.query([])?;
        match rows.next()? {
            Some(row) => Ok(Some(Session {
                id: Uuid::parse_str(&row.get::<_, String>("id")?).map_err(|e| {
                    rusqlite::Error::FromSqlConversionFailure(
                        0,
                        rusqlite::types::Type::Text,
                        Box::new(e),
                    )
                })?,
                assessment_id: Uuid::parse_str(&row.get::<_, String>("assessment_id")?).map_err(
                    |e| {
                        rusqlite::Error::FromSqlConversionFailure(
                            0,
                            rusqlite::types::Type::Text,
                            Box::new(e),
                        )
                    },
                )?,
                current_screen: row.get("current_screen")?,
                last_opened_at: chrono::DateTime::parse_from_rfc3339(
                    &row.get::<_, String>("last_opened_at")?,
                )
                .map_err(|e| {
                    rusqlite::Error::FromSqlConversionFailure(
                        0,
                        rusqlite::types::Type::Text,
                        Box::new(e),
                    )
                })?
                .with_timezone(&chrono::Utc),
            })),
            None => Ok(None),
        }
    }

    pub fn save_session(&self, session: &Session) -> anyhow::Result<()> {
        self.conn.execute(
            "INSERT OR REPLACE INTO sessions (id, assessment_id, current_screen, last_opened_at) VALUES (?1, ?2, ?3, ?4)",
            params![
                session.id.to_string(),
                session.assessment_id.to_string(),
                session.current_screen,
                session.last_opened_at.to_rfc3339(),
            ],
        )?;
        Ok(())
    }

    pub fn delete_all_sessions(&self) -> anyhow::Result<()> {
        self.conn.execute("DELETE FROM sessions", [])?;
        Ok(())
    }

    // ── HTTP ──

    pub fn insert_http_request(&self, req: &HttpRequest) -> anyhow::Result<()> {
        self.conn.execute(
            "INSERT INTO http_requests (id, method, url, headers, body, timestamp) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![req.id.to_string(), req.method, req.url, req.headers, req.body, req.timestamp.to_rfc3339()],
        )?;
        Ok(())
    }

    pub fn update_http_request(&self, req: &HttpRequest) -> anyhow::Result<()> {
        self.conn.execute(
            "UPDATE http_requests SET method=?2, url=?3, headers=?4, body=?5, timestamp=?6 WHERE id=?1",
            params![req.id.to_string(), req.method, req.url, req.headers, req.body, req.timestamp.to_rfc3339()],
        )?;
        Ok(())
    }

    pub fn get_http_request(&self, id: &Uuid) -> anyhow::Result<Option<HttpRequest>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, method, url, headers, body, timestamp FROM http_requests WHERE id=?1",
        )?;
        match stmt.query_row(params![id.to_string()], row_to_http_request) {
            Ok(r) => Ok(Some(r)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    pub fn delete_http_request(&self, id: &Uuid) -> anyhow::Result<()> {
        self.conn.execute(
            "DELETE FROM http_requests WHERE id=?1",
            params![id.to_string()],
        )?;
        Ok(())
    }

    pub fn insert_http_response(&self, resp: &HttpResponse) -> anyhow::Result<()> {
        self.conn.execute(
            "INSERT INTO http_responses (id, status_code, headers, body, timestamp) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![resp.id.to_string(), resp.status_code, resp.headers, resp.body, resp.timestamp.to_rfc3339()],
        )?;
        Ok(())
    }

    pub fn update_http_response(&self, resp: &HttpResponse) -> anyhow::Result<()> {
        self.conn.execute(
            "UPDATE http_responses SET status_code=?2, headers=?3, body=?4, timestamp=?5 WHERE id=?1",
            params![resp.id.to_string(), resp.status_code, resp.headers, resp.body, resp.timestamp.to_rfc3339()],
        )?;
        Ok(())
    }

    pub fn get_http_response(&self, id: &Uuid) -> anyhow::Result<Option<HttpResponse>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, status_code, headers, body, timestamp FROM http_responses WHERE id=?1",
        )?;
        match stmt.query_row(params![id.to_string()], row_to_http_response) {
            Ok(r) => Ok(Some(r)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    pub fn delete_http_response(&self, id: &Uuid) -> anyhow::Result<()> {
        self.conn.execute(
            "DELETE FROM http_responses WHERE id=?1",
            params![id.to_string()],
        )?;
        Ok(())
    }

    pub fn insert_http_transaction(&self, txn: &HttpTransaction) -> anyhow::Result<()> {
        self.conn.execute(
            "INSERT INTO http_transactions (id, task_id, request_id, response_id, duration_ms, created_at) VALUES (?1,?2,?3,?4,?5,?6)",
            params![txn.id.to_string(), txn.task_id.to_string(), txn.request_id.to_string(), txn.response_id.to_string(), txn.duration_ms, txn.created_at.to_rfc3339()],
        )?;
        Ok(())
    }

    pub fn list_transactions(&self, task_id: &Uuid) -> anyhow::Result<Vec<HttpTransaction>> {
        let mut stmt = self.conn.prepare("SELECT id, task_id, request_id, response_id, duration_ms, created_at FROM http_transactions WHERE task_id=?1 ORDER BY created_at DESC")?;
        let rows = stmt.query_map(params![task_id.to_string()], row_to_http_transaction)?;
        let mut items = Vec::new();
        for row in rows {
            items.push(row?);
        }
        Ok(items)
    }

    pub fn delete_http_transaction(&self, id: &Uuid) -> anyhow::Result<()> {
        self.conn.execute(
            "DELETE FROM http_transactions WHERE id=?1",
            params![id.to_string()],
        )?;
        Ok(())
    }

    // ── history ──

    pub fn insert_history(&self, entry: &HistoryEntry) -> anyhow::Result<()> {
        self.conn.execute(
            "INSERT INTO history_entries (id,transaction_id,timestamp,method,scheme,host,port,path,query,url,protocol,status_code,request_size,response_size,duration_ms,tls_enabled,source,tags,request_body,response_body,request_headers,response_headers) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12,?13,?14,?15,?16,?17,?18,?19,?20,?21,?22)",
            params![
                entry.id.to_string(), entry.transaction_id.map(|u| u.to_string()), entry.timestamp.to_rfc3339(),
                entry.method, entry.scheme, entry.host, entry.port, entry.path, entry.query, entry.url,
                entry.protocol, entry.status_code, entry.request_size, entry.response_size, entry.duration_ms,
                entry.tls_enabled as i32, entry.source, entry.tags, entry.request_body, entry.response_body,
                entry.request_headers, entry.response_headers,
            ],
        )?;
        Ok(())
    }

    pub fn get_history(&self, id: &Uuid) -> anyhow::Result<Option<HistoryEntry>> {
        let mut stmt = self
            .conn
            .prepare("SELECT * FROM history_entries WHERE id=?1")?;
        match stmt.query_row(params![id.to_string()], row_to_history) {
            Ok(r) => Ok(Some(r)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    pub fn list_history(&self, limit: usize) -> anyhow::Result<Vec<HistoryEntry>> {
        let mut stmt = self
            .conn
            .prepare("SELECT * FROM history_entries ORDER BY timestamp DESC LIMIT ?1")?;
        let rows = stmt.query_map(params![limit as i64], row_to_history)?;
        let mut items = Vec::new();
        for row in rows {
            items.push(row?);
        }
        Ok(items)
    }

    pub fn delete_history(&self, id: &Uuid) -> anyhow::Result<()> {
        self.conn.execute(
            "DELETE FROM history_entries WHERE id=?1",
            params![id.to_string()],
        )?;
        Ok(())
    }

    pub fn clear_history(&self) -> anyhow::Result<()> {
        self.conn.execute("DELETE FROM history_entries", [])?;
        Ok(())
    }
}

fn row_to_assessment(row: &rusqlite::Row) -> rusqlite::Result<Assessment> {
    let id_str: String = row.get("id")?;
    let env_str: String = row.get("environment")?;
    let scope_str: String = row.get("scope")?;
    let method_str: String = row.get("methodology")?;
    let status_str: String = row.get("status")?;
    let created_str: String = row.get("created_at")?;
    let updated_str: String = row.get("updated_at")?;

    Ok(Assessment {
        id: uuid::Uuid::parse_str(&id_str).map_err(conv_err)?,
        name: row.get("name")?,
        target: row.get("target")?,
        environment: Environment::from_label(&env_str).ok_or_else(|| {
            rusqlite::Error::InvalidParameterName(format!("unknown environment: {env_str}"))
        })?,
        scope: Scope::from_label(&scope_str).ok_or_else(|| {
            rusqlite::Error::InvalidParameterName(format!("unknown scope: {scope_str}"))
        })?,
        methodology: Methodology::from_label(&method_str).ok_or_else(|| {
            rusqlite::Error::InvalidParameterName(format!("unknown methodology: {method_str}"))
        })?,
        status: AssessmentStatus::from_str(&status_str)
            .map_err(rusqlite::Error::InvalidParameterName)?,
        created_at: chrono::DateTime::parse_from_rfc3339(&created_str)
            .map_err(conv_err)?
            .with_timezone(&chrono::Utc),
        updated_at: chrono::DateTime::parse_from_rfc3339(&updated_str)
            .map_err(conv_err)?
            .with_timezone(&chrono::Utc),
    })
}

fn row_to_finding(row: &rusqlite::Row) -> rusqlite::Result<Finding> {
    let id_str: String = row.get("id")?;
    let aid_str: String = row.get("assessment_id")?;
    let tid_str: String = row.get("task_id")?;
    let sev_str: String = row.get("severity")?;
    let conf_str: String = row.get("confidence")?;
    let status_str: String = row.get("status")?;
    let created_str: String = row.get("created_at")?;
    let updated_str: String = row.get("updated_at")?;
    Ok(Finding {
        id: Uuid::parse_str(&id_str).map_err(conv_err)?,
        assessment_id: Uuid::parse_str(&aid_str).map_err(conv_err)?,
        task_id: Uuid::parse_str(&tid_str).map_err(conv_err)?,
        title: row.get("title")?,
        description: row.get("description")?,
        severity: Severity::from_label(&sev_str).ok_or_else(|| {
            rusqlite::Error::InvalidParameterName(format!("unknown severity: {sev_str}"))
        })?,
        confidence: Confidence::from_label(&conf_str).ok_or_else(|| {
            rusqlite::Error::InvalidParameterName(format!("unknown confidence: {conf_str}"))
        })?,
        status: FindingStatus::from_label_for_db(&status_str).ok_or_else(|| {
            rusqlite::Error::InvalidParameterName(format!("unknown status: {status_str}"))
        })?,
        impact: row.get("impact")?,
        recommendation: row.get("recommendation")?,
        references: row.get("references")?,
        created_at: chrono::DateTime::parse_from_rfc3339(&created_str)
            .map_err(conv_err)?
            .with_timezone(&chrono::Utc),
        updated_at: chrono::DateTime::parse_from_rfc3339(&updated_str)
            .map_err(conv_err)?
            .with_timezone(&chrono::Utc),
    })
}

fn conv_err(e: impl std::error::Error + Send + Sync + 'static) -> rusqlite::Error {
    rusqlite::Error::FromSqlConversionFailure(0, rusqlite::types::Type::Text, Box::new(e))
}

fn row_to_evidence(row: &rusqlite::Row) -> rusqlite::Result<Evidence> {
    let id_str: String = row.get("id")?;
    let aid_str: String = row.get("assessment_id")?;
    let tid_str: String = row.get("task_id")?;
    let type_str: String = row.get("evidence_type")?;
    let created_str: String = row.get("created_at")?;
    let updated_str: String = row.get("updated_at")?;

    Ok(Evidence {
        id: Uuid::parse_str(&id_str).map_err(conv_err)?,
        assessment_id: Uuid::parse_str(&aid_str).map_err(conv_err)?,
        task_id: Uuid::parse_str(&tid_str).map_err(conv_err)?,
        evidence_type: EvidenceType::from_label(&type_str).ok_or_else(|| {
            rusqlite::Error::InvalidParameterName(format!("unknown evidence type: {type_str}"))
        })?,
        title: row.get("title")?,
        content: row.get("content")?,
        created_at: chrono::DateTime::parse_from_rfc3339(&created_str)
            .map_err(conv_err)?
            .with_timezone(&chrono::Utc),
        updated_at: chrono::DateTime::parse_from_rfc3339(&updated_str)
            .map_err(conv_err)?
            .with_timezone(&chrono::Utc),
    })
}

fn row_to_http_request(row: &rusqlite::Row) -> rusqlite::Result<HttpRequest> {
    let id_str: String = row.get("id")?;
    let ts: String = row.get("timestamp")?;
    Ok(HttpRequest {
        id: Uuid::parse_str(&id_str).map_err(conv_err)?,
        method: row.get("method")?,
        url: row.get("url")?,
        headers: row.get("headers")?,
        body: row.get("body")?,
        timestamp: chrono::DateTime::parse_from_rfc3339(&ts)
            .map_err(conv_err)?
            .with_timezone(&chrono::Utc),
    })
}

fn row_to_http_response(row: &rusqlite::Row) -> rusqlite::Result<HttpResponse> {
    let id_str: String = row.get("id")?;
    let ts: String = row.get("timestamp")?;
    Ok(HttpResponse {
        id: Uuid::parse_str(&id_str).map_err(conv_err)?,
        status_code: row.get("status_code")?,
        headers: row.get("headers")?,
        body: row.get("body")?,
        timestamp: chrono::DateTime::parse_from_rfc3339(&ts)
            .map_err(conv_err)?
            .with_timezone(&chrono::Utc),
    })
}

fn row_to_http_transaction(row: &rusqlite::Row) -> rusqlite::Result<HttpTransaction> {
    let id_str: String = row.get("id")?;
    let tid_str: String = row.get("task_id")?;
    let rid_str: String = row.get("request_id")?;
    let rsid_str: String = row.get("response_id")?;
    let ca_str: String = row.get("created_at")?;
    Ok(HttpTransaction {
        id: Uuid::parse_str(&id_str).map_err(conv_err)?,
        task_id: Uuid::parse_str(&tid_str).map_err(conv_err)?,
        request_id: Uuid::parse_str(&rid_str).map_err(conv_err)?,
        response_id: Uuid::parse_str(&rsid_str).map_err(conv_err)?,
        duration_ms: row.get("duration_ms")?,
        created_at: chrono::DateTime::parse_from_rfc3339(&ca_str)
            .map_err(conv_err)?
            .with_timezone(&chrono::Utc),
    })
}

fn row_to_history(row: &rusqlite::Row) -> rusqlite::Result<HistoryEntry> {
    let id_str: String = row.get("id")?;
    let ts: String = row.get("timestamp")?;
    let tid: Option<String> = row.get("transaction_id")?;
    Ok(HistoryEntry {
        id: Uuid::parse_str(&id_str).map_err(conv_err)?,
        transaction_id: tid.and_then(|s| Uuid::parse_str(&s).ok()),
        timestamp: chrono::DateTime::parse_from_rfc3339(&ts)
            .map_err(conv_err)?
            .with_timezone(&chrono::Utc),
        method: row.get("method")?,
        scheme: row.get("scheme")?,
        host: row.get("host")?,
        port: row.get("port")?,
        path: row.get("path")?,
        query: row.get("query")?,
        url: row.get("url")?,
        protocol: row.get("protocol")?,
        status_code: row.get("status_code")?,
        request_size: row.get::<_, i64>("request_size")? as u64,
        response_size: row.get::<_, i64>("response_size")? as u64,
        duration_ms: row.get::<_, i64>("duration_ms")? as u64,
        tls_enabled: row.get::<_, i32>("tls_enabled")? != 0,
        source: row.get("source")?,
        tags: row.get("tags")?,
        request_body: row.get("request_body")?,
        response_body: row.get("response_body")?,
        request_headers: row.get("request_headers")?,
        response_headers: row.get("response_headers")?,
    })
}
