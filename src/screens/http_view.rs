use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::models::http::{HttpRequest, HttpResponse, HttpTransaction};

#[derive(Debug, Clone)]
pub struct HttpPanelState {
    #[allow(dead_code)]
    pub task_id: uuid::Uuid,
    pub transactions: Vec<HttpTransaction>,
    pub requests: Vec<HttpRequest>,
    pub responses: Vec<HttpResponse>,
    pub selected_index: usize,
    pub scroll_offset: usize,
    pub mode: HttpMode,
    pub form: HttpFormState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub enum HttpMode {
    List,
    ViewTransaction(usize),
    CreateRequest,
    CreateResponse,
    CreateTransaction,
}

#[derive(Debug, Clone)]
pub struct HttpFormState {
    pub method: crate::models::http::HttpMethod,
    pub url: String,
    pub url_cursor: usize,
    pub status_code: i32,
    pub headers: String,
    pub headers_cursor: usize,
    pub body: String,
    pub body_cursor: usize,
    pub focused: HttpFormField,
    #[allow(dead_code)]
    pub duration_ms: i64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub enum HttpFormField {
    Method,
    Url,
    StatusCode,
    Headers,
    Body,
    Duration,
}

#[allow(dead_code)]
impl HttpFormField {
    pub fn req_fields() -> &'static [HttpFormField] {
        &[Self::Method, Self::Url, Self::Headers, Self::Body]
    }
    pub fn resp_fields() -> &'static [HttpFormField] {
        &[Self::StatusCode, Self::Headers, Self::Body]
    }
    pub fn next(self, fields: &[HttpFormField]) -> Self {
        let p = fields.iter().position(|f| *f == self).unwrap_or(0);
        fields[(p + 1) % fields.len()]
    }
    pub fn prev(self, fields: &[HttpFormField]) -> Self {
        let p = fields.iter().position(|f| *f == self).unwrap_or(0);
        fields[(p + fields.len() - 1) % fields.len()]
    }
}

impl Default for HttpPanelState {
    fn default() -> Self {
        Self {
            task_id: uuid::Uuid::nil(),
            transactions: vec![],
            requests: vec![],
            responses: vec![],
            selected_index: 0,
            scroll_offset: 0,
            mode: HttpMode::List,
            form: HttpFormState::default(),
        }
    }
}
impl Default for HttpFormState {
    fn default() -> Self {
        Self {
            method: crate::models::http::HttpMethod::Get,
            url: String::new(),
            url_cursor: 0,
            status_code: 200,
            headers: String::new(),
            headers_cursor: 0,
            body: String::new(),
            body_cursor: 0,
            focused: HttpFormField::Method,
            duration_ms: 0,
        }
    }
}

pub fn render(f: &mut Frame, area: Rect, state: &mut HttpPanelState) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title(" HTTP ")
        .title_alignment(Alignment::Center);
    let inner = block.inner(area);
    f.render_widget(block, area);

    match state.mode {
        HttpMode::List => render_list(f, inner, state),
        HttpMode::ViewTransaction(idx) => render_transaction_view(f, inner, state, idx),
        HttpMode::CreateRequest => render_request_form(f, inner, state),
        HttpMode::CreateResponse => render_response_form(f, inner, state),
        HttpMode::CreateTransaction => render_transaction_form(f, inner, state),
    }
}

fn render_list(f: &mut Frame, area: Rect, state: &mut HttpPanelState) {
    if state.transactions.is_empty() {
        let msg = Paragraph::new(
            "No HTTP transactions.\n\n[A] Add Transaction  [R] Add Request  [S] Add Response",
        )
        .style(Style::default().fg(Color::Gray))
        .alignment(Alignment::Center);
        f.render_widget(msg, area);
        return;
    }
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(1)])
        .split(area);
    for (i, txn) in state
        .transactions
        .iter()
        .skip(state.scroll_offset)
        .take(layout[0].height as usize)
        .enumerate()
    {
        let y = layout[0].y + i as u16;
        let sel = state.scroll_offset + i == state.selected_index;
        let base = if sel {
            Style::default().fg(Color::Black).bg(Color::Cyan)
        } else {
            Style::default()
        };
        let m = if sel { ">" } else { " " };
        let req = state.requests.iter().find(|r| r.id == txn.request_id);
        let resp = state.responses.iter().find(|r| r.id == txn.response_id);
        let label = match (req, resp) {
            (Some(rq), Some(rs)) => format!(
                "{} {} {} -> {} {} ({}ms)",
                rq.method,
                truncate(&rq.url, 40),
                m,
                rs.status_code,
                status_text(rs.status_code),
                txn.duration_ms
            ),
            _ => format!("{} Transaction", m),
        };
        f.render_widget(
            Paragraph::new(Line::from(Span::styled(label, base))),
            Rect {
                x: layout[0].x + 1,
                y,
                width: layout[0].width - 2,
                height: 1,
            },
        );
    }
    let hint = Paragraph::new(" A:Add  R:Request  S:Response  Enter:View  D:Delete  Left:Back ")
        .style(Style::default().fg(Color::Gray));
    f.render_widget(hint, layout[1]);
}

