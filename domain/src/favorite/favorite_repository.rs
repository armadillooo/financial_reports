pub trait FavoriteRepository {
    fn save() -> anyhow::Result<()>;
    fn delete() -> anyhow::Result<()>;
}