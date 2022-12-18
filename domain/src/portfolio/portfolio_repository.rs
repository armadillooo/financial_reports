pub trait PortfolioReposotory {
    fn save() -> anyhow::Result<()>;
    fn delete() -> anyhow::Result<()>;
}