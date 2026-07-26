#![allow(dead_code)]

#[derive(Debug, Clone)]
pub enum AutomationAction {
    Navigate {
        url: String,
    },
    Click {
        selector: String,
    },
    DoubleClick {
        selector: String,
    },
    RightClick {
        selector: String,
    },
    Hover {
        selector: String,
    },
    Focus {
        selector: String,
    },
    Blur {
        selector: String,
    },
    Scroll {
        x: i64,
        y: i64,
    },
    TypeText {
        selector: String,
        text: String,
    },
    Clear {
        selector: String,
    },
    AppendText {
        selector: String,
        text: String,
    },
    UploadFile {
        selector: String,
        path: String,
    },
    SelectOption {
        selector: String,
        value: String,
    },
    Check {
        selector: String,
    },
    Uncheck {
        selector: String,
    },
    Wait {
        ms: u64,
    },
    WaitForSelector {
        selector: String,
        timeout_ms: u64,
    },
    WaitForText {
        text: String,
        timeout_ms: u64,
    },
    WaitUntilVisible {
        selector: String,
        timeout_ms: u64,
    },
    WaitUntilHidden {
        selector: String,
        timeout_ms: u64,
    },
    FillForm {
        form_selector: String,
        fields: Vec<(String, String)>,
    },
    SubmitForm {
        selector: String,
    },
    GoBack,
    GoForward,
    Reload,
    Screenshot {
        full_page: bool,
    },
}

#[derive(Debug, Clone)]
pub struct Selector {
    pub kind: SelectorKind,
    pub value: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SelectorKind {
    Css,
    XPath,
    Id,
    Name,
    Class,
    Tag,
    Text,
    Placeholder,
    Label,
}

#[derive(Debug, Clone)]
pub struct AutomationResult {
    pub success: bool,
    pub error: Option<String>,
    pub duration_ms: u64,
}
