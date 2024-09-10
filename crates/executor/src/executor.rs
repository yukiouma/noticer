mod waterbot;

trait Executor {
    fn execute(&self) -> anyhow::Result<()>;
}