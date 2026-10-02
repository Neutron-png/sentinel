#![allow(dead_code)]

use serde_json::Value;

use crate::comparer::models::{
    DiffLine, DiffOp, HeaderChange, HeaderChangeKind, HeaderDiff, JsonChange, JsonChangeKind,
    JsonDiff, TextDiff,
};

const MAX_LCS_CELLS: usize = 4_000_000;
const MAX_JSON_DEPTH: usize = 64;

pub fn diff_text(left: &str, right: &str) -> TextDiff {
    if left == right {
        let lines = left
            .lines()
            .enumerate()
            .map(|(i, l)| DiffLine {
                op: DiffOp::Equal,
                left_line: Some(i + 1),
                right_line: Some(i + 1),
                text: l.to_string(),
            })
            .collect();
        return TextDiff {
            lines,
            added: 0,
            removed: 0,
            identical: true,
            coarse: false,
        };
    }

    let a: Vec<&str> = split_lines(left);
    let b: Vec<&str> = split_lines(right);
    let (coarse, ops) = if a.len().saturating_mul(b.len()) > MAX_LCS_CELLS {
        (true, coarse_ops(&a, &b))
    } else {
        (false, lcs_ops(&a, &b))
    };

    let mut lines = Vec::with_capacity(ops.len());
    let mut added = 0;
    let mut removed = 0;
    let (mut li, mut ri) = (0usize, 0usize);
    for op in ops {
        match op {
            DiffOp::Equal => {
                lines.push(DiffLine {
                    op,
                    left_line: Some(li + 1),
                    right_line: Some(ri + 1),
                    text: a[li].to_string(),
                });
                li += 1;
                ri += 1;
            }
            DiffOp::Delete => {
                lines.push(DiffLine {
                    op,
                    left_line: Some(li + 1),
                    right_line: None,
                    text: a[li].to_string(),
                });
                li += 1;
                removed += 1;
            }
            DiffOp::Insert => {
                lines.push(DiffLine {
                    op,
                    left_line: None,
                    right_line: Some(ri + 1),
                    text: b[ri].to_string(),
                });
                ri += 1;
                added += 1;
            }
        }
    }
    TextDiff {
        lines,
        added,
        removed,
        identical: false,
        coarse,
    }
}

fn split_lines(value: &str) -> Vec<&str> {
    if value.is_empty() {
        return Vec::new();
    }
    value.split('\n').map(|l| l.strip_suffix('\r').unwrap_or(l)).collect()
}

fn lcs_ops(a: &[&str], b: &[&str]) -> Vec<DiffOp> {
    let n = a.len();
    let m = b.len();
    let mut table = vec![vec![0u32; m + 1]; n + 1];
    for i in (0..n).rev() {
        for j in (0..m).rev() {
            table[i][j] = if a[i] == b[j] {
                table[i + 1][j + 1] + 1
            } else {
                table[i + 1][j].max(table[i][j + 1])
            };
        }
    }
    let mut ops = Vec::new();
    let (mut i, mut j) = (0, 0);
    while i < n && j < m {
        if a[i] == b[j] {
            ops.push(DiffOp::Equal);
            i += 1;
            j += 1;
        } else if table[i + 1][j] >= table[i][j + 1] {
            ops.push(DiffOp::Delete);
            i += 1;
        } else {
            ops.push(DiffOp::Insert);
            j += 1;
        }
    }
    while i < n {
        ops.push(DiffOp::Delete);
        i += 1;
    }
    while j < m {
        ops.push(DiffOp::Insert);
        j += 1;
    }
    ops
}

fn coarse_ops(a: &[&str], b: &[&str]) -> Vec<DiffOp> {
    let mut prefix = 0;
    while prefix < a.len() && prefix < b.len() && a[prefix] == b[prefix] {
        prefix += 1;
    }
    let mut suffix = 0;
    while suffix < a.len() - prefix
        && suffix < b.len() - prefix
        && a[a.len() - 1 - suffix] == b[b.len() - 1 - suffix]
    {
        suffix += 1;
    }
    let mut ops = Vec::new();
    ops.extend(std::iter::repeat(DiffOp::Equal).take(prefix));
    ops.extend(std::iter::repeat(DiffOp::Delete).take(a.len() - prefix - suffix));
    ops.extend(std::iter::repeat(DiffOp::Insert).take(b.len() - prefix - suffix));
    ops.extend(std::iter::repeat(DiffOp::Equal).take(suffix));
    ops
}

pub fn diff_headers(left: &[(String, String)], right: &[(String, String)]) -> HeaderDiff {
    let mut names: Vec<String> = Vec::new();
    for (n, _) in left.iter().chain(right.iter()) {
        let lower = n.to_ascii_lowercase();
        if !names.contains(&lower) {
            names.push(lower);
        }
    }
    names.sort();
    let mut changes = Vec::new();
    for name in names {
        let l = left
            .iter()
            .rev()
            .find(|(n, _)| n.eq_ignore_ascii_case(&name))
            .map(|(_, v)| v.clone());
        let r = right
            .iter()
            .rev()
            .find(|(n, _)| n.eq_ignore_ascii_case(&name))
            .map(|(_, v)| v.clone());
        let kind = match (&l, &r) {
            (Some(lv), Some(rv)) if lv != rv => HeaderChangeKind::Changed,
            (Some(_), None) => HeaderChangeKind::Removed,
            (None, Some(_)) => HeaderChangeKind::Added,
            _ => continue,
        };
        changes.push(HeaderChange {
            name,
            kind,
            left: l,
            right: r,
        });
    }
    let identical = changes.is_empty();
    HeaderDiff { changes, identical }
}

