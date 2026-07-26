#![allow(dead_code)]

use crate::history::models::HistoryEntry;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HistorySortField {
    Timestamp,
    Duration,
    RequestSize,
    ResponseSize,
    StatusCode,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortDirection {
    Asc,
    Desc,
}

pub fn sort(entries: &mut [HistoryEntry], field: HistorySortField, direction: SortDirection) {
    match field {
        HistorySortField::Timestamp => {
            entries.sort_by(|a, b| cmp(&a.timestamp, &b.timestamp, direction))
        }
        HistorySortField::Duration => entries.sort_by(|a, b| {
            a.duration_ms
                .cmp(&b.duration_ms)
                .then_with(|| a.timestamp.cmp(&b.timestamp))
        }),
        HistorySortField::RequestSize => entries.sort_by_key(|a| a.request_size),
        HistorySortField::ResponseSize => entries.sort_by_key(|a| a.response_size),
        HistorySortField::StatusCode => entries.sort_by_key(|a| a.status_code),
    };
    if direction == SortDirection::Desc {
        entries.reverse();
    }
}

fn cmp<T: Ord>(a: &T, b: &T, dir: SortDirection) -> std::cmp::Ordering {
    match dir {
        SortDirection::Asc => a.cmp(b),
        SortDirection::Desc => b.cmp(a),
    }
}