fn render_transaction_view(f: &mut Frame, area: Rect, state: &HttpPanelState, idx: usize) {
    if let Some(txn) = state.transactions.get(idx) {
        let req = state.requests.iter().find(|r| r.id == txn.request_id);
        let resp = state.responses.iter().find(|r| r.id == txn.response_id);
        let req_text = req.map(|r| r.format()).unwrap_or_default();
        let resp_text = resp.map(|r| r.format()).unwrap_or_default();
        let full = format!(
            "REQUEST:\n{}\n\nRESPONSE ({}ms):\n{}",
            req_text, txn.duration_ms, resp_text
        );
        f.render_widget(Paragraph::new(full).scroll((0, 0)), area);
    }
}

fn render_request_form(f: &mut Frame, area: Rect, state: &mut HttpPanelState) {
    let fld = HttpFormField::req_fields();
    render_http_form(f, area, &mut state.form, fld, "Create HTTP Request");
}

fn render_response_form(f: &mut Frame, area: Rect, state: &mut HttpPanelState) {
    let fld = HttpFormField::resp_fields();
    render_http_form(f, area, &mut state.form, fld, "Create HTTP Response");
}

fn render_transaction_form(f: &mut Frame, area: Rect, state: &mut HttpPanelState) {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Min(1),
        ])
        .split(area);
    f.render_widget(
        Block::default()
            .borders(Borders::ALL)
            .title(" Create Transaction "),
        area,
    );
    let req_url = state
        .requests
        .get(state.selected_index)
        .map(|r| r.url.clone())
        .unwrap_or_else(|| "none".into());
    let resp_label = state
        .responses
        .get(state.selected_index)
        .map(|r| format!("{} {}", r.status_code, status_text(r.status_code)))
        .unwrap_or_else(|| "none".into());
    let r_label = format!("Request: {}", req_url);
    let s_label = format!("Response: {}", resp_label);
    f.render_widget(Paragraph::new(r_label), layout[0]);
    f.render_widget(Paragraph::new(s_label), layout[1]);
    f.render_widget(
        Paragraph::new(" Enter:Save  Esc:Cancel ").style(Style::default().fg(Color::Gray)),
        layout[2],
    );
}

fn render_http_form(
    f: &mut Frame,
    area: Rect,
    form: &mut HttpFormState,
    fields: &[HttpFormField],
    title: &str,
) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title(format!(" {} ", title));
    let inner = block.inner(area);
    f.render_widget(block, area);
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Length(1); fields.len() + 2])
        .split(inner);

    for (i, field) in fields.iter().enumerate() {
        let focused = form.focused == *field;
        let style = if focused {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default()
        };
        match field {
            HttpFormField::Method => {
                let l = Line::from(vec![
                    Span::styled("  Method:   ", Style::default().fg(Color::Gray)),
                    Span::styled(format!("< {} >", form.method), style),
                ]);
                f.render_widget(Paragraph::new(l), layout[i]);
            }
            HttpFormField::Url => {
                let lt = "  URL:      ".to_string();
                let l = Line::from(vec![
                    Span::styled(&lt, Style::default().fg(Color::Gray)),
                    Span::styled(&form.url, style),
                ]);
                f.render_widget(Paragraph::new(l), layout[i]);
                if focused {
                    f.set_cursor_position((
                        inner.x + lt.len() as u16 + form.url_cursor.min(form.url.len()) as u16,
                        layout[i].y,
                    ));
                }
            }
            HttpFormField::StatusCode => {
                let l = Line::from(vec![
                    Span::styled("  Status:   ", Style::default().fg(Color::Gray)),
                    Span::styled(
                        format!("< {} {} >", form.status_code, status_text(form.status_code)),
                        style,
                    ),
                ]);
                f.render_widget(Paragraph::new(l), layout[i]);
            }
            HttpFormField::Headers => {
                let lt = "  Headers:  ".to_string();
                let l = Line::from(vec![
                    Span::styled(&lt, Style::default().fg(Color::Gray)),
                    Span::styled(&form.headers, style),
                ]);
                f.render_widget(Paragraph::new(l), layout[i]);
                if focused {
                    f.set_cursor_position((
                        inner.x
                            + lt.len() as u16
                            + form.headers_cursor.min(form.headers.len()) as u16,
                        layout[i].y,
                    ));
                }
            }
            HttpFormField::Body => {
                let lt = "  Body:     ".to_string();
                let l = Line::from(vec![
                    Span::styled(&lt, Style::default().fg(Color::Gray)),
                    Span::styled(&form.body, style),
                ]);
                f.render_widget(Paragraph::new(l), layout[i]);
                if focused {
                    f.set_cursor_position((
                        inner.x + lt.len() as u16 + form.body_cursor.min(form.body.len()) as u16,
                        layout[i].y,
                    ));
                }
            }
            HttpFormField::Duration => {}
        }
    }
    let hint = Paragraph::new(" Tab:Next  Enter:Save  Esc:Cancel ")
        .style(Style::default().fg(Color::Gray));
    f.render_widget(hint, layout[fields.len() + 1]);
}

fn truncate(s: &str, max: usize) -> &str {
    if s.len() > max {
        &s[..max]
    } else {
        s
    }
}
pub fn status_text(code: i32) -> &'static str {
    match code {
        200 => "OK",
        201 => "Created",
        301 => "Moved",
        302 => "Found",
        400 => "Bad Request",
        401 => "Unauthorized",
        403 => "Forbidden",
        404 => "Not Found",
        500 => "Internal Error",
        502 => "Bad Gateway",
        503 => "Unavailable",
        _ => "",
    }
}