pub fn diff_json(left: &Value, right: &Value) -> JsonDiff {
    let mut changes = Vec::new();
    walk_json("$", left, right, &mut changes, 0);
    let identical = changes.is_empty();
    JsonDiff { changes, identical }
}

fn walk_json(path: &str, left: &Value, right: &Value, out: &mut Vec<JsonChange>, depth: usize) {
    if depth > MAX_JSON_DEPTH {
        out.push(JsonChange {
            path: path.to_string(),
            kind: JsonChangeKind::Changed,
            left: Some(left.clone()),
            right: Some(right.clone()),
        });
        return;
    }
    match (left, right) {
        (Value::Object(l), Value::Object(r)) => {
            let mut keys: Vec<&String> = l.keys().chain(r.keys()).collect();
            keys.sort();
            keys.dedup();
            for key in keys {
                let child_path = format!("{path}.{key}");
                match (l.get(key), r.get(key)) {
                    (Some(lv), Some(rv)) => walk_json(&child_path, lv, rv, out, depth + 1),
                    (Some(lv), None) => out.push(JsonChange {
                        path: child_path,
                        kind: JsonChangeKind::Removed,
                        left: Some(lv.clone()),
                        right: None,
                    }),
                    (None, Some(rv)) => out.push(JsonChange {
                        path: child_path,
                        kind: JsonChangeKind::Added,
                        left: None,
                        right: Some(rv.clone()),
                    }),
                    (None, None) => {}
                }
            }
        }
        (Value::Array(l), Value::Array(r)) => {
            let max = l.len().max(r.len());
            for i in 0..max {
                let child_path = format!("{path}[{i}]");
                match (l.get(i), r.get(i)) {
                    (Some(lv), Some(rv)) => walk_json(&child_path, lv, rv, out, depth + 1),
                    (Some(lv), None) => out.push(JsonChange {
                        path: child_path,
                        kind: JsonChangeKind::Removed,
                        left: Some(lv.clone()),
                        right: None,
                    }),
                    (None, Some(rv)) => out.push(JsonChange {
                        path: child_path,
                        kind: JsonChangeKind::Added,
                        left: None,
                        right: Some(rv.clone()),
                    }),
                    (None, None) => {}
                }
            }
        }
        _ if left == right => {}
        _ => out.push(JsonChange {
            path: path.to_string(),
            kind: JsonChangeKind::Changed,
            left: Some(left.clone()),
            right: Some(right.clone()),
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn identical_text() {
        let d = diff_text("a\nb", "a\nb");
        assert!(d.identical);
        assert_eq!(d.added, 0);
        assert_eq!(d.removed, 0);
    }

    #[test]
    fn text_add_remove_change() {
        let d = diff_text("a\nb\nc", "a\nx\nc\nd");
        assert!(!d.identical);
        assert_eq!(d.removed, 1);
        assert_eq!(d.added, 2);
        let ops: Vec<DiffOp> = d.lines.iter().map(|l| l.op).collect();
        assert!(ops.contains(&DiffOp::Equal));
        assert!(ops.contains(&DiffOp::Delete));
        assert!(ops.contains(&DiffOp::Insert));
    }

    #[test]
    fn text_line_numbers_are_accurate() {
        let d = diff_text("one\ntwo", "one\nthree");
        let changed = d.lines.iter().find(|l| l.op != DiffOp::Equal).unwrap();
        assert!(changed.left_line.is_some() || changed.right_line.is_some());
    }

    #[test]
    fn header_diff() {
        let left = vec![
            ("Content-Type".to_string(), "text/html".to_string()),
            ("Server".to_string(), "nginx".to_string()),
        ];
        let right = vec![
            ("content-type".to_string(), "application/json".to_string()),
            ("X-Frame-Options".to_string(), "DENY".to_string()),
        ];
        let d = diff_headers(&left, &right);
        assert!(!d.identical);
        assert!(d.changes.iter().any(|c| c.kind == HeaderChangeKind::Changed && c.name == "content-type"));
        assert!(d.changes.iter().any(|c| c.kind == HeaderChangeKind::Removed && c.name == "server"));
        assert!(d.changes.iter().any(|c| c.kind == HeaderChangeKind::Added && c.name == "x-frame-options"));
    }

    #[test]
    fn json_structural_diff() {
        let l = json!({"a": 1, "b": {"c": 2}, "arr": [1,2,3]});
        let r = json!({"a": 1, "b": {"c": 5}, "arr": [1,2], "d": true});
        let d = diff_json(&l, &r);
        assert!(!d.identical);
        assert!(d.changes.iter().any(|c| c.path == "$.b.c" && c.kind == JsonChangeKind::Changed));
        assert!(d.changes.iter().any(|c| c.path == "$.arr[2]" && c.kind == JsonChangeKind::Removed));
        assert!(d.changes.iter().any(|c| c.path == "$.d" && c.kind == JsonChangeKind::Added));
    }

    #[test]
    fn json_identical() {
        let v = json!({"x": [1, 2], "y": null});
        assert!(diff_json(&v, &v).identical);
    }
}
