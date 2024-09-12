pub mod waterbot;

pub trait Executor {
    fn execute(&self) -> anyhow::Result<()>;
}
