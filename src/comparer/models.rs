#![allow(dead_code)]

use serde::Serialize;
use serde_json::Value;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum DiffOp {
    Equal,
    Insert,
    Delete,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct DiffLine {
    pub op: DiffOp,
    pub left_line: Option<usize>,
    pub right_line: Option<usize>,
    pub text: String,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct TextDiff {
    pub lines: Vec<DiffLine>,
    pub added: usize,
    pub removed: usize,
    pub identical: bool,
    pub coarse: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum HeaderChangeKind {
    Added,
    Removed,
    Changed,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct HeaderChange {
    pub name: String,
    pub kind: HeaderChangeKind,
    pub left: Option<String>,
    pub right: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct HeaderDiff {
    pub changes: Vec<HeaderChange>,
    pub identical: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum JsonChangeKind {
    Added,
    Removed,
    Changed,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct JsonChange {
    pub path: String,
    pub kind: JsonChangeKind,
    pub left: Option<Value>,
    pub right: Option<Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct JsonDiff {
    pub changes: Vec<JsonChange>,
    pub identical: bool,
}
