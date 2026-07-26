#![allow(dead_code)]

use std::collections::VecDeque;

use uuid::Uuid;

use crate::intercept::errors::InterceptError;
use crate::intercept::models::InterceptedItem;

pub struct InterceptQueue {
    items: VecDeque<InterceptedItem>,
    capacity: usize,
}

impl InterceptQueue {
    pub fn new(capacity: usize) -> Self {
        Self {
            items: VecDeque::new(),
            capacity,
        }
    }

    pub fn push(&mut self, item: InterceptedItem) -> Result<(), InterceptError> {
        if self.items.len() >= self.capacity {
            return Err(InterceptError::QueueFull("Max capacity reached".into()));
        }
        self.items.push_back(item);
        Ok(())
    }

    pub fn pop(&mut self) -> Option<InterceptedItem> {
        self.items.pop_front()
    }

    pub fn remove(&mut self, id: Uuid) -> Option<InterceptedItem> {
        if let Some(pos) = self.items.iter().position(|i| i.id() == id) {
            self.items.remove(pos)
        } else {
            None
        }
    }

    pub fn find(&self, id: Uuid) -> Option<&InterceptedItem> {
        self.items.iter().find(|i| i.id() == id)
    }

    pub fn find_mut(&mut self, id: Uuid) -> Option<&mut InterceptedItem> {
        self.items.iter_mut().find(|i| i.id() == id)
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
    pub fn items(&self) -> &VecDeque<InterceptedItem> {
        &self.items
    }

    pub fn request_count(&self) -> usize {
        self.items
            .iter()
            .filter(|i| matches!(i.direction(), super::models::InterceptDirection::Request))
            .count()
    }
    pub fn response_count(&self) -> usize {
        self.items
            .iter()
            .filter(|i| matches!(i.direction(), super::models::InterceptDirection::Response))
            .count()
    }

    pub fn pending_requests(&self) -> impl Iterator<Item = &InterceptedItem> {
        self.items.iter().filter(|i| {
            matches!(i.direction(), super::models::InterceptDirection::Request)
                && matches!(i.status(), super::models::InterceptStatus::Waiting)
        })
    }

    pub fn pending_responses(&self) -> impl Iterator<Item = &InterceptedItem> {
        self.items.iter().filter(|i| {
            matches!(i.direction(), super::models::InterceptDirection::Response)
                && matches!(i.status(), super::models::InterceptStatus::Waiting)
        })
    }
}
