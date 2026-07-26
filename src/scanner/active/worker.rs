#![allow(dead_code)]

use std::sync::Arc;
use tokio::sync::Semaphore;
use tokio::task::JoinHandle;

use crate::scanner::active::models::WorkerConfig;

pub struct WorkerPool {
    config: WorkerConfig,
    handles: Vec<JoinHandle<()>>,
    semaphore: Arc<Semaphore>,
    shutdown: Arc<tokio::sync::Notify>,
}

impl WorkerPool {
    pub fn new(config: WorkerConfig) -> Self {
        let semaphore = Arc::new(Semaphore::new(config.worker_count));
        Self {
            config,
            handles: Vec::new(),
            semaphore,
            shutdown: Arc::new(tokio::sync::Notify::new()),
        }
    }

    pub async fn execute_task<F>(&self, f: F) -> Result<(), ()>
    where
        F: std::future::Future<Output = ()> + Send + 'static,
    {
        let _permit = self.semaphore.acquire().await.map_err(|_| ())?;
        tokio::spawn(f);
        Ok(())
    }

    pub fn shutdown(&self) {
        self.shutdown.notify_waiters();
    }
    pub fn worker_count(&self) -> usize {
        self.config.worker_count
    }
}
