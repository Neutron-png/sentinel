use rusqlite::Connection;
use std::path::Path;

pub mod repository;

pub fn initialize(path: &Path) -> anyhow::Result<Connection> {
    let conn = Connection::open(path)?;

    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS assessments (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            target TEXT NOT NULL,
            environment TEXT NOT NULL,
            scope TEXT NOT NULL,
            methodology TEXT NOT NULL,
            status TEXT NOT NULL,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS sections (
            id TEXT PRIMARY KEY,
            assessment_id TEXT NOT NULL,
            title TEXT NOT NULL,
            display_order INTEGER NOT NULL,
            FOREIGN KEY (assessment_id) REFERENCES assessments(id)
        );

        CREATE TABLE IF NOT EXISTS activities (
            id TEXT PRIMARY KEY,
            section_id TEXT NOT NULL,
            title TEXT NOT NULL,
            description TEXT NOT NULL DEFAULT '',
            display_order INTEGER NOT NULL,
            FOREIGN KEY (section_id) REFERENCES sections(id)
        );

        CREATE TABLE IF NOT EXISTS tasks (
            id TEXT PRIMARY KEY,
            activity_id TEXT NOT NULL,
            title TEXT NOT NULL,
            description TEXT NOT NULL DEFAULT '',
            status TEXT NOT NULL DEFAULT 'Not Started',
            notes TEXT NOT NULL DEFAULT '',
            reference TEXT NOT NULL DEFAULT '',
            display_order INTEGER NOT NULL,
            FOREIGN KEY (activity_id) REFERENCES activities(id)
        );

        CREATE TABLE IF NOT EXISTS evidence (
            id TEXT PRIMARY KEY,
            assessment_id TEXT NOT NULL,
            task_id TEXT NOT NULL,
            evidence_type TEXT NOT NULL,
            title TEXT NOT NULL DEFAULT '',
            content TEXT NOT NULL DEFAULT '',
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL,
            FOREIGN KEY (assessment_id) REFERENCES assessments(id),
            FOREIGN KEY (task_id) REFERENCES tasks(id)
        );

        CREATE TABLE IF NOT EXISTS findings (
            id TEXT PRIMARY KEY,
            assessment_id TEXT NOT NULL,
            task_id TEXT NOT NULL,
            title TEXT NOT NULL,
            description TEXT NOT NULL DEFAULT '',
            severity TEXT NOT NULL,
            confidence TEXT NOT NULL,
            status TEXT NOT NULL DEFAULT 'Draft',
            impact TEXT NOT NULL DEFAULT '',
            recommendation TEXT NOT NULL DEFAULT '',
            `references` TEXT NOT NULL DEFAULT '',
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL,
            FOREIGN KEY (assessment_id) REFERENCES assessments(id),
            FOREIGN KEY (task_id) REFERENCES tasks(id)
        );

        CREATE TABLE IF NOT EXISTS finding_evidence (
            finding_id TEXT NOT NULL,
            evidence_id TEXT NOT NULL,
            PRIMARY KEY (finding_id, evidence_id),
            FOREIGN KEY (finding_id) REFERENCES findings(id),
            FOREIGN KEY (evidence_id) REFERENCES evidence(id)
        );

        CREATE TABLE IF NOT EXISTS sessions (
            id TEXT PRIMARY KEY,
            assessment_id TEXT NOT NULL,
            current_screen TEXT NOT NULL DEFAULT 'Home',
            last_opened_at TEXT NOT NULL,
            FOREIGN KEY (assessment_id) REFERENCES assessments(id)
        );

        CREATE TABLE IF NOT EXISTS http_requests (
            id TEXT PRIMARY KEY,
            method TEXT NOT NULL,
            url TEXT NOT NULL,
            headers TEXT NOT NULL DEFAULT '',
            body TEXT NOT NULL DEFAULT '',
            timestamp TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS http_responses (
            id TEXT PRIMARY KEY,
            status_code INTEGER NOT NULL,
            headers TEXT NOT NULL DEFAULT '',
            body TEXT NOT NULL DEFAULT '',
            timestamp TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS http_transactions (
            id TEXT PRIMARY KEY,
            task_id TEXT NOT NULL,
            request_id TEXT NOT NULL,
            response_id TEXT NOT NULL,
            duration_ms INTEGER NOT NULL DEFAULT 0,
            created_at TEXT NOT NULL,
            FOREIGN KEY (task_id) REFERENCES tasks(id),
            FOREIGN KEY (request_id) REFERENCES http_requests(id),
            FOREIGN KEY (response_id) REFERENCES http_responses(id)
        );

        CREATE TABLE IF NOT EXISTS history_entries (
            id TEXT PRIMARY KEY,
            transaction_id TEXT,
            timestamp TEXT NOT NULL,
            method TEXT NOT NULL,
            scheme TEXT NOT NULL DEFAULT 'https',
            host TEXT NOT NULL DEFAULT '',
            port INTEGER NOT NULL DEFAULT 443,
            path TEXT NOT NULL DEFAULT '/',
            query TEXT NOT NULL DEFAULT '',
            url TEXT NOT NULL,
            protocol TEXT NOT NULL DEFAULT 'HTTP/1.1',
            status_code INTEGER NOT NULL DEFAULT 200,
            request_size INTEGER NOT NULL DEFAULT 0,
            response_size INTEGER NOT NULL DEFAULT 0,
            duration_ms INTEGER NOT NULL DEFAULT 0,
            tls_enabled INTEGER NOT NULL DEFAULT 1,
            source TEXT NOT NULL DEFAULT 'proxy',
            tags TEXT NOT NULL DEFAULT '',
            request_body TEXT NOT NULL DEFAULT '',
            response_body TEXT NOT NULL DEFAULT '',
            request_headers TEXT NOT NULL DEFAULT '',
            response_headers TEXT NOT NULL DEFAULT '',
            connection_id TEXT,
            stream_id INTEGER DEFAULT 0,
            frame_metadata_json TEXT,
            negotiated_alpn TEXT,
            protocol_version TEXT DEFAULT 'HTTP/1.1'
        );",
    )?;

    // Safe migration for existing databases - add new columns if they don't exist
    for col_stmt in &[
        "ALTER TABLE history_entries ADD COLUMN connection_id TEXT",
        "ALTER TABLE history_entries ADD COLUMN stream_id INTEGER DEFAULT 0",
        "ALTER TABLE history_entries ADD COLUMN frame_metadata_json TEXT",
        "ALTER TABLE history_entries ADD COLUMN negotiated_alpn TEXT",
        "ALTER TABLE history_entries ADD COLUMN protocol_version TEXT DEFAULT 'HTTP/1.1'",
        "ALTER TABLE tasks ADD COLUMN reference TEXT NOT NULL DEFAULT ''",
    ] {
        let _ = conn.execute_batch(col_stmt);
    }

    Ok(conn)
}
