use super::Executor;
use std::collections::HashMap;

pub struct ExecutorManager {
    executors: HashMap<usize, Box<dyn Executor + Send + Sync>>,
}

impl ExecutorManager {
    pub fn new() -> ExecutorManager {
        ExecutorManager {
            executors: HashMap::new(),
        }
    }

    pub fn set_executor<T: Executor + 'static + Send + Sync>(&mut self, id: usize, executor: T) {
        self.executors.insert(id, Box::new(executor));
    }

    pub async fn execute(&self, id: usize) -> anyhow::Result<()> {
        if let Some(executor) = self.executors.get(&id) {
            executor.execute().await?;
        }
        Ok(())
    }
}
