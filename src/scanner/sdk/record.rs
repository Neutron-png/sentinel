//! Verification record for a scanner result.
//!
//! The record holds only facts the engine actually observed: the triggering
//! request, the baseline response, the observed response and the technique's
//! own evidence. The nine Verification Gate questions are then *derived* from
//! those facts. A question the engine did not test is reported as `open`
//! ("not established"), never as passed. This keeps the product rule intact:
//! if Sentinel cannot establish something, it says so instead of guessing.

use serde::{Deserialize, Serialize};

use crate::network::models::{HttpBody, HttpRequest, HttpResponse};

use super::result::{RuleConfidence, RuleStatus};

/// Bodies are stored up to this size. Enough to show the delta, small enough
/// to keep the database and the UI fast.
pub const BODY_LIMIT: usize = 4096;

pub const GATE_QUESTIONS: [&str; 9] = [
    "What is the baseline?",
    "What exactly changed?",
    "Why does the change indicate a vulnerability?",
    "Can it be reproduced?",
    "Can it be independently verified?",
    "Is there a plausible benign explanation?",
    "What evidence supports the conclusion?",
    "What is the confidence?",
    "Can another tester reproduce it?",
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum GateState {
    Passed,
    Failed,
    Open,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GateAnswer {
    pub question: String,
    pub answer: String,
    pub state: GateState,
}

/// What the engine saw when a rule reported a result.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct VerificationRecord {
    pub technique: String,
    pub parameter: String,
    pub payload: String,
    /// "METHOD /path" of the triggering request.
    pub endpoint: String,
    /// Raw baseline response, when the pipeline captured one.
    pub baseline: Option<String>,
    /// Raw response to the mutated request.
    pub observed: String,
    /// The technique's evidence: the part of the response that differs.
    pub delta: String,
    /// Raw triggering request, ready to replay.
    pub reproduction: String,
    /// Other techniques that flagged the same parameter in the same scan.
    #[serde(default)]
    pub corroborated_by: Vec<String>,
}

impl VerificationRecord {
    pub fn capture(
        technique: &str,
        parameter: &str,
        payload: &str,
        request: &HttpRequest,
        baseline: Option<&HttpResponse>,
        observed: &HttpResponse,
        delta: &str,
    ) -> Self {
        Self {
            technique: technique.to_string(),
            parameter: parameter.to_string(),
            payload: payload.to_string(),
            endpoint: endpoint_of(request),
            baseline: baseline.map(raw_response),
            observed: raw_response(observed),
            delta: delta.to_string(),
            reproduction: raw_request(request),
            corroborated_by: Vec::new(),
        }
    }

    /// Answer the nine questions from the captured facts only.
    pub fn derive_gate(&self, confidence: RuleConfidence) -> Vec<GateAnswer> {
        let mut out = Vec::with_capacity(9);
        let mut push = |i: usize, answer: String, state: GateState| {
            out.push(GateAnswer { question: GATE_QUESTIONS[i].to_string(), answer, state });
        };

        // 1. Baseline
        match &self.baseline {
            Some(b) => push(0, format!("Captured before testing: {}.", first_line(b)), GateState::Passed),
            None => push(0, "No baseline response was captured, so nothing can be compared.".into(), GateState::Failed),
        }

        // 2. What changed
        let observed_line = first_line(&self.observed);
        if self.delta.trim().is_empty() {
            push(1, format!("Observed {observed_line}, but the technique recorded no specific difference."), GateState::Open);
        } else {
            push(1, format!("Observed {observed_line}. {}", self.delta.trim()), GateState::Passed);
        }

        // 3. Why it indicates a vulnerability: the technique's own detection logic.
        if self.technique.is_empty() {
            push(2, "No detection technique was recorded.".into(), GateState::Open);
        } else {
            let param = if self.parameter.is_empty() { String::new() } else { format!(" in parameter '{}'", self.parameter) };
            push(2, format!("The {} check matched{param} with payload {}.", self.technique, quote(&self.payload)), GateState::Passed);
        }

        // 4. Reproduced: the pipeline sends each payload once and does not replay.
        push(3, "Not replayed. The engine sent this payload once.".into(), GateState::Open);

        // 5. Independent verification: only a second technique on the same parameter counts.
        if self.corroborated_by.is_empty() {
            push(4, "No second technique confirmed this parameter.".into(), GateState::Open);
        } else {
            push(4, format!("Also flagged by: {}.", self.corroborated_by.join(", ")), GateState::Passed);
        }

        // 6. Benign explanation: no false-positive controls run in this pipeline yet.
        push(5, "Not tested. No false-positive controls ran for this result.".into(), GateState::Open);

        // 7. Evidence
        if self.baseline.is_some() {
            push(6, "Baseline response, observed response and the triggering request are stored.".into(), GateState::Passed);
        } else {
            push(6, "Observed response and the triggering request are stored. No baseline.".into(), GateState::Open);
        }

        // 8. Confidence, as the technique reported it.
        let strong = matches!(confidence, RuleConfidence::Confirmed | RuleConfidence::High);
        push(
            7,
            format!("{} as reported by the technique.", confidence.label()),
            if strong { GateState::Passed } else { GateState::Open },
        );

        // 9. Another tester: the exact request is attached.
        if self.reproduction.trim().is_empty() {
            push(8, "The triggering request was not captured.".into(), GateState::Failed);
        } else {
            push(8, "Yes. The exact request is attached.".into(), GateState::Passed);
        }

        out
    }

    /// The verification status the gate supports. Confirmed only when every
    /// question passed; otherwise the rule's own status stands.
    pub fn gate_status(gate: &[GateAnswer], rule_status: RuleStatus) -> RuleStatus {
        if !gate.is_empty() && gate.iter().all(|g| g.state == GateState::Passed) {
            RuleStatus::Confirmed
        } else if rule_status == RuleStatus::Confirmed {
            // A rule may not claim Confirmed past an unanswered gate.
            RuleStatus::Potential
        } else {
            rule_status
        }
    }
}

fn endpoint_of(req: &HttpRequest) -> String {
    let path = url::Url::parse(&req.url)
        .map(|u| match u.query() {
            Some(q) => format!("{}?{}", u.path(), q),
            None => u.path().to_string(),
        })
        .unwrap_or_else(|_| req.url.clone());
    format!("{} {}", req.method, path)
}

fn host_of(url: &str) -> String {
    url::Url::parse(url)
        .ok()
        .and_then(|u| {
            u.host_str().map(|h| match u.port() {
                Some(p) => format!("{h}:{p}"),
                None => h.to_string(),
            })
        })
        .unwrap_or_default()
}

fn body_text(body: &HttpBody) -> String {
    let text = match body {
        HttpBody::Text(s) => s.clone(),
        HttpBody::Bytes(b) => String::from_utf8_lossy(b).to_string(),
        HttpBody::Json(v) => v.to_string(),
        _ => String::new(),
    };
    if text.len() > BODY_LIMIT {
        let mut cut = BODY_LIMIT;
        while !text.is_char_boundary(cut) {
            cut -= 1;
        }
        format!("{}\n[truncated, {} bytes total]", &text[..cut], text.len())
    } else {
        text
    }
}

/// Raw HTTP/1.1 request text that can be pasted into Repeater.
pub fn raw_request(req: &HttpRequest) -> String {
    let (_, path) = endpoint_of(req).split_once(' ').map(|(m, p)| (m.to_string(), p.to_string())).unwrap_or_default();
    let mut out = format!("{} {} HTTP/1.1\r\nHost: {}", req.method, if path.is_empty() { "/" } else { &path }, host_of(&req.url));
    for h in &req.headers {
        if !h.name.eq_ignore_ascii_case("host") {
            out.push_str(&format!("\r\n{}: {}", h.name, h.value));
        }
    }
    let body = body_text(&req.body);
    out.push_str("\r\n\r\n");
    out.push_str(&body);
    out
}

/// Raw response text: status line, headers, blank line, body.
pub fn raw_response(resp: &HttpResponse) -> String {
    let proto = if resp.protocol.is_empty() { "HTTP/1.1" } else { resp.protocol.as_str() };
    let mut out = format!("{} {} {}", proto, resp.status_code, resp.status_text).trim_end().to_string();
    for h in &resp.headers {
        out.push_str(&format!("\r\n{}: {}", h.name, h.value));
    }
    out.push_str("\r\n\r\n");
    out.push_str(&body_text(&resp.body));
    out
}

fn first_line(raw: &str) -> String {
    raw.lines().next().unwrap_or("").trim().to_string()
}

fn quote(s: &str) -> String {
    if s.is_empty() { "(none)".into() } else { format!("'{s}'") }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn record(baseline: bool, corroborated: bool) -> VerificationRecord {
        VerificationRecord {
            technique: "error-based".into(),
            parameter: "id".into(),
            payload: "1'".into(),
            endpoint: "GET /api/orders?id=1'".into(),
            baseline: baseline.then(|| "HTTP/1.1 200 OK\r\n\r\n{}".to_string()),
            observed: "HTTP/1.1 500 Internal Server Error\r\n\r\nsyntax error".into(),
            delta: "new database error versus baseline".into(),
            reproduction: "GET /api/orders?id=1' HTTP/1.1\r\nHost: t.test\r\n\r\n".into(),
            corroborated_by: if corroborated { vec!["boolean-based".into()] } else { vec![] },
        }
    }

    #[test]
    fn gate_has_nine_answers_in_order() {
        let g = record(true, false).derive_gate(RuleConfidence::High);
        assert_eq!(g.len(), 9);
        for (i, a) in g.iter().enumerate() {
            assert_eq!(a.question, GATE_QUESTIONS[i]);
        }
    }

    #[test]
    fn untested_questions_are_never_passed() {
        let g = record(true, true).derive_gate(RuleConfidence::Confirmed);
        // Replay and benign-explanation controls are not run by the pipeline.
        assert_eq!(g[3].state, GateState::Open);
        assert_eq!(g[5].state, GateState::Open);
    }

    #[test]
    fn current_engine_can_never_reach_confirmed() {
        let g = record(true, true).derive_gate(RuleConfidence::Confirmed);
        assert_eq!(VerificationRecord::gate_status(&g, RuleStatus::Confirmed), RuleStatus::Potential);
        assert_eq!(VerificationRecord::gate_status(&g, RuleStatus::Potential), RuleStatus::Potential);
    }

    #[test]
    fn missing_baseline_fails_question_one() {
        let g = record(false, false).derive_gate(RuleConfidence::High);
        assert_eq!(g[0].state, GateState::Failed);
        assert_eq!(g[6].state, GateState::Open);
    }

    #[test]
    fn corroboration_passes_independent_verification() {
        assert_eq!(record(true, false).derive_gate(RuleConfidence::High)[4].state, GateState::Open);
        assert_eq!(record(true, true).derive_gate(RuleConfidence::High)[4].state, GateState::Passed);
    }

    #[test]
    fn low_confidence_is_not_passed() {
        assert_eq!(record(true, false).derive_gate(RuleConfidence::Low)[7].state, GateState::Open);
    }

    #[test]
    fn all_passed_gate_is_confirmed() {
        let all: Vec<GateAnswer> = GATE_QUESTIONS
            .iter()
            .map(|q| GateAnswer { question: q.to_string(), answer: String::new(), state: GateState::Passed })
            .collect();
        assert_eq!(VerificationRecord::gate_status(&all, RuleStatus::Potential), RuleStatus::Confirmed);
    }

    #[test]
    fn record_round_trips_as_json() {
        let r = record(true, true);
        let json = serde_json::to_string(&r).unwrap();
        let back: VerificationRecord = serde_json::from_str(&json).unwrap();
        assert_eq!(r, back);
    }

    #[test]
    fn long_bodies_are_truncated_on_a_char_boundary() {
        let body = "é".repeat(BODY_LIMIT);
        let out = body_text(&HttpBody::Text(body));
        assert!(out.contains("[truncated,"));
    }
}
