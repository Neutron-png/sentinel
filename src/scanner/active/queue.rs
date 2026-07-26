#![allow(dead_code)]

use std::collections::VecDeque;
use uuid::Uuid;

use crate::scanner::active::models::{ScanTask, TaskState};

pub struct ScanQueue {
    queue: VecDeque<ScanTask>,
    max_size: usize,
}

impl ScanQueue {
    pub fn new(max_size: usize) -> Self {
        Self {
            queue: VecDeque::new(),
            max_size,
        }
    }

    pub fn enqueue(&mut self, mut task: ScanTask) -> bool {
        if self.queue.len() >= self.max_size {
            return false;
        }
        task.state = TaskState::Queued;
        self.queue.push_back(task);
        true
    }

    pub fn dequeue(&mut self) -> Option<ScanTask> {
        self.queue.pop_front()
    }
    pub fn remove(&mut self, task_id: Uuid) -> Option<ScanTask> {
        if let Some(pos) = self.queue.iter().position(|t| t.id == task_id) {
            self.queue.remove(pos)
        } else {
            None
        }
    }
    pub fn len(&self) -> usize {
        self.queue.len()
    }
    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }
    pub fn cancel_job(&mut self, job_id: Uuid) -> usize {
        let before = self.queue.len();
        self.queue.retain(|t| t.job_id != job_id);
        before - self.queue.len()
    }
}
