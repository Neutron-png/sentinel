#![allow(dead_code)]

use crate::graphql::models::GraphQlOperation;

pub struct GraphQlParser;

impl GraphQlParser {
    pub fn detect_operation(query: &str) -> Option<GraphQlOperation> {
        let q = query.trim().to_lowercase();
        if q.starts_with("mutation") {
            Some(GraphQlOperation::Mutation)
        } else if q.starts_with("subscription") {
            Some(GraphQlOperation::Subscription)
        } else if q.starts_with("{") || q.starts_with("query") {
            Some(GraphQlOperation::Query)
        } else {
            None
        }
    }

    pub fn extract_operation_name(query: &str) -> Option<String> {
        let q = query.trim();
        if q.starts_with("query") || q.starts_with("mutation") || q.starts_with("subscription") {
            let after = q.split_whitespace().nth(1)?;
            let name = after.split('(').next()?.split('{').next()?;
            if !name.is_empty() && name.chars().next()?.is_uppercase() {
                Some(name.to_string())
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn extract_arguments(query: &str, field: &str) -> Vec<(String, String)> {
        let mut args = Vec::new();
        if let Some(pos) = query.find(field) {
            let after = &query[pos + field.len()..];
            if let Some(paren_start) = after.find('(') {
                let paren_end = after[paren_start..]
                    .find(')')
                    .unwrap_or(after.len() - paren_start);
                let arg_str = &after[paren_start + 1..paren_start + paren_end];
                for part in arg_str.split(',') {
                    let kv: Vec<&str> = part.splitn(2, ':').collect();
                    if kv.len() == 2 {
                        args.push((kv[0].trim().to_string(), kv[1].trim().to_string()));
                    }
                }
            }
        }
        args
    }
}
