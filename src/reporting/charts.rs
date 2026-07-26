#![allow(dead_code)]

use crate::reporting::models::SeverityCounts;

pub fn severity_chart(counts: &SeverityCounts) -> String {
    let total = (counts.critical + counts.high + counts.medium + counts.low + counts.informational)
        .max(1) as f64;
    let mut s = String::from("Critical | ");
    s.push_str(&"#".repeat((counts.critical as f64 / total * 30.0) as usize));
    s.push_str(&format!(" {}\n", counts.critical));
    s.push_str(&format!(
        "High     | {} {}\n",
        "#".repeat((counts.high as f64 / total * 30.0) as usize),
        counts.high
    ));
    s.push_str(&format!(
        "Medium   | {} {}\n",
        "#".repeat((counts.medium as f64 / total * 30.0) as usize),
        counts.medium
    ));
    s.push_str(&format!(
        "Low      | {} {}\n",
        "#".repeat((counts.low as f64 / total * 30.0) as usize),
        counts.low
    ));
    s.push_str(&format!(
        "Info     | {} {}\n",
        "#".repeat((counts.informational as f64 / total * 30.0) as usize),
        counts.informational
    ));
    s
}
